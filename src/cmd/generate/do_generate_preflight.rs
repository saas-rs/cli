use crate::protocol::saas_rs::user::v1::generate_request::{Archive, Snapshot};
use crate::protocol::saas_rs::user::v1::{upload_file_request, FileInfo, UploadFileRequest};
use crate::{apiclient, util};
use git2::Repository;
use log::debug;
use std::fs;
use std::process::Command;
use std::str::from_utf8;
use tempfile::NamedTempFile;

pub async fn do_generate_preflight() -> Result<(String, Snapshot), Box<dyn std::error::Error>> {
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
    if !output.stderr.is_empty() {
        eprintln!("{}", from_utf8(&output.stderr)?);
    }
    debug!("Archival completed");

    // Upload the archive
    let metadata = fs::metadata(file.path())?;
    let file_info = FileInfo {
        length: metadata.len() as u32,
        filename: "archive.zip".to_string(),
        ..Default::default()
    };
    let req1 = UploadFileRequest {
        r#type: Some(upload_file_request::Type::FileInfo(file_info)),
    };
    let contents = fs::read(file.path())?; // TODO(https://github.com/saas-rs/cli/issues/32)
    let req2 = UploadFileRequest {
        r#type: Some(upload_file_request::Type::Chunk(contents)),
    };
    let outbound = async_stream::stream! {
        yield req1;
        yield req2;
    };
    let mut client = apiclient::new_user_service_client().await?;
    let file_info = client.upload_file(outbound).await?.into_inner().file_info.unwrap();
    debug!("Uploaded {file_info:?}");

    let snapshot = Snapshot::Archive(Archive {
        file_ids: vec![file_info.id],
    });
    Ok((project_id, snapshot))
}
