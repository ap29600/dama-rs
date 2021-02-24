use gtk::prelude::*;
use super::helper::*;
use super::structs::*;

pub trait AddFromSerializable {
    fn add_from(&self, obj: SerializableWidget );
}

// dirty workaround to avoid ahving to use specialization, which currently is unstable.
// it would be ideal for `add_maybe_with_label()` to be a method of AddFromSerializable,
// implemented differently for Notebook.
pub trait ContainerMaybeWithLabel {
    fn add_maybe_with_label<W: IsA<gtk::Widget>> (&self, element: &W, label: Option<&str>);
}

impl ContainerMaybeWithLabel for gtk::Notebook {
    // if the parent is a notebook, we use the label as a name for the tab
    fn add_maybe_with_label<W: IsA<gtk::Widget>>(&self, element: &W, label: Option<&str>) {
        self.append_page(element, Some(& gtk::Label::new(label)));
    }
}

impl ContainerMaybeWithLabel for gtk::ApplicationWindow {
    // otherwise, ignore the label
    fn add_maybe_with_label<W: IsA<gtk::Widget>>(&self, element: &W, _label: Option<&str>) {
        self.add(element);
    }
}

impl ContainerMaybeWithLabel for gtk::Box {
    // same here
    fn add_maybe_with_label<W: IsA<gtk::Widget>>(&self, element: &W, _label: Option<&str>) {
        self.add(element);
    }
}

// this seems redundant, maybe we could have serde derive the 
// (de)serialization directly for `gtk::Orientation`
impl Into<gtk::Orientation> for Orientation {
    fn into(self) -> gtk::Orientation {
        match self {
            Orientation::Vertical => gtk::Orientation::Vertical,
            Orientation::Horizontal => gtk::Orientation::Horizontal,
        }
    }
}

impl<T> AddFromSerializable for T 
    where T: ContainerExt, T:ContainerMaybeWithLabel {
    fn add_from(&self, obj: SerializableWidget) {
        match obj {
            SerializableWidget::Box(name, orientation, elements) => {
                let b = gtk::Box::new(orientation.into(), match orientation {
                    Orientation::Horizontal => 20,
                    Orientation::Vertical   => 0 });
                b.set_border_width(10);
                for element in elements {
                    b.add_from(element);
                }
                self.add_maybe_with_label(&b, Some(&*name));
            }
            SerializableWidget::Notebook(v) => {
                let nb = gtk::Notebook::new();
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
            SerializableWidget::Label(label) => {
                let l = gtk::Label::new(None);
                l.set_markup(&*label);
                l.set_line_wrap(true);
                self.add(&l);
            }
            SerializableWidget::Scale(start, end, initialize, update) => {
                let l = gtk::Scale::with_range( gtk::Orientation::Horizontal, start, end, 5.);
                l.set_value(read_value_from_command::<f64>(initialize, start));
                l.set_size_request(200, 12);
                l.connect_change_value(
                    move |_, _, new_value| { 
                        // the new value is accessible as an environment variable
                        std::env::set_var("DAMA_VAL", new_value.floor().to_string() );
                        execute_shell_command( update.clone() );
                        Inhibit(false)
                    });
                self.add(&l);
            }
        }
    }
}
