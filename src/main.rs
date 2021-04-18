use gio::prelude::*;
use gtk::Application;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;

mod conversions;
mod helper;
mod structs;
mod ui_builder;
mod watch;

use structs::{Notebook, SerializableWidget};
use ui_builder::{build_ui, deserialize_from_file};

fn main() {
    let base_dirs = directories::BaseDirs::new().unwrap();
    let dot_config_main_file = base_dirs.config_dir().to_path_buf().join("dama/config");
    let home_main_file = base_dirs.home_dir().to_path_buf().join(".dama/config");

    // we will store the config in this string before deserializing
    let mut config_buffer = String::new();

    // read the first file path in the vec and dump the file to buffer
    vec![dot_config_main_file, home_main_file]
        .iter()
        .find_map(|path| if path.is_file() { Some(path) } else { None })
        .map(|path| File::open(path)?.read_to_string(&mut config_buffer));

    let main_widget = SerializableWidget::Notebook(Notebook {
        css: None,
        name: Some(String::from("toplevel")),
        children: config_buffer
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

    let dot_config_css_file = base_dirs.config_dir().to_path_buf().join("dama/style.css");
    let home_css_file = base_dirs.home_dir().to_path_buf().join(".dama/style.css");

    // like above, find the first file in the list that actually exists
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
    if let Some(app) = Application::new(
        Some("com.github.ap29600.Dama"),
        gio::ApplicationFlags::REPLACE | gio::ApplicationFlags::ALLOW_REPLACEMENT,
    )
    .ok()
    {
        app.connect_activate(move |application| {
            build_ui(application, main_widget.clone(), css_path.clone())
        });
        app.run(&args().collect::<Vec<_>>());
    }
}
