use crate::dto::drink::recipe::DrinkRecipe;
use crate::models::drink::{AllDrinksApi, DrinkApi};

use reqwest::blocking::get;

pub fn get_drink_recipe_by_id(id: i32) -> DrinkRecipe {
    match get_drink_by_id(id) {
        Some(drink) => DrinkRecipe::from_drink(drink),
        None => DrinkRecipe::default(),
    }
}

fn get_drink_by_id(id: i32) -> Option<DrinkApi> {
    let id = id.to_string();
    let url = format!("https://www.thecocktaildb.com/api/json/v1/1/lookup.php?i=") + &id;

    let recived_drink: AllDrinksApi = get(url).unwrap().json().unwrap();
    if recived_drink.drinks.get(0).is_some() {
        return Some(recived_drink.drinks[0].clone());
    }
    None
}

#[test]
fn get_drink_by_id_test() {
    let should_be_drink = get_drink_by_id(11007);
    assert!(
        should_be_drink.is_some(),
        "Lookup full cocktail details by id is 'None', is API online?"
    );
}
