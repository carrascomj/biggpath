use serde::{Deserialize, Serialize};

/// Metabolite to be matched against reactome
#[derive(Deserialize, Serialize)]
pub struct Met<'a> {
    /// reactome id. Default: "R-ALL-00000"
    pub id: &'a str,
    /// species id. Default: "9606" (Homo Sapiens)
    pub species: &'a str,
}

impl<'a> Default for Met<'a> {
    fn default() -> Self {
        Met {
            id: "R-ALL-00000",
            species: "9606",
        }
    }
}

/// Pathway contains the id from reactome
#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Pathway {
    pub db_id: i32
}
