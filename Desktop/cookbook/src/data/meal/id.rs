use crate::dto::meal::recipe::MealRecipe;
use crate::models::meal::{AllMealsApi, MealApi};

use reqwest::blocking::get;

pub fn get_meal_recipe_by_id(id: i32) -> MealRecipe {
    match get_meal_by_id(id) {
        Some(meal) => MealRecipe::from_meal(meal),
        None => MealRecipe::default(),
    }
}

fn get_meal_by_id(id: i32) -> Option<MealApi> {
    let id = id.to_string();
    let url = format!("https://www.themealdb.com/api/json/v1/1/lookup.php?i=") + &id;

    let recieved_meals: AllMealsApi = get(url).unwrap().json().unwrap();
    if recieved_meals.meals.get(0).is_some() {
        return Some(recieved_meals.meals[0].clone());
    }
    None
}

#[test]
fn get_meal_by_id_test() {
    let should_be_meal = get_meal_by_id(52772);
    assert!(
        should_be_meal.is_some(),
        " Lookup full meal details by id is 'None', is API online?"
    );
}
