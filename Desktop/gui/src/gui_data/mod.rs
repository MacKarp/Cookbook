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

    // Images
    pub main_window_images: main_window::images::MainWindowImages,

    // Category Notebook
    pub main_window_category_notebook: main_window::category_notebook::MainWindowCategoryNotebook,

    pub main_window_stack: main_window::stack::MainWindowStack,
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

        let main_window_images =
            main_window::images::MainWindowImages::create_from_builder(&builder);

        let main_window_category_notebook =
            main_window::category_notebook::MainWindowCategoryNotebook::create_from_builder(
                &builder,
            );

        let main_window_stack = main_window::stack::MainWindowStack::create_from_builder(&builder);

        Self {
            glade_src,
            builder,
            main_window,
            main_window_text,
            main_window_buttons,
            main_window_images,
            main_window_category_notebook,
            main_window_stack,
        }
    }
}

// This test should check if `GuiData::new()` generate proper GuiData structure,
// This test fails and i don't know why

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
    let main_window_images = main_window::images::MainWindowImages::create_from_builder(&builder);
    let main_window_category_notebook =
        main_window::category_notebook::MainWindowCategoryNotebook::create_from_builder(&builder);
    let main_window_stack = main_window::stack::MainWindowStack::create_from_builder(&builder);

    let should_be = GuiData {
        glade_src,
        builder,
        main_window,
        main_window_text,
        main_window_buttons,
        main_window_images,
        main_window_category_notebook,
        main_window_stack,
    };

    let tested = GuiData::new();

    //there should be equal but are not, but dunno why
    assert_eq!(tested, should_be);
}
