use crate::gui_data::GuiData;
use cookbook::dto::drink::recipe::DrinkRecipe;
use cookbook::dto::meal::recipe::MealRecipe;

use gdk_pixbuf::{InterpType, Pixbuf};
use gio::{MemoryInputStream, NONE_CANCELLABLE};
use gtk::prelude::*;

pub fn set_meal_recipe_fields(gui_data: &GuiData, meal: MealRecipe) {
    set_meal_recipe_text_fields(&gui_data, &meal);
    set_meal_recipe_image_fields(&gui_data, &meal);
}

fn set_meal_recipe_text_fields(gui_data: &GuiData, meal: &MealRecipe) {
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

fn set_meal_recipe_image_fields(gui_data: &GuiData, meal: &MealRecipe) {
    let image_recipe = gui_data.main_window_images.image_recipe.clone();
    image_recipe.set_from_pixbuf(Some(&get_recipe_image(&meal.thumb)));
}

pub fn set_drink_recipe_fields(gui_data: &GuiData, drink: DrinkRecipe) {
    set_drink_recipe_text_fields(&gui_data, &drink);
    set_drink_recipe_image_fields(&gui_data, &drink);
}

fn set_drink_recipe_text_fields(gui_data: &GuiData, drink: &DrinkRecipe) {
    let recipe_name_text_buffer = gui_data.main_window_text.recipe_name_text_buffer.clone();
    let recipe_ingredients_text_buffer = gui_data
        .main_window_text
        .recipe_ingredients_text_buffer
        .clone();
    let recipe_text_buffer = gui_data.main_window_text.recipe_text_buffer.clone();

    recipe_name_text_buffer.set_text(&*drink.drink_name);
    recipe_ingredients_text_buffer.set_text(&*get_recipe_ingredients(&drink.ingredients));
    recipe_text_buffer.set_text(&*drink.instructions);
}

fn set_drink_recipe_image_fields(gui_data: &GuiData, drink: &DrinkRecipe) {
    let image_recipe = gui_data.main_window_images.image_recipe.clone();
    image_recipe.set_from_pixbuf(Some(&get_recipe_image(&drink.thumb)));
}

fn get_recipe_image(thumb: &str) -> Pixbuf {
    let result = reqwest::blocking::get(thumb).unwrap();
    let bytes = result.bytes().unwrap().to_vec();
    let bytes = glib::Bytes::from(&bytes.to_vec());
    let stream = MemoryInputStream::from_bytes(&bytes);
    let pix = Pixbuf::from_stream(&stream, NONE_CANCELLABLE).unwrap();
    pix.scale_simple(450, 450, InterpType::Bilinear).unwrap()
}

fn get_recipe_ingredients(ingredients: &[String]) -> String {
    let mut list = String::new();
    for ingredient in ingredients {
        list += &(ingredient.clone() + "\n");
    }
    list
}
