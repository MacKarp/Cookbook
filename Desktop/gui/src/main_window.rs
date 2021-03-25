use gtk::prelude::*;
#[derive(Debug, PartialEq)]
pub struct MainWindow {
    pub window: gtk::Window,
    pub recipe_name_text_buffer: gtk::TextBuffer,
    pub recipe_ingredients_text_buffer: gtk::TextBuffer,
    pub recipe_text_buffer: gtk::TextBuffer,
    pub button: gtk::Button,
}
impl MainWindow {
    pub fn new(builder: gtk::Builder) -> Self {
        MainWindow {
            window: builder
                .get_object("main_window")
                .expect("\"main_window\" ID in \"Main_Window.glade\" should exist."),

            recipe_name_text_buffer: builder
                .get_object("recipe_name_text_buffer")
                .expect("\"recipe_name_text_buffer\" ID in \"Main_Window.glade\" should exist."),

            recipe_ingredients_text_buffer: builder
                .get_object("recipe_ingredients_text_buffer")
                .expect(
                    "\"recipe_ingredients_text_buffer\" ID in \"Main_Window.glade\" should exist.",
                ),

            recipe_text_buffer: builder
                .get_object("recipe_text_buffer")
                .expect("\"recipe_text_buffer\" ID in \"Main_Window.glade\" should exist."),

            button: builder
                .get_object("random_recipe_button")
                .expect("\"random_recipe_button\" ID in \"Main_Window.glade\" should exist."),
        }
    }
}

#[cfg(test)]
mod main_window_struct_tests {
    use gtk::prelude::*;

    #[test]
    fn new_main_window_test() {
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        }

        let glade_src = include_str!("../ui/Main_Window.glade");
        let builder = gtk::Builder::from_string(glade_src);

        let main_window: gtk::Window = builder
            .get_object("main_window")
            .expect("\"main_window\" ID in \"Main_Window.glade\" should exist.");

        let recipe_name_text_buffer: gtk::TextBuffer = builder
            .get_object("recipe_name_text_buffer")
            .expect("\"recipe_name_text_buffer\" ID in \"Main_Window.glade\" should exist.");

        let recipe_ingredients_text_buffer: gtk::TextBuffer = builder
            .get_object("recipe_ingredients_text_buffer")
            .expect("\"recipe_ingredients_text_buffer\" ID in \"Main_Window.glade\" should exist.");

        let recipe_text_buffer: gtk::TextBuffer = builder
            .get_object("recipe_text_buffer")
            .expect("\"recipe_text_buffer\" ID in \"Main_Window.glade\" should exist.");

        let button: gtk::Button = builder
            .get_object("random_recipe_button")
            .expect("\"random_recipe_button\" ID in \"Main_Window.glade\" should exist.");

        let main_window_struct = crate::main_window::MainWindow {
            window: main_window,
            recipe_name_text_buffer: recipe_name_text_buffer,
            recipe_ingredients_text_buffer: recipe_ingredients_text_buffer,
            recipe_text_buffer: recipe_text_buffer,
            button: button,
        };

        let test_main_window = crate::main_window::MainWindow::new(builder.clone());
        assert_eq!(main_window_struct, test_main_window);
    }
}
