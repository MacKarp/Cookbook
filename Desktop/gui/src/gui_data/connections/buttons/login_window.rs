use gtk::prelude::*;

use crate::gui_data::login_window::LoginWindow;
use crate::gui_data::GuiData;

pub fn login_button(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let button = gui_data.main_window_buttons.login_button.clone();

    button.connect_clicked(move |_| on_login_button_clicked(&gui_data));
}

fn on_login_button_clicked(gui_data: &GuiData) {
    let src = gui_data.glade_src.clone();
    let builder = gtk::Builder::from_string(src.as_str());
    let login_window = LoginWindow::create_from_builder(&builder);
    let window = login_window.window.clone();
    let email_login_button = login_window.email_login_button.clone();

    email_login_button.connect_clicked(move |_| on_email_login_button_clicked(&login_window));

    window.show_all();
}

fn on_email_login_button_clicked(login_window: &LoginWindow) {
    let email_login_entry = login_window.email_login_entry.clone();
    let password_login_entry = login_window.password_login_entry.clone();
    let window = login_window.window.clone();

    let email = email_login_entry.get_text().to_string();
    let password = password_login_entry.get_text().to_string();
    println!("Test:\nemail: {}\npassword: {}", email, password);
    window.hide();
}
