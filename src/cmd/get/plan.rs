use crate::apiclient;
use crate::cmd::list::plans;
use crate::protocol::saas_rs::user::v1::FindPlanRequest;
use clap::Parser;
use polars::prelude::*;
use std::io::Cursor;

#[derive(Debug, Parser)]
pub struct Opts {
    /// Plan ID
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
    let req = FindPlanRequest { id };
    let res = client.find_plan(req).await?.into_inner();

    // Convert to dataframe in preparation for output
    let json = serde_json::to_vec(&res.plan)?;
    let file = Cursor::new(json);
    let df = JsonReader::new(file).finish()?;

    // Apply type-specific narrow formatting
    let mut df = match output.as_str() {
        "ps" => {
            let cols: Vec<_> = plans::PS_COLUMNS
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
