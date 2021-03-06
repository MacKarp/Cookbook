use gdk_pixbuf::{InterpType, Pixbuf};
use glib::timeout_add_local;
use gtk::prelude::*;
use rand::prelude::*;

use cookbook::data::drink::{
    alcoholic::get_alcoholic_list, categories::get_drink_category_list, glass::get_glass_list,
    ingredient::get_drink_ingredient_list,
};
use cookbook::data::meal::{
    area::get_area_category_list, categories::get_meal_category_list,
    ingredient::get_meal_ingredient_list,
};
use firebase_handler::favorites::get_favorites;

use crate::gui_data::{
    connections::buttons::{random_drink, random_meal},
    GuiData,
};
use crate::user::get_user_image;

pub fn initialize(gui_data: &GuiData) {
    initialize_logo(&gui_data);
    initialize_meal_category_tab(&gui_data);
    initialize_drink_category_tab(&gui_data);
    initialize_stack(&gui_data);
    initialize_buttons(&gui_data);
    initialize_user(&gui_data);
    favorites_update(&gui_data);

    let favorites_gui_data = gui_data.clone();
    timeout_add_local(15000, move || {
        favorites_update(&favorites_gui_data);
        Continue(true)
    });

    let user_gui_data = gui_data.clone();
    timeout_add_local(1000, move || {
        refresh_initialize_user(&user_gui_data);
        Continue(true)
    });
}

fn refresh_initialize_user(gui_data: &GuiData) {
    let login_button = gui_data.main_window_buttons.login_button.clone();
    let label = login_button.get_label().unwrap().to_string();
    if label == "Login" {
        initialize_user(&gui_data);
    }
}

fn initialize_logo(gui_data: &GuiData) {
    let logo_image = gui_data.main_window_logo_image.clone();
    let pixbuf = Pixbuf::from_file("gui/ui/logo.png").unwrap();
    let pixbuf = pixbuf.scale_simple(135, 84, InterpType::Bilinear).unwrap();
    logo_image.set_from_pixbuf(Some(&pixbuf));
}

pub fn initialize_user(gui_data: &GuiData) {
    let users = firebase_handler::get_user_info();
    let login_button = gui_data.main_window_buttons.login_button.clone();
    let user_image = gui_data.main_window_images.user_image.clone();
    let welcome_label = gui_data.main_window_welcome_label.clone();
    let favorite_button = gui_data.main_window_buttons.favorite_button.clone();
    match users {
        Ok(u) => {
            let user_name = u.users[0]
                .displayName
                .clone()
                .unwrap_or_else(|| String::from("Guest"));
            let user_welcome = "Welcome ".to_owned() + &user_name;
            let user_welcome = user_welcome + "!";
            welcome_label.set_text(&user_welcome);

            let user_img = u.users[0].photoUrl.clone();
            user_image.set_from_pixbuf(Some(&get_user_image(user_img)));
            login_button.set_label("Logout");
            favorite_button.set_sensitive(true);
        }
        Err(_) => {
            welcome_label.set_text("Welcome Geust!");
            user_image.set_from_pixbuf(Some(&get_user_image(None)));
            login_button.set_label("Login");
            favorite_button.set_sensitive(false);
        }
    };
    set_favorite_button_image(&gui_data);
}

fn initialize_buttons(gui_data: &GuiData) {
    let previous_stack_button = gui_data.main_window_buttons.previous_stack_button.clone();
    previous_stack_button.set_sensitive(false);
    let favorite_button = gui_data.main_window_buttons.favorite_button.clone();
    favorite_button.set_label("Add to favorites");
    favorite_button.set_sensitive(false);

    let favorite_image = gui_data.main_window_favorite_button_image.clone();
    if let Some(p) = set_favorite_button_image(&gui_data) {
        favorite_image.set_from_pixbuf(Some(&p))
    }
}

pub fn set_favorite_button_image(gui_data: &GuiData) -> Option<Pixbuf> {
    let button = gui_data.main_window_buttons.favorite_button.clone();
    let button_label = button.get_label().unwrap();
    let button_label = button_label.as_str();
    match button_label {
        "Add to favorites" => {
            let pix = Pixbuf::from_file("gui/ui/add_favorite.png").unwrap();
            Some(pix)
        }
        "Remove from favorites" => {
            let pix = Pixbuf::from_file("gui/ui/remove_favorite.png").unwrap();
            Some(pix)
        }
        _ => None,
    }
}

fn initialize_stack(gui_data: &GuiData) {
    let mut rng = rand::thread_rng();
    let rand_recipe: u8 = rng.gen_range(0..=1);
    match rand_recipe {
        0 => random_drink::on_random_recipe_button_clicked(&gui_data),
        1 => random_meal::on_random_recipe_button_clicked(&gui_data),
        _ => (),
    }
}

fn initialize_drink_category_tab(gui_data: &GuiData) {
    let drink_category_tree_store = gui_data
        .main_window_category_notebook
        .drink_category_tree_store
        .clone();

    let drink_category_iter = drink_category_tree_store.insert_with_values(
        None,
        None,
        &[0],
        &[&"Categories".to_string()],
    );
    let drink_category = get_drink_category_list();
    let drink_category = drink_category.categories;
    for s in drink_category {
        drink_category_tree_store.insert_with_values(Some(&drink_category_iter), None, &[0], &[&s]);
    }

    let drink_glass_iter =
        drink_category_tree_store.insert_with_values(None, None, &[0], &[&"Glass".to_string()]);
    let drink_glass = get_glass_list();
    let drink_glass = drink_glass.categories;
    for s in drink_glass {
        drink_category_tree_store.insert_with_values(Some(&drink_glass_iter), None, &[0], &[&s]);
    }

    let drink_alcoholic_iter =
        drink_category_tree_store.insert_with_values(None, None, &[0], &[&"Alcoholic".to_string()]);
    let drink_alcoholic = get_alcoholic_list();
    let drink_alcoholic = drink_alcoholic.categories;
    for s in drink_alcoholic {
        drink_category_tree_store.insert_with_values(
            Some(&drink_alcoholic_iter),
            None,
            &[0],
            &[&s],
        );
    }
    let drink_ingredient_iter = drink_category_tree_store.insert_with_values(
        None,
        None,
        &[0],
        &[&"Ingredients".to_string()],
    );
    let drink_ingredient = get_drink_ingredient_list();
    let drink_ingredient = drink_ingredient.categories;
    for s in drink_ingredient {
        drink_category_tree_store.insert_with_values(
            Some(&drink_ingredient_iter),
            None,
            &[0],
            &[&s],
        );
    }

    let drink_category_tree_view = gui_data
        .main_window_category_notebook
        .drink_category_tree_view
        .clone();

    drink_category_tree_view.expand_all();
}

fn initialize_meal_category_tab(gui_data: &GuiData) {
    let meal_category_tree_store = gui_data
        .main_window_category_notebook
        .meal_category_tree_store
        .clone();

    let meal_category_iter =
        meal_category_tree_store.insert_with_values(None, None, &[0], &[&"Categories".to_string()]);
    let meal_category = get_meal_category_list();
    let meal_category = meal_category.categories;
    for s in meal_category {
        meal_category_tree_store.insert_with_values(Some(&meal_category_iter), None, &[0], &[&s]);
    }

    let meal_area_iter =
        meal_category_tree_store.insert_with_values(None, None, &[0], &[&"Area".to_string()]);
    let meal_area = get_area_category_list();
    let meal_area = meal_area.categories;
    for s in meal_area {
        meal_category_tree_store.insert_with_values(Some(&meal_area_iter), None, &[0], &[&s]);
    }

    let meal_ingredient_iter = meal_category_tree_store.insert_with_values(
        None,
        None,
        &[0],
        &[&"Ingredients".to_string()],
    );
    let meal_ingredient = get_meal_ingredient_list();
    let meal_ingredient = meal_ingredient.categories;
    for s in meal_ingredient {
        meal_category_tree_store.insert_with_values(Some(&meal_ingredient_iter), None, &[0], &[&s]);
    }

    let meal_category_tree_view = gui_data
        .main_window_category_notebook
        .meal_category_tree_view
        .clone();

    meal_category_tree_view.expand_all();
}

pub fn favorites_update(gui_data: &GuiData) {
    let tree_store = gui_data
        .main_window_category_notebook
        .favorite_tree_store
        .clone();

    let (favorites_meal, favorites_drinks) = get_favorites();
    let mut favorites_meal: Vec<_> = favorites_meal.iter().collect();
    favorites_meal.sort_by_key(|a| a.0);

    let mut favorites_drinks: Vec<_> = favorites_drinks.iter().collect();
    favorites_drinks.sort_by_key(|a| a.0);

    let tree_selection = gui_data
        .main_window_category_notebook
        .favorite_tree_selection
        .clone();
    tree_selection.set_mode(gtk::SelectionMode::None);
    tree_store.clear();

    for (document_id, meal) in favorites_meal {
        let name = meal.meal_name.as_ref().unwrap();
        let id = meal.meal_id.as_ref().unwrap().parse::<i32>().unwrap();
        let category = "Meal";
        let doc_id = document_id;
        tree_store.insert_with_values(None, None, &[0, 1, 2, 3], &[&name, &id, &category, &doc_id]);
    }
    for (document_id, drink) in favorites_drinks {
        let name = drink.drink_name.as_ref().unwrap();
        let id = drink.drink_id.as_ref().unwrap().parse::<i32>().unwrap();
        let category = "Drink";
        let doc_id = document_id;
        tree_store.insert_with_values(None, None, &[0, 1, 2, 3], &[&name, &id, &category, &doc_id]);
    }
    let tree_view = gui_data
        .main_window_category_notebook
        .favorite_tree_view
        .clone();

    tree_selection.set_mode(gtk::SelectionMode::Single);
    tree_view.expand_all();
}

#[test]
fn initialize_logo_test() {
    gtk::init().expect("Failed to initialize GTK...");

    let should_be = Pixbuf::from_file("gui/ui/logo.png").unwrap();
    let should_be = should_be
        .scale_simple(135, 84, InterpType::Bilinear)
        .unwrap();
    let should_be = unsafe { should_be.get_pixels() };

    let gui_data = GuiData::new();
    initialize_logo(&gui_data);
    let tested = gui_data.main_window_logo_image.get_pixbuf().unwrap();
    let tested = unsafe { tested.get_pixels() };

    assert_eq!(should_be, tested);
}

#[test]
fn initialize_meal_category_tab_test() {
   gtk::init().expect("Failed to initialize GTK...");
    let gui_data = GuiData::new();
    initialize_meal_category_tab(&gui_data);

    let meal_category_tree_store = gui_data
        .main_window_category_notebook
        .meal_category_tree_store;
    let iter = meal_category_tree_store.get_iter_first().unwrap();
    let category_tested = meal_category_tree_store.get_value(&iter, 0);
    let category_tested = category_tested.get::<String>().unwrap().unwrap();
    assert_eq!("Categories", category_tested);

    meal_category_tree_store.iter_next(&iter);
    let category_tested = meal_category_tree_store.get_value(&iter, 0);
    let category_tested = category_tested.get::<String>().unwrap().unwrap();
    assert_eq!("Area", category_tested);

    meal_category_tree_store.iter_next(&iter);
    let category_tested = meal_category_tree_store.get_value(&iter, 0);
    let category_tested = category_tested.get::<String>().unwrap().unwrap();
    assert_eq!("Ingredients", category_tested);
}

#[test]
fn initialize_drink_category_tab_test() {
    std::env::set_current_dir(std::path::Path::new("../")).unwrap();
    gtk::init().expect("Failed to initialize GTK...");
    let gui_data = GuiData::new();
    initialize_drink_category_tab(&gui_data);

    let drink_category_tree_store = gui_data
        .main_window_category_notebook
        .drink_category_tree_store;
    let iter = drink_category_tree_store.get_iter_first().unwrap();
    let category_tested = drink_category_tree_store.get_value(&iter, 0);
    let category_tested = category_tested.get::<String>().unwrap().unwrap();
    assert_eq!("Categories", category_tested);

    drink_category_tree_store.iter_next(&iter);
    let category_tested = drink_category_tree_store.get_value(&iter, 0);
    let category_tested = category_tested.get::<String>().unwrap().unwrap();
    assert_eq!("Glass", category_tested);

    drink_category_tree_store.iter_next(&iter);
    let category_tested = drink_category_tree_store.get_value(&iter, 0);
    let category_tested = category_tested.get::<String>().unwrap().unwrap();
    assert_eq!("Alcoholic", category_tested);

    drink_category_tree_store.iter_next(&iter);
    let category_tested = drink_category_tree_store.get_value(&iter, 0);
    let category_tested = category_tested.get::<String>().unwrap().unwrap();
    assert_eq!("Ingredients", category_tested);
}
