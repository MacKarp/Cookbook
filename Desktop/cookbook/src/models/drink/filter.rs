use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllDrinkFilteredAPI {
    pub drinks: Vec<DrinkAPI>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrinkAPI {
    pub str_drink: Option<String>,
    pub str_drink_thumb: Option<String>,
    pub id_drink: Option<String>,
}

impl AllDrinkFilteredAPI {
    pub fn default() -> Self {
        let ingredient = DrinkAPI {
            id_drink: Some(String::from("0")),
            str_drink: None,
            str_drink_thumb: None,
        };
        Self {
            drinks: vec![ingredient],
        }
    }
}
