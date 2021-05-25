use gtk::prelude::*;
#[derive(Clone, PartialEq, Debug)]
pub struct MainWindowText {
    pub recipe_name_text_label: gtk::Label,
    pub recipe_ingredients_label: gtk::Label,
    pub recipe_text_label: gtk::Label,
    pub displayed_recipe_type_text_buffer: gtk::TextBuffer,
    pub displayed_recipe_id_text_buffer: gtk::TextBuffer,
    pub displayed_recipe_favorite_document_id_text_buffer: gtk::TextBuffer,
}

impl MainWindowText {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let recipe_name_text_label: gtk::Label = builder
            .get_object("recipe_name_text_label")
            .expect("\"recipe_name_text_label\" ID in \"Main_Window.glade\" should exist.");
        let recipe_ingredients_label: gtk::Label = builder
            .get_object("recipe_ingredients_label")
            .expect("\"recipe_ingredients_label\" ID in \"Main_Window.glade\" should exist.");
        let recipe_text_label: gtk::Label = builder
            .get_object("recipe_text_label")
            .expect("\"recipe_text_label\" ID in \"Main_Window.glade\" should exist.");
        let displayed_recipe_type_text_buffer: gtk::TextBuffer = builder
            .get_object("displayed_recipe_type_text_buffer")
            .expect(
                "\"displayed_recipe_type_text_buffer\" ID in \"Main_Window.glade\" should exist.",
            );
        let displayed_recipe_id_text_buffer: gtk::TextBuffer = builder
            .get_object("displayed_recipe_id_text_buffer")
            .expect(
                "\"displayed_recipe_id_text_buffer\" ID in \"Main_Window.glade\" should exist.",
            );
        let displayed_recipe_favorite_document_id_text_buffer: gtk::TextBuffer = builder
            .get_object("displayed_recipe_favorite_document_id_text_buffer")
            .expect(
                "\"displayed_recipe_favorite_document_id_text_buffer\" ID in \"Main_Window.glade\" should exist.",
            );

        Self {
            recipe_name_text_label,
            recipe_ingredients_label,
            recipe_text_label,
            displayed_recipe_type_text_buffer,
            displayed_recipe_id_text_buffer,
            displayed_recipe_favorite_document_id_text_buffer,
        }
    }
}

#[test]
fn create_from_builder_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let glade_src = include_str!("../../../ui/Cookbook.glade").to_string();
    let builder = gtk::Builder::from_string(glade_src.as_str());

    let recipe_name_text_label: gtk::Label = builder
        .get_object("recipe_name_text_label")
        .expect("\"recipe_name_text_label\" ID in \"Main_Window.glade\" should exist.");
    let recipe_ingredients_label: gtk::Label = builder
        .get_object("recipe_ingredients_label")
        .expect("\"recipe_ingredients_label\" ID in \"Main_Window.glade\" should exist.");
    let recipe_text_label: gtk::Label = builder
        .get_object("recipe_text_label")
        .expect("\"recipe_text_label\" ID in \"Main_Window.glade\" should exist.");
    let displayed_recipe_type_text_buffer: gtk::TextBuffer = builder
        .get_object("displayed_recipe_type_text_buffer")
        .expect("\"displayed_recipe_type_text_buffer\" ID in \"Main_Window.glade\" should exist.");
    let displayed_recipe_id_text_buffer: gtk::TextBuffer = builder
        .get_object("displayed_recipe_id_text_buffer")
        .expect("\"displayed_recipe_id_text_buffer\" ID in \"Main_Window.glade\" should exist.");
    let displayed_recipe_favorite_document_id_text_buffer: gtk::TextBuffer = builder
        .get_object("displayed_recipe_favorite_document_id_text_buffer")
        .expect(
            "\"displayed_recipe_favorite_document_id_text_buffer\" ID in \"Main_Window.glade\" should exist.",
        );

    let should_be = MainWindowText {
        recipe_ingredients_label,
        recipe_ingredients_label,
        recipe_text_label,
        displayed_recipe_type_text_buffer,
        displayed_recipe_id_text_buffer,
        displayed_recipe_favorite_document_id_text_buffer,
    };

    let tested = MainWindowText::create_from_builder(&builder);

    assert_eq!(tested, should_be);
}
