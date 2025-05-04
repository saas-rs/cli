use serde::Deserialize;

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
