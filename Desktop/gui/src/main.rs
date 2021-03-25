use gio::prelude::*;
use glib;
use gtk::prelude::*;
use std::env::args;
mod main_window;
fn main() {
    let application =
        gtk::Application::new(Some("com.github.mackarp.cookbook"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("../ui/Main_Window.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let main_window = main_window::MainWindow::new(builder.clone());
    main_window.window.set_application(Some(application));
    main_window
        .recipe_name_text_buffer
        .set_text(get_recipe_name());
    main_window
        .recipe_ingredients_text_buffer
        .set_text(get_recipe_ingredients());
    main_window.recipe_text_buffer.set_text(get_recipe_text());

    let random_button = main_window.button;
    builder.connect_signals(move |_, handler_name| match handler_name {
        "on_random_recipe_button_clicked" => Box::new(
            glib::clone!(@weak random_button => @default-return None, move |_| {
              on_random_recipe_button_clicked();
              None}),
        ),
        _ => Box::new(|_| None),
    });

    main_window.window.show_all();
}

fn get_recipe_name() -> &'static str {
    "Home-made Mandazi"
}
fn get_recipe_ingredients() -> &'static str {
    "750g Self-raising Flour
        6 tablespoons Sugar
        2 Eggs
        1 cup Milk"
}

fn get_recipe_text() -> &'static str {
    "This is one recipe a lot of people have requested and I have tried to make it as simple as possible and I hope it will work for you. Make sure you use the right flour which is basically one with raising agents. Adjust the amount of sugar to your taste and try using different flavours to have variety whenever you have them. You can use Coconut milk instead of regular milk, you can also add desiccated coconut to the dry flour or other spices like powdered cloves or cinnamon. For “healthy looking” mandazis do not roll the dough too thin before frying and use the procedure I have indicated above.\n1. Mix the flour,cinnamon and sugar in a suitable bowl.\n2. In a separate bowl whisk the egg into the milk\n3. Make a well at the centre of the flour and add the milk and egg mixture and slowly mix to form a dough.\n4. Knead the dough for 3-4 minutes or until it stops sticking to the sides of the bowl and you have a smooth surface.\n5. Cover the dough with a damp cloth  and allow to rest for 15 minutes.\n6. Roll the dough on a lightly floured surface into a 1cm thick piece.\n7. Using a sharp small knife, cut the dough into the desired size setting aside ready for deep frying.\n8. Heat your oil in a suitable pot and gently dip the mandazi pieces to cook until light brown on the first side then turn to cook on the second side.\n9. Serve them warm or cold"
}

fn on_random_recipe_button_clicked() {
    println!("Random recipe button clicked");
}

#[cfg(test)]
mod main_window_tests {
    #[test]
    fn get_recipe_name_test() {
        assert_eq!("Home-made Mandazi", crate::get_recipe_name());
    }

    #[test]
    fn get_recipe_ingredients_test() {
        assert_eq!(
            "750g Self-raising Flour
        6 tablespoons Sugar
        2 Eggs
        1 cup Milk",
            crate::get_recipe_ingredients()
        );
    }

    #[test]
    fn get_recipe_text_test() {
        assert_eq!("This is one recipe a lot of people have requested and I have tried to make it as simple as possible and I hope it will work for you. Make sure you use the right flour which is basically one with raising agents. Adjust the amount of sugar to your taste and try using different flavours to have variety whenever you have them. You can use Coconut milk instead of regular milk, you can also add desiccated coconut to the dry flour or other spices like powdered cloves or cinnamon. For “healthy looking” mandazis do not roll the dough too thin before frying and use the procedure I have indicated above.\n1. Mix the flour,cinnamon and sugar in a suitable bowl.\n2. In a separate bowl whisk the egg into the milk\n3. Make a well at the centre of the flour and add the milk and egg mixture and slowly mix to form a dough.\n4. Knead the dough for 3-4 minutes or until it stops sticking to the sides of the bowl and you have a smooth surface.\n5. Cover the dough with a damp cloth  and allow to rest for 15 minutes.\n6. Roll the dough on a lightly floured surface into a 1cm thick piece.\n7. Using a sharp small knife, cut the dough into the desired size setting aside ready for deep frying.\n8. Heat your oil in a suitable pot and gently dip the mandazi pieces to cook until light brown on the first side then turn to cook on the second side.\n9. Serve them warm or cold"
        , crate::get_recipe_text());
    }

    #[test]
    fn on_random_recipe_button_clicked_test() {
        assert_eq!((), crate::on_random_recipe_button_clicked());
    }
}
