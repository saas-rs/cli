use crate::apiclient;
use crate::protocol::saas_rs::user::v1::FindManyActionsRequest;
use clap::Parser;
use polars::prelude::{JsonReader, SerReader};
use std::io::Cursor;

pub const PS_COLUMNS: &[&str] = &["id", "createdAt", "service", "version", "method"];

#[derive(Debug, Parser)]
pub struct Opts {
    #[arg(long = "limit")]
    pub limit: Option<u32>,

    #[arg(long = "offset")]
    pub offset: Option<u32>,

    /// Output format
    #[arg(short = 'o', long = "output", default_value = "ps", value_parser = clap::builder::PossibleValuesParser::new(crate::OUTPUTS)
    )]
    pub output: String,
}

pub async fn run(offset: Option<u32>, limit: Option<u32>, output: String) -> Result<(), Box<dyn std::error::Error>> {
    // Request all records
    let mut client = apiclient::new_user_service_client().await?;
    let req = FindManyActionsRequest {
        filter: None,
        field_mask: None,
        limit,
        offset,
        sort: None,
    };
    let res = client.find_many_actions(req).await?.into_inner();

    // Without a schema, Polars chokes parsing an empty json array
    if res.actions.is_empty() {
        return crate::cmd::list::output_empty_result_of_unknown_schema(&output);
    }

    // Convert to dataframe in preparation for output
    let json = serde_json::to_vec(&res.actions)?;
    let file = Cursor::new(json);
    let df = JsonReader::new(file).finish()?;

    // Apply type-specific narrow formatting
    let mut df = match output.as_str() {
        "ps" => crate::cmd::list::select_existing_cols(df, PS_COLUMNS)?,
        _ => df,
    };

    // Output
    crate::cmd::list::output(&mut df, output.as_str())
}
