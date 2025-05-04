use polars::prelude::*;
use std::error::Error;
use std::io;
use std::io::Write;

pub fn output(df: &mut DataFrame, output: &str) -> Result<(), Box<dyn Error>> {
    let _tmp_env = (
        tmp_env::set_var("POLARS_FMT_MAX_ROWS", "-1"),
        tmp_env::set_var("POLARS_FMT_TABLE_HIDE_DATAFRAME_SHAPE_INFORMATION", "1"),
    );
    let w = io::stdout();
    let mut w = io::BufWriter::new(w);
    match output {
        "arrow" => {
            IpcWriter::new(w).finish(df)?;
        }
        "json" => {
            JsonWriter::new(&mut w).finish(df)?;
        }
        "markdown" => {
            let _tmp_env = tmp_env::set_var("POLARS_FMT_TABLE_FORMATTING", "ASCII_MARKDOWN");
            writeln!(&mut w, "{df}")?;
        }
        "ps" => {
            writeln!(&mut w, "{df}")?;
        }
        _ /* "wide" */ => {
            writeln!(&mut w, "{df}")?;
        }
    }

    Ok(())
}

pub fn output_empty_result_of_unknown_schema(output: &str) -> Result<(), Box<dyn Error>> {
    let w = io::stdout();
    let mut w = io::BufWriter::new(w);
    match output {
        "arrow" => {}
        "json" => {
            writeln!(&mut w, "[]")?;
        }
        _ => {
            writeln!(&mut w, "No rows found.")?;
        }
    }

    Ok(())
}
