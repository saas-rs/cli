use crate::apiclient;
use crate::protocol::saas_rs::user::v1::{DownloadFileRequest, FindFileRequest, GenerateRequest};
use crate::util::git::is_dirty;
use git2::Repository;
use std::{fs::File, io::Write, process::Command};
use tempfile::TempDir;
use tonic::codegen::tokio_stream::StreamExt;

pub async fn do_generate(req: GenerateRequest) -> Result<(), Box<dyn std::error::Error>> {
    // Perform the generate request
    let mut client = apiclient::new_user_service_client().await?;
    let res = client.generate(req).await?.into_inner();

    let tempdir = TempDir::new()?;
    {
        // Download FileInfo for patch file to get filename
        let file_info = client
            .find_file(FindFileRequest {
                id: res.file_id.clone(),
            })
            .await?
            .into_inner()
            .file_info
            .unwrap();

        // Open a local temp file for output
        let mut file = File::options().create(true).truncate(true).write(true).open(format!(
            "{}/{}",
            tempdir.path().display(),
            file_info.filename
        ))?;

        // Download chunks, and append them to temp file
        let req = DownloadFileRequest { id: res.file_id };
        let mut input_stream = client.download_file(req).await?.into_inner();
        while let Some(res) = input_stream.next().await {
            match res {
                Ok(item) => {
                    file.write_all(&item.chunk)?;
                }
                Err(e) => {
                    eprintln!("Failed downloading file: {e:?}");
                    std::process::exit(1);
                }
            }
        }
        file.flush()?;

        eprintln!("Response received");
    }

    // Apply a patch, if it was received
    let patch_path = format!("{}/my.patch", tempdir.path().display());
    if std::fs::exists(&patch_path)? {
        if let Ok(metadata) = std::fs::metadata(&patch_path) {
            if metadata.len() == 0 {
                eprintln!("No changes to the workspace were necessary");
                return Ok(());
            }
        }
        let _output = Command::new("git").arg("apply").arg(patch_path).output()?;
        eprintln!("Patch applied to local workspace");
    }

    // If workspace is dirty now, suggest running make
    let repo = Repository::open(".")?;
    if is_dirty(&repo)? {
        eprintln!("Workspace is dirty; now would be a good time to run `make` and then commit");
    }

    Ok(())
}
