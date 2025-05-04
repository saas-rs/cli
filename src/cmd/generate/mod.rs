pub(super) mod controller;
pub(super) mod model;
pub(super) mod resource;
pub(super) mod service;
pub(super) mod uuid_v4;
pub(super) mod xid;

use crate::protocol::saas_rs::user::v1::generate_request::{Archive, Snapshot};
use crate::protocol::saas_rs::user::v1::{
    upload_file_request, DownloadFileRequest, FileInfo, FindFileRequest, GenerateRequest, UploadFileRequest,
};
use crate::{apiclient, util};
use clap::Parser;
use git2::Repository;
use log::debug;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempdir::TempDir;
use tempfile::tempfile;
use tonic::codegen::tokio_stream::StreamExt;

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Parser)]
#[allow(clippy::enum_variant_names)]
pub enum Subcommand {
    #[command(name = "controller")]
    Controller(controller::Opts),

    #[command(name = "model")]
    Model(model::Opts),

    #[command(name = "resource")]
    Resource(resource::Opts),

    #[command(name = "service")]
    Service(service::Opts),

    #[command(name = "xid")]
    Xid(xid::Opts),

    #[command(name = "uuidv4")]
    UuidV4(xid::Opts),
}

pub async fn run(subcommand: Subcommand) -> Result<(), Box<dyn std::error::Error>> {
    match subcommand {
        Subcommand::Controller(controller::Opts {
            service,
            version,
            resource,
        }) => {
            controller::run(service, version, resource).await?;
        }
        Subcommand::Model(model::Opts {
            service,
            version,
            name,
            fields,
        }) => {
            model::run(service, version, name, fields).await?;
        }
        Subcommand::Resource(resource::Opts {
            service,
            version,
            name,
            fields,
        }) => {
            resource::run(service, version, name, fields).await?;
        }
        Subcommand::Service(service::Opts {
            name,
            resources,
            version,
            with_cli,
        }) => {
            service::run(name, resources, version, with_cli).await?;
        }
        Subcommand::Xid(xid::Opts { n }) => {
            xid::run(n).await?;
        }
        Subcommand::UuidV4(xid::Opts { n }) => {
            uuid_v4::run(n).await?;
        }
    }
    Ok(())
}

pub(super) async fn do_generate_preflight() -> Result<(String, Snapshot), Box<dyn std::error::Error>> {
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
    let output = Command::new("git")
        .arg("archive")
        .arg("--format")
        .arg("zip")
        .arg("HEAD")
        .output()?;
    debug!("Archival completed");

    // Upload the archive
    let mut file = tempfile()?;
    file.write_all(&output.stdout)?;
    let mut client = apiclient::new_user_service_client().await?;
    let file_info = FileInfo {
        length: output.stdout.len() as u32,
        filename: "archive.zip".to_string(),
        ..Default::default()
    };
    let req1 = UploadFileRequest {
        r#type: Some(upload_file_request::Type::FileInfo(file_info)),
    };
    let req2 = UploadFileRequest {
        r#type: Some(upload_file_request::Type::Chunk(output.stdout)),
    };
    let outbound = async_stream::stream! {
        yield req1;
        yield req2;
    };
    let file_info = client.upload_file(outbound).await?.into_inner().file_info.unwrap();
    debug!("Uploaded {file_info:?}");

    let snapshot = Snapshot::Archive(Archive {
        file_ids: vec![file_info.id],
    });
    Ok((project_id, snapshot))
}

pub(super) async fn do_generate(req: GenerateRequest) -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}
