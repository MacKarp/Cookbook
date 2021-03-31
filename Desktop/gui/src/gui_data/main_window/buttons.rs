use gtk::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct MainWindowButtons {
    pub random_meal_recipe_button: gtk::Button,
}

impl MainWindowButtons {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let random_meal_recipe_button: gtk::Button = builder
            .get_object("random_meal_recipe_button")
            .expect("\"random_meal_recipe_button\" ID in \"Main_Window.glade\" should exist.");
        Self {
            random_meal_recipe_button,
        }
    }
}

#[test]
fn create_from_builder_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let glade_src = include_str!("../../../ui/Cookbook.glade").to_string();
    let builder = gtk::Builder::from_string(glade_src.as_str());

    let random_meal_recipe_button: gtk::Button = builder
        .get_object("random_meal_recipe_button")
        .expect("\"random_meal_recipe_button\" ID in \"Main_Window.glade\" should exist.");

    let should_be = MainWindowButtons {
        random_meal_recipe_button,
    };

    let tested = MainWindowButtons::create_from_builder(&builder);
    assert_eq!(tested, should_be);
}
