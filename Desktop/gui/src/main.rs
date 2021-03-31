mod gui_data;

use gtk::prelude::*;
use gui_data::connections::random_meal::connect_random_recipe_button;

fn main() {
    gtk::init().expect("Failed to initialize GTK...");
    let gui_data = gui_data::GuiData::new();

    connect_random_recipe_button(&gui_data);

    {
        let window_main = gui_data.main_window;
        window_main.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }
    gtk::main();
}
