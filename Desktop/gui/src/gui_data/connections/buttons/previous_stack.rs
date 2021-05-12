use gtk::prelude::*;

use crate::gui_data::GuiData;

pub fn previous_stack_button(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let button = gui_data.main_window_buttons.previous_stack_button.clone();

    button.connect_clicked(move |_| on_previous_stack_button_clicked(&gui_data));
}

pub fn on_previous_stack_button_clicked(gui_data: &GuiData) {
    let stack = gui_data.main_window_stack.stack.clone();
    let page = stack.get_visible_child_name();
    if let Some(p) = page {
        match p.as_str() {
            "page0" => stack.set_visible_child_name("page1"),
            "page1" => stack.set_visible_child_name("page0"),
            _ => {}
        }
    }
}

#[test]
fn on_previous_stack_button_clicked_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let gui_data = crate::gui_data::GuiData::new();

    assert_eq!((), on_previous_stack_button_clicked(&gui_data));
}
