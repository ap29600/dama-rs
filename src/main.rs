use gtk::Application;
use gio::prelude::*;

use std::io::prelude::*;
use std::io::{ Error, ErrorKind };

use std::fs::File;

use std::env::args;

mod structs;
mod helper;
mod watch;
mod ui_builder;

use ui_builder::{deserialize_from_file, build_ui};
use structs::SerializableWidget;

fn main() -> std::io::Result<()> {
    
    let base_dirs = directories::BaseDirs::new().unwrap(); 
    let mut config_file = base_dirs.config_dir().to_path_buf();
    let mut home_file = base_dirs.home_dir().to_path_buf();
    config_file.push("dama/config");
    home_file.push(".dama/config");
    
    // we will store the config in this string before deserializing
    let mut config = String::new();
    
    // try to get a config file
    if let Ok(mut f) = File::open(config_file) {
        f.read_to_string(&mut config)?;}
    else if let Ok(mut f) = File::open(home_file) {
        f.read_to_string(&mut config)?;} 
    
    let main_widget = 
        SerializableWidget::Notebook(
            config.split('\n').into_iter()
            // take out the comments
            .filter(|&line| {! line.starts_with("#")}) 
            // take out empty lines
            .filter(|&line| {! line.is_empty()}) 
            // make a new page out of every file found
            .map( deserialize_from_file )
            .collect() 
            );

    // try to create a new application, if that fails then just return an error and quit 
    let app = Application::new(Some("com.github.ap29600.Dama"), gio::ApplicationFlags::REPLACE | gio::ApplicationFlags::ALLOW_REPLACEMENT)
        .ok().ok_or( Error::new(ErrorKind::Other, "couldn't create application"))?;
    
    app.connect_activate(move |application| build_ui(application, main_widget.clone()));
    app.run(&args().collect::<Vec<_>>());
    Ok(())
}
