use duplicate::duplicate;

use crate::structs::SerializableWidget;
use crate::helper::*;

use gtk::{prelude::*, Orientation};
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

#[duplicate (SimpleContainer; [gtk::ApplicationWindow]; [gtk::Box])]
impl ContainerMaybeWithLabel for SimpleContainer {
    // otherwise, ignore the label
    fn add_maybe_with_label<W: IsA<gtk::Widget>>(&self, element: &W, _label: Option<&str>) {
        self.add(element);
    }
}

impl<T> AddFromSerializable for T 
    where T: ContainerExt, T:ContainerMaybeWithLabel {
    fn add_from(&self, obj: SerializableWidget) {
        match obj {
            SerializableWidget::Box(name, orientation, elements) => {
                let b = gtk::Box::new(orientation, match orientation {
                    Orientation::Horizontal => 30,
                    _ => 0
                });
                b.set_border_width(10);
                // this is checked here because elements gets moved later
                let first_is_label = match elements.iter().next() {
                        Some(SerializableWidget::Label(_)) => true,
                        _ => false };
                for element in elements {
                    b.add_from(element);
                }
                if orientation == gtk::Orientation::Horizontal && first_is_label {
                    // a leading label will fill all the left side, 
                    // pushing buttons to the right
                    b.get_children().iter().next()
                        .map(|w| b.set_child_packing(
                                w,
                                true,   // expand
                                true,  // fill
                                12,     // padding
                                gtk::PackType::Start)
                            );
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
            SerializableWidget::Scale(start, end, initialize, update) => {
                let l = gtk::Scale::with_range( gtk::Orientation::Horizontal, start, end, 5.);
                l.set_value(read_value_from_command::<f64>(initialize, start));
                l.set_size_request(250, 12);
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
