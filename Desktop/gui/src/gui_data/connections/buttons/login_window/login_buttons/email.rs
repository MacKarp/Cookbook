use firebase_handler::email_handler::login_with_email;
use gtk::prelude::*;

use crate::gui::favorites_update;
use crate::gui::initialize_user;
use crate::gui_data::login_window::LoginWindow;
use crate::gui_data::GuiData;

pub fn on_email_login_button_clicked(gui_data: &GuiData, login_window: &LoginWindow) {
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
