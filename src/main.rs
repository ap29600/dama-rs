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
    let mut f = File::open("dama.json")?; 
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
