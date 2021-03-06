#![windows_subsystem = "windows"]

use gtk::prelude::*;

use gui_data::connections::*;

mod field_set;
mod gui;
mod gui_data;
mod user;

fn main() {
    gtk::init().expect("Failed to initialize GTK...");
    let gui_data = gui_data::GuiData::new();

    gui::initialize(&gui_data);
    buttons::connect(&gui_data);
    tree_selections::connect(&gui_data);
    others::connect_search_bar(&gui_data);
    others::connect_document_id_buffer(&gui_data);

    {
        let window_main = gui_data.main_window;
        window_main.show_all();
        window_main.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }
    gtk::main();
}
