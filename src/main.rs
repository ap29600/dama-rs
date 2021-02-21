use gtk::{ApplicationWindow, Application};
use gtk::prelude::*;
use gio::prelude::*;

use std::io::prelude::*;
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
    Scale(f64, f64, String), // min, max, command
}


trait Attach {
    fn attach (&self, obj: SeralizableWidget );
}

impl Attach for gtk::Box {
    fn attach(&self, obj: SeralizableWidget) {
        match obj {
            SeralizableWidget::Box(orientation, elements) => {
                let nb = gtk::Box::new(match orientation {
                    Orientation::Vertical => gtk::Orientation::Vertical,
                    Orientation::Horizontal => gtk::Orientation::Horizontal,}
                    , 12);
                for element in elements {
                    nb.attach(element);
                }
                self.add(&nb);
            }
            SeralizableWidget::Notebook(v) => {
                let nb = gtk::Notebook::new();
                for elem in v {
                    nb.attach(elem);
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
            SeralizableWidget::Scale(start, end, command) => {
                let l = gtk::Scale::with_range(
                    gtk::Orientation::Horizontal, 
                    start, 
                    end, 
                    1.);
                l.connect_change_value(
                    move |_, _, new_value| {std::process::Command::new("sh").arg("-c")
                            .arg(format!("{} {}", command.clone(), new_value as i32))
                            .spawn().unwrap();
                        Inhibit(false)
                    });
                self.add(&l);
            }
        }
    }
}
impl Attach for gtk::Notebook {
    fn attach(&self, obj: SeralizableWidget) {
        match obj {
            SeralizableWidget::Box(orientation, elements) => {
                let nb = gtk::Box::new(match orientation {
                    Orientation::Vertical => gtk::Orientation::Vertical,
                    Orientation::Horizontal => gtk::Orientation::Horizontal,
                }, 12);
                for element in elements {
                    nb.attach(element);
                }
                self.add(&nb);
            }
            _ => {}
        }
    }
}

impl Attach for ApplicationWindow {
    fn attach(&self, obj: SeralizableWidget) {
        match obj {
            SeralizableWidget::Notebook(v) => {
                let nb = gtk::Notebook::new();
                for elem in v {
                    nb.attach(elem);
                }
                self.add(&nb);
            }
            _ => {}
        }
    }
}



fn build_ui(app: &Application, source: &str) {


    let win = ApplicationWindow::new(app);
    win.set_title("Dama - Menu");
    win.set_border_width(10);
   
    let otherwidget = SeralizableWidget::Box(Orientation::Vertical, vec![]);
    let s = serde_json::to_string_pretty(&otherwidget).unwrap();
    println!("{}", s);
    
    win.set_position(gtk::WindowPosition::Center);
    let mywidget: SeralizableWidget = serde_json::from_str(source).unwrap();
//  let mywidget: SeralizableWidget = SeralizableWidget::Notebook(
//      vec![
//      SeralizableWidget::Box(
//          vec! [
//          SeralizableWidget::Button("hello".to_string(), "notify-send world".to_string()),
//          SeralizableWidget::Label("some label".to_string()),
//          SeralizableWidget::Scale(0., 100., "notify-send".to_string()),
//          ])
//      ]);
    

    win.attach(mywidget);
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
