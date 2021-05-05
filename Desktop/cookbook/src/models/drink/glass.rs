use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllGlassApi {
    pub drinks: Vec<GlassApi>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlassApi {
    pub str_glass: Option<String>,
}
