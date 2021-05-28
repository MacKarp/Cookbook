use gtk::prelude::*;

use cookbook::data::drink::id::get_drink_recipe_by_id;
use cookbook::data::meal::id::get_meal_recipe_by_id;

use firebase_handler::favorites::{
    remove_favorite_recipe, save_favorite_drink_recipe, save_favorite_meal_recipe,
};

use crate::gui::{favorites_update, set_favorite_button_image};
use crate::gui_data::GuiData;

pub fn favorite_button(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let button = gui_data.main_window_buttons.favorite_button.clone();

    button.connect_clicked(move |_| on_favorite_button_clicked(&gui_data));
}

pub fn on_favorite_button_clicked(gui_data: &GuiData) {
    let button = gui_data.main_window_buttons.favorite_button.clone();
    let button_label = button.get_label().unwrap().to_string();
    match button_label.as_str() {
        "Add to favorites" => add_to_favorites(&gui_data),
        "Remove from favorites" => remove_from_favorites(&gui_data),
        _ => {}
    };
}

fn remove_from_favorites(gui_data: &GuiData) {
    let favorite_doc_id = gui_data
        .main_window_text
        .displayed_recipe_favorite_document_id_text_buffer
        .clone();
    let document_id = favorite_doc_id
        .get_text(
            &favorite_doc_id.get_start_iter(),
            &favorite_doc_id.get_end_iter(),
            false,
        )
        .unwrap()
        .to_string();
    let result = remove_favorite_recipe(&document_id);
    match result {
        Ok(()) => {
            let favorite_document_id = gui_data
                .main_window_text
                .displayed_recipe_favorite_document_id_text_buffer
                .clone();
            favorite_document_id.set_text("");
        }
        Err(e) => println!("Error while removing from favorite: {}", e),
    };
    favorites_update(&gui_data);
}

fn add_to_favorites(gui_data: &GuiData) {
    let recipe_type = gui_data
        .main_window_text
        .displayed_recipe_type_text_buffer
        .clone();
    let recipe_id = gui_data
        .main_window_text
        .displayed_recipe_id_text_buffer
        .clone();

    let recipe_type = recipe_type
        .get_text(
            &recipe_type.get_start_iter(),
            &recipe_type.get_end_iter(),
            false,
        )
        .unwrap()
        .to_string();

    let recipe_id = recipe_id
        .get_text(
            &recipe_id.get_start_iter(),
            &recipe_id.get_end_iter(),
            false,
        )
        .unwrap()
        .parse::<i32>()
        .unwrap_or_default();

    match recipe_type.as_str() {
        "Meal" => {
            let result = save_favorite_meal_recipe(get_meal_recipe_by_id(recipe_id));
            match result {
                Ok(r) => {
                    let favorite_button = gui_data.main_window_buttons.favorite_button.clone();
                    favorite_button.set_label("Remove from favorites");

                    let favorite_image = gui_data.main_window_favorite_button_image.clone();
                    if let Some(p) = set_favorite_button_image(&gui_data) {
                        favorite_image.set_from_pixbuf(Some(&p))
                    }
                    let favorite_id = gui_data
                        .main_window_text
                        .displayed_recipe_favorite_document_id_text_buffer
                        .clone();
                    favorite_id.set_text(&r.document_id);
                }
                Err(e) => println!("Adding to favorites error: {}", e),
            }
        }
        "Drink" => {
            let result = save_favorite_drink_recipe(get_drink_recipe_by_id(recipe_id));
            match result {
                Ok(r) => {
                    let favorite_button = gui_data.main_window_buttons.favorite_button.clone();
                    favorite_button.set_label("Remove from favorites");

                    let favorite_image = gui_data.main_window_favorite_button_image.clone();
                    if let Some(p) = set_favorite_button_image(&gui_data) {
                        favorite_image.set_from_pixbuf(Some(&p))
                    }
                    let favorite_id = gui_data
                        .main_window_text
                        .displayed_recipe_favorite_document_id_text_buffer
                        .clone();
                    favorite_id.set_text(&r.document_id);
                }
                Err(e) => println!("Adding to favorites error: {}", e),
            }
        }
        _ => println!("No recipe type"),
    }
    favorites_update(&gui_data);
}

#[test]
fn on_favorite_button_clicked_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let gui_data = crate::gui_data::GuiData::new();

    assert_eq!((), on_favorite_button_clicked(&gui_data));
}
