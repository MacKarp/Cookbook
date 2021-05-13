use crate::dto::meal::recipe::MealRecipe;
use crate::models::meal::{AllMealsApi, MealApi};

use reqwest::blocking::get;

fn get_random_meal() -> Option<MealApi> {
    let recieved_meals: AllMealsApi = get("https://www.themealdb.com/api/json/v1/1/random.php")
        .unwrap()
        .json()
        .unwrap();
    if recieved_meals.meals.get(0).is_some() {
        return Some(recieved_meals.meals[0].clone());
    }
    None
}

pub fn get_random_meal_recipe() -> MealRecipe {
    match get_random_meal() {
        Some(meal) => MealRecipe::from_meal(meal),
        None => MealRecipe::default(),
    }
}

#[test]
fn get_random_meal_test() {
    let should_be_meal = get_random_meal();
    assert!(
        should_be_meal.is_some(),
        "Random meal is 'None', is API online?"
    );
}
