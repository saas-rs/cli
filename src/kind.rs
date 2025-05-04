use globwalk::DirEntry;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, PartialEq)]
pub enum Kind {
    Account,
    ApiKey,
    Plan,
    Service,
}

#[derive(Debug, Deserialize)]
struct Any {
    #[allow(dead_code)]
    kind: Option<Kind>,
}

#[allow(dead_code)]
pub fn is_kind(dir_entry: DirEntry, match_kind: Kind) -> Result<Option<DirEntry>, Box<dyn Error>> {
    let contents = std::fs::read(dir_entry.path())?;
    let any: Any = serde_yaml::from_slice(&contents)?;
    Ok(match any.kind {
        None => Some(dir_entry),
        Some(kind) => {
            if kind == match_kind {
                Some(dir_entry)
            } else {
                None
            }
        }
    })
}
