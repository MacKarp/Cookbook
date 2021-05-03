use crate::gui_data::GuiData;
use cookbook::data::meal::filtered::get_filtered_meal_category_items;

use gtk::prelude::*;

pub fn category_tree_selection(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let tree_selection = gui_data
        .main_window_category_notebook
        .meal_category_tree_selection
        .clone();
    tree_selection.connect_changed(move |_| on_meal_category_selection_changed(&gui_data));
}

fn on_meal_category_selection_changed(gui_data: &GuiData) -> () {
    let tree_selection = gui_data
        .main_window_category_notebook
        .meal_category_tree_selection
        .clone();
    let t = tree_selection.get_selected().unwrap();
    let tree_model = t.0;
    let tree_iter = t.1;

    let category_iter = tree_model.iter_parent(&tree_iter);

    let mut category = None;

    if category_iter.is_some() {
        let category_iter = category_iter.unwrap();
        let category_value = tree_model.get_value(&category_iter, 0);
        category = category_value.get::<String>().unwrap();
    }

    let selected_value = tree_model.get_value(&tree_iter, 0);
    let value = selected_value.get::<String>().unwrap().unwrap();

    update_stack(gui_data, value, category);
}

fn update_stack(gui_data: &GuiData, value: String, category: Option<String>) {
    if category.is_none() {
        return;
    }
    println!("update_stack");

    let category = category.unwrap();
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

    let selected_category = get_filtered_meal_category_items(&category);
    let selected_category = selected_category.filtered_meals;
    for s in selected_category {
        tree_store.insert_with_values(None, None, &[0], &[&s.str_meal]);
    }

    tree_view.expand_all();
    previous_stack_button.set_sensitive(true);

    stack.set_visible_child_name("page1");
}

#[test]
fn on_meal_category_selection_changed_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let gui_data = crate::gui_data::GuiData::new();

    assert_eq!((), on_meal_category_selection_changed(&gui_data));
}
