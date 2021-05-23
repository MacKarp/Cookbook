use gtk::prelude::*;

use firebase_handler::email_handler::login_with_email;

use crate::gui::initialize_user;
use crate::gui_data::GuiData;
use crate::{gui::favorites_update, gui_data::login_window::LoginWindow};

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
            let login_error_label = login_window.login_error_label.clone();

            login_error_label.set_text("");

            let gui_data = gui_data.clone();
            email_login_button
                .connect_clicked(move |_| on_email_login_button_clicked(&gui_data, &login_window));

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

fn on_email_login_button_clicked(gui_data: &GuiData, login_window: &LoginWindow) {
    let email_login_entry = login_window.email_login_entry.clone();
    let password_login_entry = login_window.password_login_entry.clone();
    let window = login_window.window.clone();

    let email = email_login_entry.get_text();
    let email = email.as_str();

    let password = password_login_entry.get_text();
    let password = password.as_str();

    let session_result = login_with_email(email, password);

    match session_result {
        Ok(session) => {
            let login_session =
                firebase_handler::write_cached_refresh_token(session.user_id.as_str());

            match login_session {
                Ok(()) => {
                    window.hide();
                    initialize_user(&gui_data);
                    favorites_update(&gui_data);
                }
                Err(e) => {
                    println!("Login session error: {}", e);
                    let login_error_label = login_window.login_error_label.clone();
                    login_error_label.set_text("Incorrect email or password");
                }
            }
        }
        Err(e) => {
            println!("Login error: {}", e);
            let login_error_label = login_window.login_error_label.clone();
            login_error_label.set_text("Incorrect email or password");
        }
    }
}
