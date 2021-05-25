use gtk::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct OAuthWindow {
    pub oauth_window: gtk::Window,
}
impl OAuthWindow {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let oauth_window: gtk::Window = builder
            .get_object("oauth_window")
            .expect("\"oauth_window\" ID in \"Cookbook.glade\" should exist.");

        Self { oauth_window }
    }
}

#[test]
fn create_from_builder_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let glade_src = include_str!("../../../ui/Cookbook.glade").to_string();
    let builder = gtk::Builder::from_string(glade_src.as_str());

    let oauth_window: gtk::Window = builder
        .get_object("oauth_window")
        .expect("\"oauth_window\" ID in \"Cookbook.glade\" should exist.");

    let should_be = OAuthWindow { oauth_window };

    let tested = OAuthWindow::create_from_builder(&builder);

    assert_eq!(tested, should_be);
}
