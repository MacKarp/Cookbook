use gtk::prelude::*;
#[derive(Clone, PartialEq, Debug)]
pub struct MainWindowText {
    pub recipe_name_text_buffer: gtk::TextBuffer,
    pub recipe_ingredients_text_buffer: gtk::TextBuffer,
    pub recipe_text_buffer: gtk::TextBuffer,
}

impl MainWindowText {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let recipe_name_text_buffer: gtk::TextBuffer = builder
            .get_object("recipe_name_text_buffer")
            .expect("\"recipe_name_text_buffer\" ID in \"Main_Window.glade\" should exist.");
        let recipe_ingredients_text_buffer: gtk::TextBuffer = builder
            .get_object("recipe_ingredients_text_buffer")
            .expect("\"recipe_ingredients_text_buffer\" ID in \"Main_Window.glade\" should exist.");
        let recipe_text_buffer: gtk::TextBuffer = builder
            .get_object("recipe_text_buffer")
            .expect("\"recipe_text_buffer\" ID in \"Main_Window.glade\" should exist.");

        Self {
            recipe_name_text_buffer,
            recipe_ingredients_text_buffer,
            recipe_text_buffer,
        }
    }
}

#[test]
fn create_from_builder_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let glade_src = include_str!("../../../ui/Cookbook.glade").to_string();
    let builder = gtk::Builder::from_string(glade_src.as_str());

    let recipe_name_text_buffer: gtk::TextBuffer = builder
        .get_object("recipe_name_text_buffer")
        .expect("\"recipe_name_text_buffer\" ID in \"Main_Window.glade\" should exist.");
    let recipe_ingredients_text_buffer: gtk::TextBuffer = builder
        .get_object("recipe_ingredients_text_buffer")
        .expect("\"recipe_ingredients_text_buffer\" ID in \"Main_Window.glade\" should exist.");
    let recipe_text_buffer: gtk::TextBuffer = builder
        .get_object("recipe_text_buffer")
        .expect("\"recipe_text_buffer\" ID in \"Main_Window.glade\" should exist.");

    let should_be = MainWindowText {
        recipe_name_text_buffer,
        recipe_ingredients_text_buffer,
        recipe_text_buffer,
    };

    let tested = MainWindowText::create_from_builder(&builder);

    assert_eq!(tested, should_be);
}
