use crate::models::{
    meal::{Meal, Meals},
    recipe::MealRecipe,
};
use reqwest::blocking::get;

pub fn get_random_meal() -> Option<Meal> {
    let recieved_meals: Meals = get("https://www.themealdb.com/api/json/v1/1/random.php")
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
        Some(meal) => return MealRecipe::from_meal(meal),
        None => return MealRecipe::default(),
    };
}

#[test]
fn get_random_meal_test() {
    let should_be_meal = get_random_meal();
    assert!(
        should_be_meal.is_some(),
        format!("Random meal is 'None', is API online?")
    );
}
