use gtk::prelude::*;

mod gui_data;

fn main() {
    gtk::init().expect("Failed to initialize GTK...");
    let gui_data = gui_data::GuiData::new();
    {
        let window_main = gui_data.main_window.clone();
        window_main.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }
    gtk::main();
}
