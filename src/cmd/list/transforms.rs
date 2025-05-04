use polars::prelude::*;

pub fn select_existing_cols(df: DataFrame, columns: &[&str]) -> PolarsResult<DataFrame> {
    let cols: Vec<_> = columns.iter().filter(|col| df.column(col).is_ok()).cloned().collect();
    df.select(cols)
}
