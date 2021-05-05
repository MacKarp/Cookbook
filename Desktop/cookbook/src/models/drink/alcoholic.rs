use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllAlcoholicApi {
    pub drinks: Vec<AlcoholicApi>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlcoholicApi {
    pub str_alcoholic: Option<String>,
}
