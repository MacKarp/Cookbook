use gtk::prelude::*;

use crate::buttons::login_window::login_window_buttons::on_email_login_button_clicked;
use crate::buttons::login_window::login_window_buttons::on_google_login_button_clicked;
use crate::gui::initialize_user;
use crate::gui_data::GuiData;
use crate::{gui::favorites_update, gui_data::login_window::LoginWindow};

pub mod login_window_buttons;

pub fn login_button(gui_data: &GuiData) {
    let gui_data = gui_data.clone();
    let button = gui_data.main_window_buttons.login_button.clone();

    button.connect_clicked(move |_| on_login_button_clicked(&gui_data));
}

fn on_login_button_clicked(gui_data: &GuiData) {
    let button = gui_data.main_window_buttons.login_button.clone();
    let button_text = button.get_label().unwrap();
    let button_text = button_text.as_str();
    match button_text {
        "Login" => {
            let src = gui_data.glade_src.clone();
            let builder = gtk::Builder::from_string(src.as_str());
            let login_window = LoginWindow::create_from_builder(&builder);
            let window = login_window.window.clone();
            let email_login_button = login_window.email_login_button.clone();
            let google_login_button = login_window.google_login_button.clone();
            let _facebook_login_button = login_window.facebook_login_button.clone();
            let login_error_label = login_window.login_error_label.clone();

            login_error_label.set_text("");

            let email_gui_data = gui_data.clone();
            let email_login_window = login_window.clone();
            email_login_button.connect_clicked(move |_| {
                on_email_login_button_clicked(&email_gui_data, &email_login_window)
            });

            let google_gui_data = gui_data.clone();
            let google_login_window = login_window.clone();
            google_login_button.connect_clicked(move |_| {
                on_google_login_button_clicked(&google_gui_data, &google_login_window)
            });

            window.show_all();
        }
        "Logout" => {
            let x = std::fs::write("token.txt", "");
            match x {
                Ok(_) => {
                    initialize_user(&gui_data);
                    favorites_update(&gui_data);
                    button.set_label("Login");
                }
                Err(e) => {
                    println!("Loggin out error: {}", e);
                    on_login_button_clicked(&gui_data);
                }
            }
        }
        _ => {}
    }
}
