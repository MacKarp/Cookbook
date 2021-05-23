pub mod favorite_button;
pub mod login_window;
pub mod previous_stack;
pub mod random_drink;
pub mod random_meal;
use crate::gui_data::GuiData;

pub fn connect(gui_data: &GuiData) {
    random_meal::random_button(&gui_data);
    random_drink::random_button(&gui_data);
    previous_stack::previous_stack_button(&gui_data);
    login_window::login_button(&gui_data);
    favorite_button::favorite_button(&gui_data);
}
