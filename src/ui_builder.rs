use crate::structs::SerializableWidget;
use gtk::{Application, ApplicationWindow, prelude::*};


// accepts a widget intermediate representation and an application,
// constructs a new window and populates it with the widgets
pub fn build_ui(app: &Application, widget: SerializableWidget) {
    // generic gtk boiler plate
    let win = ApplicationWindow::new(app);
    win.set_title("Dama - Menu");
    win.set_border_width(10);
    win.set_position(gtk::WindowPosition::Center);
    
    // construct a widget from the intermediate representation
    win.add_from(widget);
    
    win.show_all();
}


// helper to make an error page out of a string
fn generate_fallback_layout(text: String) -> SerializableWidget {
    SerializableWidget::Box(
        String::from("Error"),
        gtk::Orientation::Horizontal, 
        vec![ SerializableWidget::Label(text) ])
}


// reads a widget description from file and generates 
// an intermediate representation with serde
use std::io::Read;
pub fn deserialize_from_file (file_name : &str ) -> SerializableWidget {
    // this is declared here to be dropped after the closure it is passed to
    let mut file_contents = String::new();
   
    // here we determine the deserializer we need to use
    let deserializer: Box<dyn Fn (_) -> Option<SerializableWidget>>;
    if file_name.ends_with(".yml") {
        deserializer = Box::new(|file| serde_yaml::from_str(file).ok());
    } else if file_name.ends_with("json") {
        deserializer = Box::new(|file| serde_json::from_str(file).ok());
    } else {
        return generate_fallback_layout( 
            format!( "this file does not have a supported extension: {};\n\
                     Supported file extensions are .json and .yml", file_name));
    }
    
    // try to read from file
    if let Some(_) = 
        std::fs::File::open(file_name.clone()).ok()
            .map(|mut f| {f.read_to_string(&mut file_contents).ok()}).flatten() 
    // the flatten makes sure we catch errors both  opening and reading the file
    { match &*file_contents 
        { // if the file is empty, just generate an error page
            "" => generate_fallback_layout(
                format!("the config file for this page was empty: {}", file_name)), 
            //  otherwise, try to deserialize
            widget_string => deserializer(widget_string)
                // if that fails then make an error including the faulty file
                .unwrap_or( generate_fallback_layout( 
                    format!("could not deserialize: \n{}", widget_string)
                    )) }
    } else { // if we encountered an error reading from file, generate an error page
        generate_fallback_layout(
            format!("it seems no config file was found for this page: {}", file_name))
    }
}


// dirty workaround to avoid having to use specialization, which currently is unstable.
// it would be ideal for `add_maybe_with_label()` to be a method of AddFromSerializable,
// implemented differently for Notebook.
trait ContainerMaybeWithLabel {
    fn add_maybe_with_label<W: IsA<gtk::Widget>> (&self, element: &W, label: Option<&str>);
}


// setting a separate implementation for boxes and the main window as opposed to notebooks:
// Notebooks will use the child's label if it has one.
impl ContainerMaybeWithLabel for gtk::Notebook {
    fn add_maybe_with_label<W: IsA<gtk::Widget>>(&self, element: &W, label: Option<&str>) {
        self.append_page(element, Some(& gtk::Label::new(label)));
    }
}


// anything else will ignore the label.
#[duplicate (SimpleContainer; [gtk::ApplicationWindow]; [gtk::Box])]
impl ContainerMaybeWithLabel for SimpleContainer {
    fn add_maybe_with_label<W: IsA<gtk::Widget>>(&self, element: &W, _label: Option<&str>) {
        self.add(element);
    }
}


use gtk::Orientation;
use duplicate::duplicate;
use crate::helper::*;
use crate::watch::Watch;

trait AddFromSerializable {
    fn add_from(&self, obj: SerializableWidget );
}

// TODO: split this into smaller chunks for maintainability
impl<T> AddFromSerializable for T 
    where T: ContainerExt, T:ContainerMaybeWithLabel {
    // accepts an intermediate representation, produces a widget and attaches it to self
    fn add_from(&self, obj: SerializableWidget) {
        match obj {
            SerializableWidget::Box(name, orientation, elements) => {
                let b = gtk::Box::new(orientation, match orientation {
                    Orientation::Horizontal => 30,
                    _ => 0
                });
                b.set_border_width(10);
                // would be nice to just stop listening to draw signals after the first one
                // but gtk does not expose a connect_first_draw() function or similar;
                // there is probably a better way to do this.
                b.connect_draw(
                    move |b, _|
                    {
                        // only populate the box when drawing, if empty.
                        // this way if you have many pages running
                        // intensive scripts only the ones you actually use
                        // will be loaded.
                        if b.get_children().len() == 0 {
                            for element in elements.clone() {
                                b.add_from(element);
                            }
                            // if the first element is a label make
                            // it expand to push other stuff aside
                            b.get_children().iter().next() .map( 
                                |w| { 
                                    if w.is::<gtk::Label>() {
                                        b.set_child_packing(
                                            w,
                                            true,   // expand
                                            true,  // fill
                                            12,     // padding
                                            gtk::PackType::Start);
                                    }
                                });
                        }
                        b.show_all();
                        Inhibit(false)
                    });
                self.add_maybe_with_label(&b, Some(&*name));
            }
            SerializableWidget::Notebook(v) => {
                let nb = gtk::Notebook::new();
                nb.set_tab_pos(gtk::PositionType::Left);
                for elem in v {
                    nb.add_from(elem);
                }
                self.add(&nb)
            }
            SerializableWidget::Button(label, command) => {
                let b = gtk::Button::with_label(&*label);
                b.connect_clicked( move |_| execute_shell_command (command.clone()) );
                self.add(&b);
            }
            SerializableWidget::Checkbox(label, initialize, update) => {
                let c = gtk::CheckButton::with_label(&*label);
                c.set_active(read_value_from_command::<bool>(initialize, false));
                c.connect_toggled(move |checkbox| {
                        std::env::set_var("DAMA_VAL", checkbox.get_active().to_string());
                        execute_shell_command( update.clone() );
                });
                self.add(&c);
            }
            SerializableWidget::Image(path) => {
                let l = gtk::Image::from_file(path);
                l.set_margin_top(10);
                l.set_margin_bottom(10);
                l.set_margin_start(10);
                l.set_margin_end(10);
                self.add(&l);
            }
            SerializableWidget::Label(label) => {
                let l = gtk::Label::new(None);
                l.set_markup(&*label);
                l.set_line_wrap(true);
                l.set_xalign(0.0);
                self.add(&l);
            }
            SerializableWidget::Scale(start, end, initial_command, update_command) => {
                let l = gtk::Scale::with_range( gtk::Orientation::Horizontal, start, end, 5.);
                let initial_value = read_value_from_command::<f64>(initial_command, start);
                l.set_size_request(250, 12);
                l.set_value(initial_value);
                
                let tx = Watch::new(initial_value); 
                let mut rx = tx.clone();
                std::thread::spawn( move || { loop { 
                    std::env::set_var("DAMA_VAL", rx.wait().floor().to_string());
                    execute_shell_command(update_command.clone()); 
                }});
                l.connect_change_value(
                    move |_, _, new_value| { 
                        tx.clone().set_value(new_value);
                        Inhibit(false)
                    });
                self.add(&l);
            }
            SerializableWidget::Combo(list, init, update) => {
                let combo = gtk::ComboBoxText::new();
                let rawoptions = read_stdout_from_command(list);
                let options = rawoptions
                    .split("\n")
                    .filter(|line| !line.is_empty())
                    .collect::<Vec<_>>();

                let active = options.iter()
                    .position( move |entry| {
                        entry.to_string() == 
                            read_value_from_command(init.clone(), "".to_string()).to_string() })
                    .map(|i| i as u32); 
                for entry in options {
                    combo.append(None, entry);
                }
                combo.set_active(active);
                combo.connect_changed( move |combo| {
                    std::env::set_var("DAMA_VAL", combo.get_active_text().unwrap());
                    execute_shell_command(update.clone())} );
                self.add(&combo);
            }
        }
    }
}
