use crate::gui_data::{
    connections::buttons::{random_drink, random_meal},
    GuiData,
};
use cookbook::data::drink::{
    alcoholic::get_alcoholic_list, categories::get_drink_category_list, glass::get_glass_list,
    ingredient::get_drink_ingredient_list,
};
use cookbook::data::meal::ingredient::get_meal_ingredient_list;
use cookbook::data::meal::{area::get_area_category_list, categories::get_meal_category_list};

use gtk::prelude::*;
use rand::prelude::*;

pub fn initialize(gui_data: &GuiData) {
    initialize_meal_category_tab(&gui_data);
    initialize_drink_category_tab(&gui_data);
    initialize_stack(&gui_data);
    initialize_buttons(&gui_data);
}

fn initialize_buttons(gui_data: &GuiData) -> () {
    let previous_stack_button = gui_data.main_window_buttons.previous_stack_button.clone();
    previous_stack_button.set_sensitive(false);
}

fn initialize_stack(gui_data: &GuiData) -> () {
    let mut rng = rand::thread_rng();
    let rand_recipe: u8 = rng.gen_range(0..=1);
    match rand_recipe {
        0 => random_drink::on_random_recipe_button_clicked(&gui_data),
        1 => random_meal::on_random_recipe_button_clicked(&gui_data),
        _ => (),
    }
}

fn initialize_drink_category_tab(gui_data: &GuiData) -> () {
    let drink_category_tree_store = gui_data
        .main_window_category_notebook
        .drink_category_tree_store
        .clone();

    let drink_category_iter =
        drink_category_tree_store.insert_with_values(None, None, &[0], &[&format!("Categories")]);
    let drink_category = get_drink_category_list();
    let drink_category = drink_category.categories;
    for s in drink_category {
        drink_category_tree_store.insert_with_values(Some(&drink_category_iter), None, &[0], &[&s]);
    }

    let drink_glass_iter =
        drink_category_tree_store.insert_with_values(None, None, &[0], &[&format!("Glass")]);
    let drink_glass = get_glass_list();
    let drink_glass = drink_glass.categories;
    for s in drink_glass {
        drink_category_tree_store.insert_with_values(Some(&drink_glass_iter), None, &[0], &[&s]);
    }

    let drink_alcoholic_iter =
        drink_category_tree_store.insert_with_values(None, None, &[0], &[&format!("Alcoholic")]);
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
    let drink_ingredient_iter =
        drink_category_tree_store.insert_with_values(None, None, &[0], &[&format!("Ingredients")]);
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

fn initialize_meal_category_tab(gui_data: &GuiData) -> () {
    let meal_category_tree_store = gui_data
        .main_window_category_notebook
        .meal_category_tree_store
        .clone();

    let meal_category_iter =
        meal_category_tree_store.insert_with_values(None, None, &[0], &[&format!("Categories")]);
    let meal_category = get_meal_category_list();
    let meal_category = meal_category.categories;
    for s in meal_category {
        meal_category_tree_store.insert_with_values(Some(&meal_category_iter), None, &[0], &[&s]);
    }

    let meal_area_iter =
        meal_category_tree_store.insert_with_values(None, None, &[0], &[&format!("Area")]);
    let meal_area = get_area_category_list();
    let meal_area = meal_area.categories;
    for s in meal_area {
        meal_category_tree_store.insert_with_values(Some(&meal_area_iter), None, &[0], &[&s]);
    }

    let meal_ingredient_iter =
        meal_category_tree_store.insert_with_values(None, None, &[0], &[&format!("Ingredients")]);
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
