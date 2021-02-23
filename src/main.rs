use gtk::{ApplicationWindow, Application};
use gtk::prelude::*;
use gio::prelude::*;

use std::{fmt::Debug, io::prelude::*, str::FromStr};
use std::fs::File;
use serde_derive::{Serialize, Deserialize};

use std::env::args;

#[derive(Serialize, Deserialize)]
enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Serialize, Deserialize)]
enum SeralizableWidget {
    Notebook(Vec<SeralizableWidget>),
    Box(Orientation, Vec<SeralizableWidget>),
    Label(String),
    Button(String, String), // label, command
    Scale(f64, f64, String, String), // min, max, initialize, update
}


fn read_value_from_command <T> (command: String, default: T) -> T 
    where T: std::str::FromStr, <T as FromStr>::Err: Debug {
    match std::process::Command::new("sh").arg("-c").arg(command).output() {
        Err(e) => { println!("error while running command: {}", e); default },
        Ok(std::process::Output{status: _ , stdout: utf_8_vec, stderr: _}) => 
            match String::from_utf8(utf_8_vec.iter().take_while(|&c| {*c != '\n' as u8}).map(|&c| c).collect()) { // this is very ugly, but it avoids parse() failing with newlines
                Err(e) => { println!("error while making a string: {}", e); default },
                Ok(string) => 
                    match string.parse() {
                        Err(e) => { println!("error while parsing string \"{}\" {:?}",string,  e); default },
                        Ok(value) => value
                    }
            }
    }
}


trait AddFromSerializable {
    fn add_from(&self, obj: SeralizableWidget );
}

impl<T> AddFromSerializable for T 
    where T: ContainerExt {
    fn add_from(&self, obj: SeralizableWidget) {
        match obj {
            SeralizableWidget::Box(orientation, elements) => {
                let nb = gtk::Box::new(match orientation {
                    Orientation::Vertical => gtk::Orientation::Vertical,
                    Orientation::Horizontal => gtk::Orientation::Horizontal,}
                    , 12);
                for element in elements {
                    nb.add_from(element);
                }
                self.add(&nb);
            }
            SeralizableWidget::Notebook(v) => {
                let nb = gtk::Notebook::new();
                for elem in v {
                    nb.add_from(elem);
                }
                self.add(&nb);
            }
            SeralizableWidget::Button(label, command) => {
                let b = gtk::Button::with_label(&*label);
                b.connect_clicked(
                    move |_| {std::process::Command::new("sh").arg("-c")
                        .arg(command.clone()).spawn().unwrap();
                    });
                self.add(&b);
            }
            SeralizableWidget::Label(label) => {
                let l = gtk::Label::new(None);
                l.set_markup(&*label);
                l.set_line_wrap(true);
                self.add(&l);
            }
            SeralizableWidget::Scale(start, end, initialize, update) => {
                let l = gtk::Scale::with_range( gtk::Orientation::Horizontal, start, end, 1.);

                l.set_value(read_value_from_command::<f64>(initialize, start));

                l.connect_change_value(
                    move |_, _, new_value| {std::process::Command::new("sh").arg("-c")
                            .arg(format!("{} {}", update.clone(), new_value as i32))
                            .spawn().unwrap();
                        Inhibit(false)
                    });
                self.add(&l);
            }
        }
    }
}


fn build_ui(app: &Application, source: &str) {
    let win = ApplicationWindow::new(app);
    win.set_title("Dama - Menu");
    win.set_border_width(10);
   
    win.set_position(gtk::WindowPosition::Center);
    let mywidget: SeralizableWidget = serde_json::from_str(source).unwrap();
   
    win.add_from(mywidget);
    win.show_all();
}


fn main() {
    let mut f = File::open("dama.json").expect("could not read from file");
    let mut source = String::new();
    f.read_to_string(&mut source).expect("there was an error parsing the config file");
    let app = Application::new(Some("com.andrea.example"), Default::default())
        .expect("Could not create a new application");
    app.connect_activate(move |a| build_ui(a, &*source));
    app.run(&args().collect::<Vec<_>>());
}
