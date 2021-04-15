use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllAlcoholicAPI {
    pub drinks: Vec<AlcoholicAPI>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlcoholicAPI {
    pub str_alcoholic: Option<String>,
}
