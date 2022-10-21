use gio::prelude::*;
use gtk::Application;

use std::env::args;
use std::fs::File;
use std::io::Read;

mod conversions;
mod helper;
mod structs;
mod ui_builder;
mod watch;

use helper::{get_config_lines, get_configuration};
use structs::{Notebook, SerializableWidget};
use ui_builder::{build_ui, deserialize_from_file};

fn main() {
    let mut remaining_args = vec![];
    let pages_from_args = args()
        .filter_map(|mut arg| {
            if arg.starts_with("-p:") {
                Some(arg.split_off(3))
            } else {
                remaining_args.push(arg);
                None
            }
        })
        .collect::<Vec<_>>();

    let (config_path, css_path) = get_configuration();
    let pages = if pages_from_args.is_empty() {
        let mut config = String::new();
        if let Some(config_path) = config_path {
            File::open(config_path)
                .ok()
                .map(|mut f| f.read_to_string(&mut config));
        };
        get_config_lines(&config)
            .map(deserialize_from_file)
            .collect()
    } else {
        pages_from_args
            .iter()
            .map(|file| deserialize_from_file(&file))
            .collect()
    };

    let main_widget = SerializableWidget::Notebook(Notebook {
        css: None,
        name: Some("toplevel".to_string()),
        // each line in the config file should be the path of a file describing a widget.
        children: pages,
    });

    // try to create a new application, if that fails then just return an error and quit
    if let Ok(app) = Application::new(
        Some("com.github.ap29600.Dama"),
        gio::ApplicationFlags::REPLACE | gio::ApplicationFlags::ALLOW_REPLACEMENT,
    ) {
        app.connect_activate(move |application| {
            build_ui(application, main_widget.clone(), css_path.clone())
        });
        app.run(&remaining_args);
    } else {
        eprint!("Could not create a new gtk application.");
    }
}
