use crate::apiclient;
use crate::cmd::list::projects;
use crate::protocol::saas_rs::user::v1::FindProjectRequest;
use clap::Parser;
use polars::prelude::*;
use std::io::Cursor;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Project ID
    #[arg(value_name = "ID")]
    pub(super) id: String,

    /// Output format
    #[arg(short = 'o', long = "output", default_value = "ps", value_parser = clap::builder::PossibleValuesParser::new(crate::OUTPUTS)
    )]
    pub(super) output: String,
}

pub async fn run(id: String, output: String) -> Result<(), Box<dyn std::error::Error>> {
    // Request record
    let mut client = apiclient::new_user_service_client().await?;
    let req = FindProjectRequest { id };
    let res = client.find_project(req).await?.into_inner();

    // Convert to dataframe in preparation for output
    let json = serde_json::to_vec(&res.project)?;
    let file = Cursor::new(json);
    let df = JsonReader::new(file).finish()?;

    // Apply type-specific narrow formatting
    let mut df = match output.as_str() {
        "ps" => {
            let cols: Vec<_> = projects::PS_COLUMNS
                .iter()
                .filter(|col| df.column(col).is_ok())
                .cloned()
                .collect();
            df.select(cols)?
        }
        _ => df,
    };

    // Output
    super::output(&mut df, output.as_str())
}
