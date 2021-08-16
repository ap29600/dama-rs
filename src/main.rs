use gio::prelude::*;
use gtk::Application;

use std::env::args;

mod conversions;
mod helper;
mod structs;
mod ui_builder;
mod watch;

use helper::get_configuration;
use structs::{Notebook, SerializableWidget};
use ui_builder::{build_ui, deserialize_from_file};

fn main() {
    let (config_buffer, css_path) = get_configuration();
    let main_widget = SerializableWidget::Notebook(Notebook {
        css: None,
        name: Some("toplevel".to_string()),
        // each line in the config file should be the path of a file describing a widget.
        children: config_buffer
            .split('\n')
            .into_iter()
            .filter(|&line| !(line.starts_with('#') || line.is_empty()))
            .map(deserialize_from_file)
            .collect(),
    });

    // try to create a new application, if that fails then just return an error and quit
    if let Ok(app) = Application::new(
        Some("com.github.ap29600.Dama"),
        gio::ApplicationFlags::REPLACE | gio::ApplicationFlags::ALLOW_REPLACEMENT,
    ) {
        app.connect_activate(move |application| {
            build_ui(application, main_widget.clone(), css_path.clone())
        });
        app.run(&args().collect::<Vec<_>>());
    } else {
        eprint!("Could not create a new gtk application.");
    }
}
