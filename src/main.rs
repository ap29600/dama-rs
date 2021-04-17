use gio::prelude::*;
use gtk::Application;

use std::io::prelude::*;
use std::io::{Error, ErrorKind};

use std::fs::File;

use std::env::args;

mod conversions;
mod helper;
mod structs;
mod ui_builder;
mod watch;

use structs::{Notebook, SerializableWidget};
use ui_builder::{build_ui, deserialize_from_file};

fn main() -> std::io::Result<()> {
    let base_dirs = directories::BaseDirs::new().unwrap();
    let dot_config_main_file = base_dirs.config_dir().to_path_buf().join("dama/config");
    let home_main_file = base_dirs.home_dir().to_path_buf().join(".dama/config");

    let dot_config_css_file = base_dirs.config_dir().to_path_buf().join("dama/style.css");
    let home_css_file = base_dirs.home_dir().to_path_buf().join(".dama/style.css");

    // we will store the config in this string before deserializing
    let mut config = String::new();

    // grabs the first valid config and dumps it to our buffer
    vec![dot_config_main_file, home_main_file]
        .iter()
        .find_map(|path| if path.is_file() { Some(path) } else { None })
        .map(|path| File::open(path)?.read_to_string(&mut config));

    let main_widget = SerializableWidget::Notebook(Notebook {
        css: None,
        children: config
            .split('\n')
            .into_iter()
            // take out the comments
            .filter(|&line| !line.starts_with("#"))
            // take out empty lines
            .filter(|&line| !line.is_empty())
            // make a new page out of every file found
            .map(deserialize_from_file)
            .collect(),
    });

    let css_path = vec![dot_config_css_file, home_css_file]
        .iter()
        .find_map(|path| {
            if path.is_file() {
                path.clone().into_os_string().into_string().ok()
            } else {
                None
            }
        });

    // try to create a new application, if that fails then just return an error and quit
    let app = Application::new(
        Some("com.github.ap29600.Dama"),
        gio::ApplicationFlags::REPLACE | gio::ApplicationFlags::ALLOW_REPLACEMENT,
    )
    .ok()
    .ok_or(Error::new(ErrorKind::Other, "couldn't create application"))?;

    app.connect_activate(move |application| {
        build_ui(application, main_widget.clone(), css_path.clone())
    });
    app.run(&args().collect::<Vec<_>>());
    Ok(())
}
