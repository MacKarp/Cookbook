use crate::gui_data::GuiData;
use cookbook::{data::meal::get_random_meal_recipe, models::recipe::MealRecipe};
use gtk::{ButtonExt, TextBufferExt};

pub fn connect_random_recipe_button(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let random_meal_button = gui_data
        .main_window_buttons
        .random_meal_recipe_button
        .clone();
    random_meal_button.connect_clicked(move |_| on_random_recipe_button_clicked(&gui_data));
}

fn on_random_recipe_button_clicked(gui_data: &GuiData) {
    let meal = get_random_meal_recipe();
    set_recipe_fields(&gui_data, meal);
}

fn set_recipe_fields(gui_data: &GuiData, meal: MealRecipe) {
    set_recipe_text_fields(&gui_data, meal);
}

fn set_recipe_text_fields(gui_data: &GuiData, meal: MealRecipe) {
    let recipe_name_text_buffer = gui_data.main_window_text.recipe_name_text_buffer.clone();
    let recipe_ingredients_text_buffer = gui_data
        .main_window_text
        .recipe_ingredients_text_buffer
        .clone();
    let recipe_text_buffer = gui_data.main_window_text.recipe_text_buffer.clone();

    recipe_name_text_buffer.set_text(&*meal.meal_name);
    recipe_ingredients_text_buffer.set_text(&*get_recipe_ingredients(&meal.ingredients));
    recipe_text_buffer.set_text(&*meal.instructions);
}

fn get_recipe_ingredients(ingredients: &Vec<String>) -> String {
    let mut list = String::new();
    for ingredient in ingredients {
        list += &(ingredient.clone() + "\n");
    }
    list
}

#[test]
fn on_random_recipe_button_clicked_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let gui_data = crate::gui_data::GuiData::new();

    assert_eq!((), on_random_recipe_button_clicked(&gui_data));
}
