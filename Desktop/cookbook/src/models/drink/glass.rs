use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllGlassAPI {
    pub drinks: Vec<GlassAPI>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlassAPI {
    pub str_glass: Option<String>,
}
