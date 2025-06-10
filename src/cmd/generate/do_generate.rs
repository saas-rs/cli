use crate::apiclient;
use crate::protocol::saas_rs::user::v1::{DownloadFileRequest, FindFileRequest, GenerateRequest};
use crate::util::git::is_dirty;
use git2::Repository;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempdir::TempDir;
use tonic::codegen::tokio_stream::StreamExt;

pub async fn do_generate(req: GenerateRequest) -> Result<(), Box<dyn std::error::Error>> {
    // Perform the generate request
    let mut client = apiclient::new_user_service_client().await?;
    let res = client.generate(req).await?.into_inner();

    let tempdir = TempDir::new("saas-rs-cli")?;
    {
        // Download patch file
        let req = DownloadFileRequest {
            id: res.file_id.clone(),
        };
        let mut input_stream = client.download_file(req).await?.into_inner();
        let mut buf = vec![];
        while let Some(res) = input_stream.next().await {
            match res {
                Ok(item) => {
                    buf = [&buf[..], &item.chunk[..]].concat();
                }
                Err(e) => {
                    eprintln!("{e:?}");
                }
            }
        }

        // Write it to fs
        let file_info = client
            .find_file(FindFileRequest { id: res.file_id })
            .await?
            .into_inner()
            .file_info
            .unwrap();
        let mut file = File::options().create(true).truncate(true).write(true).open(format!(
            "{}/{}",
            tempdir.path().display(),
            file_info.filename
        ))?;
        file.write_all(&buf)?;

        eprintln!("Response received");
    }

    // Apply a patch, if it was received
    let patch_path = format!("{}/my.patch", tempdir.path().display());
    if std::fs::exists(&patch_path)? {
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
