use gtk::prelude::*;

use crate::gui_data::login_window::LoginWindow;
use crate::gui_data::GuiData;
use firebase_handler::email_handler::login_with_email;

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
    let login_error_label = login_window.login_error_label.clone();

    login_error_label.set_text("");

    email_login_button.connect_clicked(move |_| on_email_login_button_clicked(&login_window));

    window.show_all();
}

fn on_email_login_button_clicked(login_window: &LoginWindow) {
    let email_login_entry = login_window.email_login_entry.clone();
    let password_login_entry = login_window.password_login_entry.clone();
    let window = login_window.window.clone();

    let email = email_login_entry.get_text();
    let email = email.as_str();

    let password = password_login_entry.get_text();
    let password = password.as_str();

    println!("Test:\nemail: {}\npassword: {}", email, password);
    let session_result = login_with_email(email, password);

    match session_result {
        Ok(session) => {
            let api_key = session.api_key.clone();
            let refresh_token = session.refresh_token.clone();
            let user_id = session.user_id.clone();
            println!(
                "api key:{},\nuser_id: {},\nrefresh_token: {:?}",
                api_key, user_id, refresh_token
            );
            firebase_handler::get_user_info(&session);
            window.hide();
        }
        Err(e) => {
            println!("Login error: {}",e);
            let login_error_label = login_window.login_error_label.clone();
            login_error_label.set_text("Incorrect email or password");
        }
    }
}
