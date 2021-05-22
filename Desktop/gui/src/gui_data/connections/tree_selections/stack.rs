use crate::field_set::set_drink_recipe_fields;
use crate::field_set::set_meal_recipe_fields;
use crate::gui_data::GuiData;
use cookbook::data::drink::id::get_drink_recipe_by_id;
use cookbook::data::meal::id::get_meal_recipe_by_id;

use gtk::prelude::*;

pub fn stack_tree_selection(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let tree_selection = gui_data
        .main_window_stack
        .selected_category_tree_selection
        .clone();
    tree_selection.connect_changed(move |_| on_selected_category_tree_selection_changed(&gui_data));
}

fn on_selected_category_tree_selection_changed(gui_data: &GuiData) {
    let tree_selection = gui_data
        .main_window_stack
        .selected_category_tree_selection
        .clone();

    let t = match tree_selection.get_selected() {
        Some(t) => t,
        None => return,
    };
    let tree_model = t.0;
    let tree_iter = t.1;
    let selected_id = tree_model.get_value(&tree_iter, 1);
    let selected_type = tree_model.get_value(&tree_iter, 2);

    let recipe_id = selected_id.get::<i32>().unwrap().unwrap();
    let recipe_type = selected_type.get::<String>().unwrap().unwrap();

    update_stack_with_recipe(gui_data, (recipe_id, recipe_type));
}

fn update_stack_with_recipe(gui_data: &GuiData, value: (i32, String)) {
    let stack = gui_data.main_window_stack.stack.clone();
    let recipe_id = value.0;
    let recipe_type = value.1;

    match recipe_type.as_str() {
        "Meal" => set_meal_recipe(&gui_data, recipe_id),
        "Drink" => set_drink_recipe(&gui_data, recipe_id),
        _ => return,
    }
    let tree_selection = gui_data
        .main_window_stack
        .selected_category_tree_selection
        .clone();
    tree_selection.unselect_all();
    stack.set_visible_child_name("page0");
}

fn set_drink_recipe(gui_data: &GuiData, recipe_id: i32) {
    let drink = get_drink_recipe_by_id(recipe_id);
    set_drink_recipe_fields(&gui_data, drink);
}

fn set_meal_recipe(gui_data: &GuiData, recipe_id: i32) {
    let meal = get_meal_recipe_by_id(recipe_id);
    set_meal_recipe_fields(&gui_data, meal);
}
