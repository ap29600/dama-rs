use crate::structs::SerializableWidget;
use gtk::{prelude::*, Application, ApplicationWindow};

// accepts a widget intermediate representation and an application,
// constructs a new window and populates it with the widgets
pub fn build_ui(app: &Application, widget: SerializableWidget, css_path: Option<String>) {
    // generic gtk boiler plate
    let win = ApplicationWindow::new(app);
    win.set_title("Dama - Menu");
    win.set_border_width(10);
    win.set_position(gtk::WindowPosition::Center);
    if let Some(css_path) = css_path {
        let screen = gdk::Screen::get_default().unwrap();
        let style_provider = gtk::CssProvider::new();
        match style_provider.load_from_path(&*css_path) {
            Ok(_) => gtk::StyleContext::add_provider_for_screen(&screen, &style_provider, 0),
            Err(e) => eprint!("{}", e),
        }
    } else {
        eprint!("No css stylesheet was found");
    }

    // construct a widget from the intermediate representation
    win.add_from(widget);

    win.show_all();
}

// helper to make an error page out of a string
pub fn generate_fallback_layout(text: String) -> SerializableWidget {
    SerializableWidget::Box(crate::structs::Box {
        title: String::from("Error"),
        orientation: crate::structs::OrientationSerial::Horizontal,
        children: vec![SerializableWidget::Label(Label { text, css: None })],
        css: None,
    })
}

// reads a widget description from file and generates
// an intermediate representation with serde
use std::io::Read;
pub fn deserialize_from_file(file_name: &str) -> SerializableWidget {
    // this is declared here to be dropped after the closure it is passed to
    let mut file_contents = String::new();

    // here we determine the deserializer we need to use
    let deserializer: std::boxed::Box<dyn Fn(_) -> Option<SerializableWidget>>;
    if file_name.ends_with(".yml") {
        deserializer = std::boxed::Box::new(|file| serde_yaml::from_str(file).ok());
    } else if file_name.ends_with("json") {
        deserializer = std::boxed::Box::new(|file| serde_json::from_str(file).ok());
    } else {
        return generate_fallback_layout(format!(
            "this file does not have a supported extension: {};\n\
                     Supported file extensions are .json and .yml",
            file_name
        ));
    }

    // try to read from file
    if let Some(_) = std::fs::File::open(file_name.clone())
        .ok()
        .map(|mut f| f.read_to_string(&mut file_contents).ok())
        .flatten()
    // the flatten makes sure we catch errors both  opening and reading the file
    {
        match &*file_contents {
            // if the file is empty, just generate an error page
            "" => generate_fallback_layout(format!(
                "the config file for this page was empty: {}",
                file_name
            )),
            //  otherwise, try to deserialize
            widget_string => deserializer(widget_string)
                // if that fails then make an error including the faulty file
                .unwrap_or(generate_fallback_layout(format!(
                    "could not deserialize: \n{}",
                    widget_string
                ))),
        }
    } else {
        // if we encountered an error reading from file, generate an error page
        generate_fallback_layout(format!(
            "it seems no config file was found for this page: {}",
            file_name
        ))
    }
}

// dirty workaround to avoid having to use specialization, which currently is unstable.
// it would be ideal for `add_maybe_with_label()` to be a method of AddFromSerializable,
// implemented differently for Notebook.
pub trait ContainerMaybeWithLabel {
    fn add_maybe_with_label<W: IsA<gtk::Widget>>(&self, element: &W, label: Option<&str>);
}

// setting a separate implementation for boxes and the main window as opposed to notebooks:
// Notebooks will use the child's label if it has one.
impl ContainerMaybeWithLabel for gtk::Notebook {
    fn add_maybe_with_label<W: IsA<gtk::Widget>>(&self, element: &W, label: Option<&str>) {
        self.append_page(element, Some(&gtk::Label::new(label)));
    }
}

use duplicate::duplicate;
// anything else will ignore the label.
#[duplicate (SimpleContainer; [gtk::ApplicationWindow]; [gtk::Box])]
impl ContainerMaybeWithLabel for SimpleContainer {
    fn add_maybe_with_label<W: IsA<gtk::Widget>>(&self, element: &W, _label: Option<&str>) {
        self.add(element);
    }
}

use crate::structs::*;

pub trait AddFromSerializable {
    fn add_from(&self, obj: SerializableWidget);
}

// TODO: split this into smaller chunks for maintainability
impl<T> AddFromSerializable for T
where
    T: ContainerExt,
    T: ContainerMaybeWithLabel,
{
    // accepts an intermediate representation, produces a widget and attaches it to self
    fn add_from(&self, obj: SerializableWidget) {
        match obj {
            SerializableWidget::Box(serialbox) => {
                let title = serialbox.title.clone();
                let gtkbox: gtk::Box = serialbox.into();
                self.add_maybe_with_label(&gtkbox, Some(&title));
            }
            SerializableWidget::Notebook(notebook) => {
                self.add::<gtk::Notebook>(&notebook.into());
            }
            SerializableWidget::Button(button) => {
                self.add::<gtk::Button>(&button.into());
            }
            SerializableWidget::CheckBox(checkbox) => {
                self.add::<gtk::CheckButton>(&checkbox.into());
            }
            SerializableWidget::Image(image) => {
                self.add::<gtk::Image>(&image.into());
            }
            SerializableWidget::Label(label) => {
                self.add::<gtk::Label>(&label.into());
            }
            SerializableWidget::Scale(scale) => {
                self.add::<gtk::Scale>(&scale.into());
            }
            SerializableWidget::ComboBox(combo_box) => {
                self.add::<gtk::ComboBoxText>(&combo_box.into());
            }
        }
    }
}
