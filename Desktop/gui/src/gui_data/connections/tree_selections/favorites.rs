use gtk::prelude::*;

use crate::gui_data::GuiData;

pub fn favorite_tree_selection(gui_data: &GuiData) {
    println!("favorites update");
    let gui_data = gui_data.clone();
    let tree_selection = gui_data
        .main_window_category_notebook
        .favorite_tree_selection
        .clone();
    tree_selection.connect_changed(move |_| on_favorite_selection_changed(&gui_data));
}

fn on_favorite_selection_changed(gui_data: &GuiData) -> () {
    let tree_selection = gui_data
        .main_window_category_notebook
        .favorite_tree_selection
        .clone();

    let t = tree_selection.get_selected().unwrap();
    let tree_model = t.0;
    let tree_iter = t.1;

    let selected_value_id = tree_model.get_value(&tree_iter, 1);
    let _selected_value_id = selected_value_id
        .get::<String>()
        .unwrap()
        .unwrap()
        .parse::<i32>()
        .unwrap_or(0);
    let selected_category = tree_model.get_value(&tree_iter, 2);
    let _selected_category = selected_category.get::<String>().unwrap().unwrap();
}
/*
fn update_stack(gui_data: &GuiData, value: (u32, String)) {
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
let selected_category = get_filtered_meal_category_items(&value);
let selected_category = selected_category.filtered_meals;

tree_store.clear();
for s in selected_category {
    let id = s.id_meal.parse::<i32>().unwrap_or_default();
    tree_store.insert_with_values(
        None,
        None,
        &[0, 1, 2],
        &[&s.str_meal, &id, &String::from("Meal")],
    );
}

tree_view.expand_all();
previous_stack_button.set_sensitive(true);

stack.set_visible_child_name("page1");
}
*/
