use gtk::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct MainWindowCategoryNotebook {
    pub meal_category_tree_store: gtk::TreeStore,
}

impl MainWindowCategoryNotebook {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let meal_category_tree_store: gtk::TreeStore = builder
            .get_object("meal_category_tree_store")
            .expect("\"meal_category_tree_store\" ID in \"Main_Window.glade\" should exist.");

        for i in 0..10 {
            let iter = meal_category_tree_store.insert_with_values(
                None,
                None,
                &[0],
                &[&format!("Hello {}", i)],
            );

            for _ in 0..i {
                meal_category_tree_store.insert_with_values(
                    Some(&iter),
                    None,
                    &[0],
                    &[&format!("I'm a child node")],
                );
            }
        }

        Self {
            meal_category_tree_store,
        }
    }
}

#[test]
fn create_from_builder_test() {
    gtk::init().expect("Failed to initialize GTK...");
    let glade_src = include_str!("../../../ui/Cookbook.glade").to_string();
    let builder = gtk::Builder::from_string(glade_src.as_str());

    let meal_category_tree_store: gtk::TreeStore = builder
        .get_object("meal_category_tree_store")
        .expect("\"meal_category_tree_store\" ID in \"Main_Window.glade\" should exist.");

    let should_be = MainWindowCategoryNotebook {
        meal_category_tree_store,
    };

    let tested = MainWindowCategoryNotebook::create_from_builder(&builder);
    assert_eq!(tested, should_be);
}
