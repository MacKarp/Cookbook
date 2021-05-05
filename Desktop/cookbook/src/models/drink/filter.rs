use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllDrinkFilteredApi {
    pub drinks: Vec<DrinkApi>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrinkApi {
    pub str_drink: Option<String>,
    pub str_drink_thumb: Option<String>,
    pub id_drink: Option<String>,
}

impl AllDrinkFilteredApi {
    pub fn default() -> Self {
        let ingredient = DrinkApi {
            id_drink: Some(String::from("0")),
            str_drink: None,
            str_drink_thumb: None,
        };
        Self {
            drinks: vec![ingredient],
        }
    }
}
