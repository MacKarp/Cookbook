use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;

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

    let window: gtk::Window = builder.get_object("MainWindow").unwrap();
    window.set_application(Some(application));

    window.show_all();
}
