use crate::gui_data::{connections::buttons::common, GuiData};
use cookbook::{data::drink::get_random_drink_recipe, models::drink::recipe::DrinkRecipe};

use gtk::prelude::*;

pub fn random_button(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let random_drink_button = gui_data
        .main_window_buttons
        .random_drink_recipe_button
        .clone();
    random_drink_button.connect_clicked(move |_| on_random_recipe_button_clicked(&gui_data));
}

fn on_random_recipe_button_clicked(gui_data: &GuiData) {
    let drink = get_random_drink_recipe();
    set_recipe_fields(&gui_data, drink);
}

fn set_recipe_fields(gui_data: &GuiData, drink: DrinkRecipe) {
    set_recipe_text_fields(&gui_data, &drink);
    set_recipe_image_fields(&gui_data, &drink);
}

fn set_recipe_text_fields(gui_data: &GuiData, drink: &DrinkRecipe) {
    let recipe_name_text_buffer = gui_data.main_window_text.recipe_name_text_buffer.clone();
    let recipe_ingredients_text_buffer = gui_data
        .main_window_text
        .recipe_ingredients_text_buffer
        .clone();
    let recipe_text_buffer = gui_data.main_window_text.recipe_text_buffer.clone();

    recipe_name_text_buffer.set_text(&*drink.drink_name);
    recipe_ingredients_text_buffer.set_text(&*common::get_recipe_ingredients(&drink.ingredients));
    recipe_text_buffer.set_text(&*drink.instructions);
}

fn set_recipe_image_fields(gui_data: &GuiData, drink: &DrinkRecipe) {
    let image_recipe = gui_data.main_window_images.image_recipe.clone();
    image_recipe.set_from_pixbuf(Some(&common::get_recipe_image(&drink.thumb)));
}
#[test]
fn on_random_recipe_button_clicked_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let gui_data = crate::gui_data::GuiData::new();

    assert_eq!((), on_random_recipe_button_clicked(&gui_data));
}
