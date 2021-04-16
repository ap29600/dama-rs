use gtk::{ApplicationWindow, Application};
use gtk::prelude::*;
use gio::prelude::*;
use helper::deserialize_from_file;
use std::io::prelude::*;

use std::fs::File;

use std::env::args;

mod structs;
use structs::*;

pub mod helper;

mod traits;
use traits::*;


fn build_ui(app: &Application, widget: SerializableWidget) {
    let win = ApplicationWindow::new(app);
    win.set_title("Dama - Menu");
    win.set_border_width(10);
    win.set_position(gtk::WindowPosition::Center);
    
    // here we construct a widget structure recursively
    // from the deserializable version
    win.add_from(widget);
    
    win.show_all();
}


fn main() -> std::io::Result<()> {
    
    let base_dirs = directories::BaseDirs::new().unwrap(); 
    let mut conf_file = base_dirs.config_dir().to_path_buf();
    let mut home_file = base_dirs.home_dir().to_path_buf();
    conf_file.push("dama/config");
    home_file.push(".dama/config");
    
    // we will store the config in this string before deserializing
    let mut source = String::new();
    
    // try to get a config file
    if let Ok(mut f) = File::open(conf_file) {
        f.read_to_string(&mut source)?;}
    else if let Ok(mut f) = File::open(home_file) {
        f.read_to_string(&mut source)?;} 
    
    let widgets_list = source.split('\n').into_iter()
        .filter(|&line| {! line.starts_with("#")}) // take out the comments
        .filter(|&line| {! line.is_empty()}) // take out empty lines
        .map( deserialize_from_file )
        .collect::<Vec<_>>();
    
    let widget = SerializableWidget::Notebook(widgets_list);

    let app = match Application::new(Some("com.github.ap29600.Dama"), gio::ApplicationFlags::REPLACE | gio::ApplicationFlags::ALLOW_REPLACEMENT) {
        Ok(app) => app,
        Err(e)  => panic!("could not initialize application {}", e)
    }; // would be nice to use `?` here, but we would need to convert from `glib::error::BoolError`
    app.connect_activate(move |a| build_ui(a, widget.clone()));
    app.run(&args().collect::<Vec<_>>());
    Ok(())
}
