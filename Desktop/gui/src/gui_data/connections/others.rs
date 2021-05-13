use gtk::prelude::*;

use crate::gui_data::GuiData;
use cookbook::data::drink::search::get_drink_recipe_by_search;
use cookbook::data::meal::search::get_meal_recipe_by_search;

pub fn connect_search_bar(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let search_bar = gui_data.main_window_search_bar.clone();

    search_bar.connect_search_changed(move |_| on_search_bar_search_changed(&gui_data));
}

pub fn on_search_bar_search_changed(gui_data: &GuiData) {
    let search_bar = gui_data.main_window_search_bar.clone();
    let searched_name = search_bar.get_text().parse::<String>().unwrap();
    println!("Test: on_search_bar_search_changed: {:?}", searched_name);

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

fn insert_drink_recipes(gui_data: &GuiData, searched_name: &str)  {
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

fn insert_meal_recipes(gui_data: &GuiData, searched_name: &str)  {
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
