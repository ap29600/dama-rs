use gtk::{ApplicationWindow, Application};
use gtk::prelude::*;
use gio::prelude::*;
use std::io::prelude::*;

use std::fs::File;

use std::env::args;

mod structs;
use structs::*;

pub mod helper;

mod traits;
use traits::*;


fn build_ui(app: &Application, source: &str) {
    let win = ApplicationWindow::new(app);
    win.set_title("Dama - Menu");
    win.set_border_width(10);
    win.set_position(gtk::WindowPosition::Center);
    if let Some(mywidget) = serde_json::from_str::<SeralizableWidget>(source).ok(){
            win.add_from(mywidget);
            win.show_all();
        } else { eprint!("could not deserialize string: {}", source); }
}


fn main() -> std::io::Result<()> {
    let home_path = std::env::var("HOME").unwrap();
    let fallback_config_path = home_path.clone() + "/.config";
    let config_path = match std::env::var("XDG_CONFIG_HOME") {
        Ok(f) => f,
        _     => fallback_config_path.clone()
    };
    
    // very ugly, there must be a way to chain these more cleanly
    let mut f = match File::open(config_path + "/dama.json") {
        Ok(f) => f,
        _     => match File::open(fallback_config_path + "/dama.json") {
            Ok(f) => f,
            _     => match File::open(home_path + "/.dama.json") {
                Ok(f) => f,
                _     => panic!("could not find a suitable config file")
            }
        }
    };

    let mut source = String::new();
    f.read_to_string(&mut source)? ;
    let app = match Application::new(Some("com.andrea.example"), Default::default()) {
        Ok(app) => app,
        Err(e)  => panic!("could not initialize application {}", e)
    }; // would be nice to use `?` here, but we would need to convert from `glib::error::BoolError`
    app.connect_activate(move |a| build_ui(a, &*source));
    app.run(&args().collect::<Vec<_>>());
    Ok(())
}
