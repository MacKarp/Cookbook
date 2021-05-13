use crate::field_set::set_drink_recipe_fields;
use crate::gui_data::GuiData;
use cookbook::data::drink::get_random_drink_recipe;

use gtk::prelude::*;

pub fn random_button(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let random_drink_button = gui_data
        .main_window_buttons
        .random_drink_recipe_button
        .clone();
    random_drink_button.connect_clicked(move |_| on_random_recipe_button_clicked(&gui_data));
}

pub fn on_random_recipe_button_clicked(gui_data: &GuiData) {
    gui_data
        .main_window_stack
        .stack
        .set_visible_child(&gui_data.main_window_stack.recipe_editor_box);

    let drink = get_random_drink_recipe();
    set_drink_recipe_fields(&gui_data, drink);
}

#[test]
fn on_random_recipe_button_clicked_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let gui_data = crate::gui_data::GuiData::new();

    assert_eq!((), on_random_recipe_button_clicked(&gui_data));
}
