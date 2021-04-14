use crate::models::drink::api::{AllDrinksAPI, DrinkAPI};
use crate::models::drink::recipe::DrinkRecipe;

use reqwest::blocking::get;

pub fn get_random_drink() -> Option<DrinkAPI> {
    let recieved_cocktail: AllDrinksAPI =
        get(" https://www.thecocktaildb.com/api/json/v1/1/random.php")
            .unwrap()
            .json()
            .unwrap();
    if recieved_cocktail.drinks.get(0).is_some() {
        return Some(recieved_cocktail.drinks[0].clone());
    }
    None
}

pub fn get_random_drink_recipe() -> DrinkRecipe {
    match get_random_drink() {
        Some(drink) => return DrinkRecipe::from_drink(drink),
        None => return DrinkRecipe::default(),
    };
}

#[test]
fn get_random_drink_test() {
    let should_be_drink = get_random_drink();
    assert!(
        should_be_drink.is_some(),
        "Random drink is 'None', is API online?"
    );
}