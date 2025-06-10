use crate::protocol::saas_rs::user::v1::generate_request::{Archive, Snapshot};
use crate::protocol::saas_rs::user::v1::{upload_file_request, FileInfo, UploadFileRequest};
use crate::{apiclient, util};
use git2::Repository;
use log::debug;
use std::fs;
use std::process::Command;
use std::str::from_utf8;
use tempfile::NamedTempFile;
use tokio::sync::mpsc;
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;

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
    // let file_info = FileInfo {
    //     length: metadata.len() as u32,
    //     filename: "archive.zip".to_string(),
    //     ..Default::default()
    // };
    // let req1 = UploadFileRequest {
    //     r#type: Some(upload_file_request::Type::FileInfo(file_info)),
    // };
    // let contents = fs::read(file.path())?; // TODO(https://github.com/saas-rs/cli/issues/32)
    // let req2 = UploadFileRequest {
    //     r#type: Some(upload_file_request::Type::Chunk(contents)),
    // };
    // let outbound = async_stream::stream! {
    //     yield req1;
    //     yield req2;
    // };
    // Setup channels to communicate with a task
    let (tx, rx) = mpsc::channel(2);
    let outbound = ReceiverStream::new(rx);

    // Start task to feed the output stream
    tokio::spawn(async move {
        // The first message is just the file info
        let file_info = FileInfo {
            length: metadata.len() as u32,
            filename: "archive.zip".to_string(),
            ..Default::default()
        };
        let req = UploadFileRequest {
            r#type: Some(upload_file_request::Type::FileInfo(file_info)),
        };
        tx.send(req).await.unwrap();

        // The rest of the mssages are chunks of content
        let contents = fs::read(file.path()).unwrap(); // TODO(https://github.com/saas-rs/cli/issues/32)
        let req = UploadFileRequest {
            r#type: Some(upload_file_request::Type::Chunk(contents)),
        };
        tx.send(req).await.unwrap();

        // TODO
        // while let Some(item) = input_stream.next().await {
        //     match item {
        //         Ok(item) => {
        //             let res = StreamDownloadedFileResponse { chunk: item.chunk };
        //             if let Err(_e) = tx.send(Ok(res)).await {
        //                 break;
        //             }
        //         }
        //         _ => break,
        //     }
        // }

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

// pub(super) async fn download(
//     this: &UserGrpcServerV1,
//     req: Request<StreamDownloadedFileRequest>,
// ) -> Result<Response<DownloadedFileResponseStream>, Status> {
//     // Make upstream request to start stream
//     let mut store_client = StoreClient::new(this.store_client_channel.clone());
//     let stream_downloaded_file_req = proto::onprem::store::v1::StreamDownloadedFileRequest {
//         id: req.into_inner().id.clone(),
//     };
//     let mut input_stream = store_client
//         .stream_downloaded_file(stream_downloaded_file_req)
//         .await?
//         .into_inner();
//
//     // Setup channels to communicate with a task
//     let (tx, rx) = mpsc::channel(128);
//
//     // Start task to run the message pump
//     tokio::spawn(async move {
//         while let Some(item) = input_stream.next().await {
//             match item {
//                 Ok(item) => {
//                     let res = StreamDownloadedFileResponse { chunk: item.chunk };
//                     if let Err(_e) = tx.send(Ok(res)).await {
//                         break;
//                     }
//                 }
//                 _ => break,
//             }
//         }
//         debug!("Terminating");
//     });
//
//     // Streaming response
//     let output_stream = ReceiverStream::new(rx);
//     Ok(Response::new(Box::pin(output_stream) as DownloadedFileResponseStream))
// }
