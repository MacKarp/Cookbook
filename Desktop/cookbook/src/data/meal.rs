use crate::models::meal::{Meal, Meals};
use reqwest::blocking::get;

pub fn get_random_recipe() -> Option<Meal> {
    let recieved_meals: Meals = get("https://www.themealdb.com/api/json/v1/1/random.php")
        .unwrap()
        .json()
        .unwrap();
    if recieved_meals.meals.get(0).is_some() {
        return Some(recieved_meals.meals[0].clone());
    }
    None
}

#[test]
fn get_random_recipe_test() {
    let should_be_meal = get_random_recipe();
    assert!(
        should_be_meal.is_some(),
        format!("Random meal is 'None', is API online?")
    );
}
