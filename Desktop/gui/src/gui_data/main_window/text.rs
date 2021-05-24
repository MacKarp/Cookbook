use gtk::prelude::*;
#[derive(Clone, PartialEq, Debug)]
pub struct MainWindowText {
    pub recipe_name_text_label: gtk::Label,
    pub recipe_ingredients_label: gtk::Label,
    pub recipe_text_label: gtk::Label,
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

        Self {
            recipe_name_text_label,
            recipe_ingredients_label: recipe_ingredients_label,
            recipe_text_label: recipe_text_label,
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

    let should_be = MainWindowText {
        recipe_name_text_label: recipe_ingredients_label,
        recipe_ingredients_label,
        recipe_text_label: recipe_text_label,
    };

    let tested = MainWindowText::create_from_builder(&builder);

    assert_eq!(tested, should_be);
}
