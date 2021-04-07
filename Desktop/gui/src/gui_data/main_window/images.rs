use gtk::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct MainWindowImages {
    pub image_recipe: gtk::Image,
}

impl MainWindowImages {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let image_recipe: gtk::Image = builder
            .get_object("image_recipe")
            .expect("\"image_recipe\" ID in \"Main_Window.glade\" should exist.");

        Self { image_recipe }
    }
}

#[test]
fn create_from_builder_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let glade_src = include_str!("../../../ui/Cookbook.glade").to_string();
    let builder = gtk::Builder::from_string(glade_src.as_str());

    let image_recipe: gtk::Image = builder
        .get_object("image_recipe")
        .expect("\"image_recipe\" ID in \"Main_Window.glade\" should exist.");

    let should_be = MainWindowImages { image_recipe };

    let tested = MainWindowImages::create_from_builder(&builder);
    assert_eq!(tested, should_be);
}
