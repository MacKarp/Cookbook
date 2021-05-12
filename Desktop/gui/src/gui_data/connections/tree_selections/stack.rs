use crate::gui_data::GuiData;

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
    let selected_value = tree_model.get_value(&tree_iter, 0);
    let selected_value2 = tree_model.get_value(&tree_iter, 1);
    let value = selected_value.get::<String>().unwrap().unwrap();
    let value2 = selected_value2.get::<i32>().unwrap().unwrap();

    println!("Test: {:?}, {:?}", value, value2);

    update_stack(gui_data);
}

fn update_stack(gui_data: &GuiData) {
    let stack = gui_data.main_window_stack.stack.clone();

    stack.set_visible_child_name("page0");
}
