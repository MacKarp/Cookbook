use crate::dto::drink::recipe::DrinkRecipe;
use crate::models::drink::AllDrinksApi;

use reqwest::blocking::get;

pub fn get_drink_recipe_by_search(searched: &String) -> Vec<DrinkRecipe> {
    let mut drinks = Vec::<DrinkRecipe>::new();
    match get_drink_by_search(searched) {
        Some(drink) => {
            for d in drink.drinks {
                drinks.push(DrinkRecipe::from_drink(d));
            }
        }
        None => drinks.push(DrinkRecipe::default()),
    }
    drinks
}

fn get_drink_by_search(searched: &String) -> Option<AllDrinksApi> {
    let url = format!("https://www.thecocktaildb.com/api/json/v1/1/search.php?s=") + searched;

    let recived_drink: AllDrinksApi = get(url).unwrap().json().unwrap();
    if recived_drink.drinks.get(0).is_some() {
        return Some(recived_drink);
    }
    None
}

#[test]
fn get_drink_by_search_test() {
    let should_be_drink = get_drink_by_search(&String::from("margarrita"));
    assert!(
        should_be_drink.is_some(),
        "Search cocktail by name is 'None', is API online?"
    );
}
