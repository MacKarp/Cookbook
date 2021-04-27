use gtk::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct MainWindowStack {
    pub stack: gtk::Stack,

    pub recipe_editor_box: gtk::Box,
}

impl MainWindowStack {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let stack: gtk::Stack = builder
            .get_object("main_window_stack")
            .expect("\"main_window_stack\" ID in \"Main_Window.glade\" should exist.");
        let recipe_editor_box: gtk::Box = builder
            .get_object("recipe_editor_box")
            .expect("\"recipe_editor_box\" ID in \"Main_Window.glade\" should exist.");

        Self {
            stack,
            recipe_editor_box,
        }
    }
}

#[test]
fn create_from_builder_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let glade_src = include_str!("../../../ui/Cookbook.glade").to_string();
    let builder = gtk::Builder::from_string(glade_src.as_str());

    let stack: gtk::Stack = builder
        .get_object("main_window_stack")
        .expect("\"main_window_stack\" ID in \"Main_Window.glade\" should exist.");
    let recipe_editor_box: gtk::Box = builder
        .get_object("recipe_editor_box")
        .expect("\"recipe_editor_box\" ID in \"Main_Window.glade\" should exist.");

    let should_be = MainWindowStack {
        stack,
        recipe_editor_box,
    };

    let tested = MainWindowStack::create_from_builder(&builder);
    assert_eq!(tested, should_be);
}
