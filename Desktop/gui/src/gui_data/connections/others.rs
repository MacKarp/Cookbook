use gtk::prelude::*;

use cookbook::data::drink::search::get_drink_recipe_by_search;
use cookbook::data::meal::search::get_meal_recipe_by_search;

use crate::gui::set_favorite_button_image;
use crate::gui_data::GuiData;

pub fn connect_search_bar(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let search_bar = gui_data.main_window_search_bar.clone();

    search_bar.connect_search_changed(move |_| on_search_bar_search_changed(&gui_data));
}

pub fn connect_document_id_buffer(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let text_buffer = gui_data
        .main_window_text
        .displayed_recipe_favorite_document_id_text_buffer
        .clone();

    text_buffer.connect_changed(move |_| on_document_id_text_buffer_changed(&gui_data));
}

fn on_document_id_text_buffer_changed(gui_data: &GuiData) {
    let text_buffer = gui_data
        .main_window_text
        .displayed_recipe_favorite_document_id_text_buffer
        .clone();
    let document_id = text_buffer
        .get_text(
            &text_buffer.get_start_iter(),
            &text_buffer.get_end_iter(),
            false,
        )
        .unwrap()
        .to_string();

    let favorite_button = gui_data.main_window_buttons.favorite_button.clone();

    match document_id.as_str() {
        "" => {
            favorite_button.set_label("Add to favorites");

            let favorite_image = gui_data.main_window_favorite_button_image.clone();
            if let Some(p) = set_favorite_button_image(&gui_data) {
                favorite_image.set_from_pixbuf(Some(&p))
            }
        }

        _ => {
            favorite_button.set_label("Remove from favorites");

            let favorite_image = gui_data.main_window_favorite_button_image.clone();
            if let Some(p) = set_favorite_button_image(&gui_data) {
                favorite_image.set_from_pixbuf(Some(&p))
            }
        }
    };
}

pub fn on_search_bar_search_changed(gui_data: &GuiData) {
    let search_bar = gui_data.main_window_search_bar.clone();
    let searched_name = search_bar.get_text().parse::<String>().unwrap();

    update_stack(gui_data, &searched_name);
}

fn update_stack(gui_data: &GuiData, searched_name: &str) {
    let stack = gui_data.main_window_stack.stack.clone();
    let previous_stack_button = gui_data.main_window_buttons.previous_stack_button.clone();
    let tree_store = gui_data
        .main_window_stack
        .selected_category_tree_store
        .clone();
    let tree_view = gui_data
        .main_window_stack
        .selected_category_tree_view
        .clone();
    let selected_category_tree_selection = gui_data
        .main_window_stack
        .selected_category_tree_selection
        .clone();

    selected_category_tree_selection.unselect_all();
    tree_store.clear();

    insert_meal_recipes(&gui_data, &searched_name);
    insert_drink_recipes(&gui_data, &searched_name);

    tree_view.expand_all();
    previous_stack_button.set_sensitive(true);

    stack.set_visible_child_name("page1");
}

fn insert_drink_recipes(gui_data: &GuiData, searched_name: &str) {
    let tree_store = gui_data
        .main_window_stack
        .selected_category_tree_store
        .clone();

    let searched_recipe = get_drink_recipe_by_search(&searched_name);

    for d in searched_recipe {
        let id = d.id as i32;
        tree_store.insert_with_values(
            None,
            None,
            &[0, 1, 2],
            &[&d.drink_name, &id, &String::from("Drink")],
        );
    }
}

fn insert_meal_recipes(gui_data: &GuiData, searched_name: &str) {
    let tree_store = gui_data
        .main_window_stack
        .selected_category_tree_store
        .clone();

    let searched_recipe = get_meal_recipe_by_search(&searched_name);

    for m in searched_recipe {
        let id = m.id as i32;
        tree_store.insert_with_values(
            None,
            None,
            &[0, 1, 2],
            &[&m.meal_name, &id, &String::from("Meal")],
        );
    }
}

#[test]
fn on_search_bar_search_changed_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let gui_data = crate::gui_data::GuiData::new();

    assert_eq!((), on_search_bar_search_changed(&gui_data));
}
