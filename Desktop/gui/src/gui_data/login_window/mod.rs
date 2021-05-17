use gtk::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct LoginWindow {
    pub window: gtk::Window,
    pub email_login_entry: gtk::Entry,
    pub password_login_entry: gtk::Entry,
    pub email_login_button: gtk::Button,
    pub google_login_button: gtk::Button,
    pub facebook_login_button: gtk::Button,
}
impl LoginWindow {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let window: gtk::Window = builder
            .get_object("login_window")
            .expect("\"login_window\" ID in \"Cookbook.glade\" should exist.");
        let email_login_entry: gtk::Entry = builder
            .get_object("email_login_entry")
            .expect("\"email_login_entry\" ID in \"Cookbook.glade\" should exist.");
        let password_login_entry: gtk::Entry = builder
            .get_object("password_login_entry")
            .expect("\"password_login_entry\" ID in \"Cookbook.glade\" should exist.");
        let email_login_button: gtk::Button = builder
            .get_object("email_login_button")
            .expect("\"email_login_button\" ID in \"Cookbook.glade\" should exist.");
        let google_login_button: gtk::Button = builder
            .get_object("google_login_button")
            .expect("\"google_login_button\" ID in \"Cookbook.glade\" should exist.");
        let facebook_login_button: gtk::Button = builder
            .get_object("facebook_login_button")
            .expect("\"facebook_login_button\" ID in \"Cookbook.glade\" should exist.");

        Self {
            window,
            email_login_entry,
            password_login_entry,
            email_login_button,
            google_login_button,
            facebook_login_button,
        }
    }
}

#[test]
fn create_from_builder_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let glade_src = include_str!("../../../ui/Cookbook.glade").to_string();
    let builder = gtk::Builder::from_string(glade_src.as_str());

    let window: gtk::Window = builder
        .get_object("login_window")
        .expect("\"login_window\" ID in \"Cookbook.glade\" should exist.");
    let email_login_entry: gtk::Entry = builder
        .get_object("email_login_entry")
        .expect("\"email_login_entry\" ID in \"Cookbook.glade\" should exist.");
    let password_login_entry: gtk::Entry = builder
        .get_object("password_login_entry")
        .expect("\"password_login_entry\" ID in \"Cookbook.glade\" should exist.");
    let email_login_button: gtk::Button = builder
        .get_object("email_login_button")
        .expect("\"email_login_button\" ID in \"Cookbook.glade\" should exist.");
    let google_login_button: gtk::Button = builder
        .get_object("google_login_button")
        .expect("\"google_login_button\" ID in \"Cookbook.glade\" should exist.");
    let facebook_login_button: gtk::Button = builder
        .get_object("facebook_login_button")
        .expect("\"facebook_login_button\" ID in \"Cookbook.glade\" should exist.");

    let should_be = LoginWindow {
        window,
        email_login_entry,
        password_login_entry,
        email_login_button,
        google_login_button,
        facebook_login_button,
    };

    let tested = LoginWindow::create_from_builder(&builder);

    assert_eq!(tested, should_be);
}
