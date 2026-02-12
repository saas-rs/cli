use crate::protocol::saas_rs::user::v1::{
    generate_request::{Archive, Snapshot},
    {FileInfo, UploadFileRequest, upload_file_request},
};
use crate::{apiclient, util};
use git2::Repository;
use log::debug;
use std::{fs, process::Command, str::from_utf8};
use tempfile::NamedTempFile;
use tokio::{fs::File, sync::mpsc};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;

pub async fn do_generate_preflight(
    ignore_missing_head: bool,
) -> Result<(String, Snapshot), Box<dyn std::error::Error>> {
    // Don't run with a bare repository
    let repo = Repository::open(".")?;
    if repo.is_bare() {
        return Err("Cannot work with a bare repository".into());
    }

    // Don't run if the git workspace is dirty
    util::git::require_clean_repo(&repo)?;

    // Find the default project
    let project_id = util::git::find_or_create_default_project_id().await?;

    // Archive the workspace
    let file = NamedTempFile::new()?;
    let output = Command::new("git")
        .arg("archive")
        .arg("--format")
        .arg("zip")
        .arg("--output")
        .arg(file.path().display().to_string())
        .arg("HEAD")
        .output()?;
    if !output.status.success() {
        if !output.stderr.is_empty() {
            eprintln!("{}", from_utf8(&output.stderr)?);
        }
        if from_utf8(&output.stderr)?.ends_with("HEAD") && ignore_missing_head {
        } else {
            std::process::exit(1);
        }
    }
    debug!("Archival completed");

    // Prepare a FileInfo descriptor for the archive
    let metadata = fs::metadata(file.path())?;
    let file_info = FileInfo {
        length: metadata.len() as u32,
        filename: "archive.zip".to_string(),
        ..Default::default()
    };

    // Start task to feed an output stream with the FileInfo then the chunked contents
    let file = File::open(file.path()).await?;
    let mut file_reader_stream = FramedRead::new(file, BytesCodec::new());
    let (tx, rx) = mpsc::channel(2);
    let outbound = ReceiverStream::new(rx);
    tokio::spawn(async move {
        // The first message is just the file info
        let req = UploadFileRequest {
            r#type: Some(upload_file_request::Type::FileInfo(file_info)),
        };
        tx.send(req).await.unwrap();

        // The rest of the mssages are chunks of content
        while let Some(item) = file_reader_stream.next().await {
            match item {
                Ok(item) => {
                    let req = UploadFileRequest {
                        r#type: Some(upload_file_request::Type::Chunk(item.to_vec())),
                    };
                    if let Err(_e) = tx.send(req).await {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Failed uploading archive: {e:?}");
                    std::process::exit(1);
                }
            }
        }

        debug!("Terminating");
    });

    // Upload the workspace archive
    let mut client = apiclient::new_user_service_client().await?;
    let file_info = client.upload_file(outbound).await?.into_inner().file_info.unwrap();
    debug!("Uploaded {file_info:?}");

    // Respond
    let snapshot = Snapshot::Archive(Archive {
        file_ids: vec![file_info.id],
    });
    Ok((project_id, snapshot))
}
