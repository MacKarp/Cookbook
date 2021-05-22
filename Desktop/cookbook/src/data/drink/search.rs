use crate::dto::drink::recipe::DrinkRecipe;
use crate::models::drink::AllDrinksApi;

use reqwest::blocking::get;

pub fn get_drink_recipe_by_search(searched: &str) -> Vec<DrinkRecipe> {
    let mut drinks = Vec::<DrinkRecipe>::new();
    let searched_result = get_drink_by_search(searched);
    if searched_result.is_ok() {
        match searched_result.ok().unwrap() {
            Some(drink) => {
                for d in drink.drinks {
                    drinks.push(DrinkRecipe::from_drink(d));
                }
            }
            None => {
                drinks.push(DrinkRecipe::default());
                return drinks;
            }
        }
    }

    drinks
}

fn get_drink_by_search(searched: &str) -> Result<Option<AllDrinksApi>, reqwest::Error> {
    let url = "https://www.thecocktaildb.com/api/json/v1/1/search.php?s=".to_string() + searched;

    let recived_drink: AllDrinksApi = get(url)?.json()?;
    if recived_drink.drinks.get(0).is_some() {
        return Ok(Some(recived_drink));
    }
    Ok(None)
}

#[test]
fn get_drink_by_search_test() {
    let should_be_drink = get_drink_by_search(&String::from("Margarita")).unwrap();
    assert!(
        should_be_drink.is_some(),
        "Search cocktail by name is 'None', is API online?"
    );
}
