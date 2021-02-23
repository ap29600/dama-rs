use gtk::prelude::*;
use super::helper::*;
use super::structs::*;

pub trait AddFromSerializable {
    fn add_from(&self, obj: SeralizableWidget );
}

pub trait ContainerMaybeLabel {
    fn add_maybe_with_label<W: IsA<gtk::Widget>> (&self, element: &W, label: Option<&str>);
}

impl ContainerMaybeLabel for gtk::Notebook {
    fn add_maybe_with_label<W: IsA<gtk::Widget>>(&self, element: &W, label: Option<&str>) {
        self.append_page(element, Some(& gtk::Label::new(label)));
    }
}

impl ContainerMaybeLabel for gtk::ApplicationWindow {
    fn add_maybe_with_label<W: IsA<gtk::Widget>>(&self, element: &W, _label: Option<&str>) {
        self.add(element);
    }
}

impl ContainerMaybeLabel for gtk::Box {
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
    where T: ContainerExt, T:ContainerMaybeLabel {
    fn add_from(&self, obj: SeralizableWidget) {
        match obj {
            SeralizableWidget::Box(name, orientation, elements) => {
                let nb = gtk::Box::new(orientation.into(), 12);
                for element in elements {
                    nb.add_from(element);
                }
                self.add_maybe_with_label(&nb, Some(&*name));
            }
            SeralizableWidget::Notebook(v) => {
                let nb = gtk::Notebook::new();
                for elem in v {
                    nb.add_from(elem);
                }
                self.add(&nb)
            }
            SeralizableWidget::Button(label, command) => {
                let b = gtk::Button::with_label(&*label);
                b.connect_clicked( move |_| execute_shell_command (command.clone()) );
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
                l.set_hexpand(true);
                l.connect_change_value(
                    move |_, _, new_value| { 
                        execute_shell_command( format!("{} {}", update.clone(), new_value as i32));
                        Inhibit(false)
                    });
                self.add(&l);
            }
        }
    }
}
