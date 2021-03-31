use gtk::prelude::*;

pub mod connections;
pub mod main_window;

#[derive(Clone, PartialEq, Debug)]
pub struct GuiData {
    // Glade builder
    pub glade_src: String,
    pub builder: gtk::Builder,

    // Windows
    pub main_window: gtk::Window,

    // Text buffers
    pub main_window_text: main_window::text::MainWindowText,

    // Buttons
    pub main_window_buttons: main_window::buttons::MainWindowButtons,
}

impl GuiData {
    pub fn new() -> Self {
        // Glade builder
        let glade_src = include_str!("../../ui/Cookbook.glade").to_string();
        let builder = gtk::Builder::from_string(glade_src.as_str());

        // Windows
        let main_window: gtk::Window = builder
            .get_object("main_window")
            .expect("\"main_window\" ID in \"Cookbook.glade\" should exist.");
        main_window.show_all();

        let main_window_text = main_window::text::MainWindowText::create_from_builder(&builder);
        let main_window_buttons =
            main_window::buttons::MainWindowButtons::create_from_builder(&builder);
        Self {
            glade_src,
            builder,
            main_window,
            main_window_text,
            main_window_buttons,
        }
    }
}
#[test]
#[ignore]
fn new_gui_data_test() {
    gtk::init().expect("Failed to initialize GTK...");

    let glade_src = include_str!("../../ui/Cookbook.glade").to_string();
    let builder = gtk::Builder::from_string(glade_src.as_str());
    let main_window: gtk::Window = builder
        .get_object("main_window")
        .expect("\"main_window\" ID in \"Cookbook.glade\" should exist.");
    let main_window_text = main_window::text::MainWindowText::create_from_builder(&builder);
    let main_window_buttons =
        main_window::buttons::MainWindowButtons::create_from_builder(&builder);

    let should_be = GuiData {
        glade_src,
        builder,
        main_window,
        main_window_text,
        main_window_buttons,
    };

    let tested = GuiData::new();

    //there should be equal but are not, but dunno why(different pointers?)
    assert_eq!(tested, should_be);
}
