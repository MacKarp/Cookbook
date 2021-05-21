use gtk::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct MainWindowCategoryNotebook {
    pub meal_category_tree_store: gtk::TreeStore,
    pub meal_category_tree_view: gtk::TreeView,
    pub meal_category_tree_selection: gtk::TreeSelection,

    pub drink_category_tree_store: gtk::TreeStore,
    pub drink_category_tree_view: gtk::TreeView,
    pub drink_category_tree_selection: gtk::TreeSelection,

    pub favorite_tree_store: gtk::TreeStore,
    pub favorite_tree_view: gtk::TreeView,
    pub favorite_tree_selection: gtk::TreeSelection,
}

impl MainWindowCategoryNotebook {
    pub fn create_from_builder(builder: &gtk::Builder) -> Self {
        let meal_category_tree_store: gtk::TreeStore = builder
            .get_object("meal_category_tree_store")
            .expect("\"meal_category_tree_store\" ID in \"Main_Window.glade\" should exist.");
        let meal_category_tree_view: gtk::TreeView = builder
            .get_object("meal_category_tree_view")
            .expect("\"meal_category_tree_view\" ID in \"Main_Window.glade\" should exist.");
        let meal_category_tree_selection: gtk::TreeSelection = builder
            .get_object("meal_category_tree_selection")
            .expect("\"meal_category_tree_selection\" ID in \"Main_Window.glade\" should exist.");

        let drink_category_tree_store: gtk::TreeStore = builder
            .get_object("drink_category_tree_store")
            .expect("\"drink_category_tree_store\" ID in \"Main_Window.glade\" should exist.");
        let drink_category_tree_view: gtk::TreeView = builder
            .get_object("drink_category_tree_view")
            .expect("\"drink_category_tree_view\" ID in \"Main_Window.glade\" should exist.");
        let drink_category_tree_selection: gtk::TreeSelection = builder
            .get_object("drink_category_tree_selection")
            .expect("\"drink_category_tree_selection\" ID in \"Main_Window.glade\" should exist.");

        let favorite_tree_store: gtk::TreeStore = builder
            .get_object("favorite_tree_store")
            .expect("\"favorite_tree_store\" ID in \"Main_Window.glade\" should exist.");
        let favorite_tree_view: gtk::TreeView = builder
            .get_object("favorite_tree_view")
            .expect("\"favorite_tree_view\" ID in \"Main_Window.glade\" should exist.");
        let favorite_tree_selection: gtk::TreeSelection = builder
            .get_object("favorite_tree_selection")
            .expect("\"favorite_tree_selection\" ID in \"Main_Window.glade\" should exist.");

        Self {
            meal_category_tree_store,
            meal_category_tree_view,
            meal_category_tree_selection,

            drink_category_tree_store,
            drink_category_tree_view,
            drink_category_tree_selection,

            favorite_tree_store,
            favorite_tree_view,
            favorite_tree_selection,
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
    let meal_category_tree_view: gtk::TreeView = builder
        .get_object("meal_category_tree_view")
        .expect("\"meal_category_tree_view\" ID in \"Main_Window.glade\" should exist.");
    let drink_category_tree_store: gtk::TreeStore = builder
        .get_object("drink_category_tree_store")
        .expect("\"drink_category_tree_store\" ID in \"Main_Window.glade\" should exist.");
    let drink_category_tree_view: gtk::TreeView = builder
        .get_object("drink_category_tree_view")
        .expect("\"drink_category_tree_view\" ID in \"Main_Window.glade\" should exist.");
    let meal_category_tree_selection: gtk::TreeSelection = builder
        .get_object("meal_category_tree_selection")
        .expect("\"meal_category_tree_selection\" ID in \"Main_Window.glade\" should exist.");
    let drink_category_tree_selection: gtk::TreeSelection = builder
        .get_object("drink_category_tree_selection")
        .expect("\"drink_category_tree_selection\" ID in \"Main_Window.glade\" should exist.");

    let favorite_tree_store: gtk::TreeStore = builder
        .get_object("favorite_tree_store")
        .expect("\"favorite_tree_store\" ID in \"Main_Window.glade\" should exist.");
    let favorite_tree_view: gtk::TreeView = builder
        .get_object("favorite_tree_view")
        .expect("\"favorite_tree_view\" ID in \"Main_Window.glade\" should exist.");
    let favorite_tree_selection: gtk::TreeSelection = builder
        .get_object("favorite_tree_selection")
        .expect("\"favorite_tree_selection\" ID in \"Main_Window.glade\" should exist.");

    let should_be = MainWindowCategoryNotebook {
        meal_category_tree_store,
        meal_category_tree_view,
        meal_category_tree_selection,

        drink_category_tree_store,
        drink_category_tree_view,
        drink_category_tree_selection,

        favorite_tree_store,
        favorite_tree_view,
        favorite_tree_selection,
    };

    let tested = MainWindowCategoryNotebook::create_from_builder(&builder);
    assert_eq!(tested, should_be);
}
