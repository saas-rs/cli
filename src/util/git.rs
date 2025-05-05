use crate::apiclient;
use crate::protocol::saas_rs::user::v1::{
    CreateProjectRequest, FindAccountRequest, FindManyProjectsRequest, Project, UpdateAccountRequest,
};
use git2::{Repository, RepositoryState, Status};
use pbjson_types::FieldMask;

pub async fn find_or_create_default_project_id() -> Result<String, Box<dyn std::error::Error>> {
    let mut client = apiclient::new_user_service_client().await?;
    let req = FindAccountRequest { id: "me".to_string() };
    let mut account = client.find_account(req).await?.into_inner().account.unwrap();

    let project_id = match account.default_project_id {
        Some(id) => id,
        None => {
            let req = FindManyProjectsRequest {
                filter: None,
                field_mask: Some(FieldMask {
                    paths: vec!["id".to_string()],
                }),
                limit: Some(1),
                offset: None,
            };
            let project = {
                // Find first project
                match client.find_many_projects(req).await?.into_inner().projects.first() {
                    Some(project) => project.clone(),
                    None => {
                        // Create project
                        let project = Project {
                            name: "Default".to_string(),
                            ..Default::default()
                        };
                        let req = CreateProjectRequest { project: Some(project) };
                        client.create_project(req).await?.into_inner().project.unwrap()
                    }
                }
            };

            // Update account to set it as the default
            account.default_project_id = Some(project.id.clone());
            let req = UpdateAccountRequest { account: Some(account) };
            client.update_account(req).await?;
            project.id
        }
    };
    Ok(project_id)
}

pub fn require_clean_repo(repo: &Repository) -> Result<(), Box<dyn std::error::Error>> {
    const MSG: &str = "Cannot perform work in a dirty git repo";
    if repo.state() != RepositoryState::Clean {
        return Err(MSG.into());
    }
    let statuses = repo.statuses(None)?;
    let is_dirty = !statuses
        .iter()
        .filter(|entry| entry.status() != Status::CURRENT && entry.status() != Status::IGNORED)
        .collect::<Vec<_>>()
        .is_empty();
    if is_dirty {
        return Err(MSG.into());
    }
    Ok(())
}

#[allow(unused)]
pub fn last_commit_hash(repo: &Repository) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let revspec = repo.revparse("HEAD")?;
    if let Some(from) = revspec.from() {
        return Ok(Some(from.id().to_string()));
    }
    Ok(None)
}
