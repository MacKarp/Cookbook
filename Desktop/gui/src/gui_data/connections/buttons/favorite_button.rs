use gtk::prelude::*;

use firebase_handler::favorites::save_favorite_meal_recipe;

use crate::gui_data::GuiData;

pub fn favorite_button(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let button = gui_data.main_window_buttons.favorite_button.clone();

    button.connect_clicked(move |_| on_favorite_button_clicked(&gui_data));
}

pub fn on_favorite_button_clicked(_gui_data: &GuiData) {
    save_favorite_meal_recipe();
}

#[test]
fn on_favorite_button_clicked_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let gui_data = crate::gui_data::GuiData::new();

    assert_eq!((), on_previous_stack_button_clicked(&gui_data));
}
