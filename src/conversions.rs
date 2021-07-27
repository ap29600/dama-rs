use crate::helper::*;
use crate::structs::*;
use crate::watch::*;
use gtk::prelude::*;

#[macro_export]
macro_rules! add_css{
    ($css:expr, $($widget:expr),*) => (
        {
            $(
                if let Some(css) = $css {
                    let provider = gtk::CssProvider::new();
                    match provider.load_from_data(css.as_bytes()) {
                        Ok(_) => $widget
                            .get_style_context()
                            .add_provider(&provider,
                                          gtk::STYLE_PROVIDER_PRIORITY_USER),
                        Err(e) => eprint!("CSS: {}", e),
                    }
                }
            )*
        }
    );
}

#[macro_export]
macro_rules! add_name{
    ($name:expr, $($widget:expr),*) => (
        {
            $(
                if let Some(name) = $name {
                    $widget.set_widget_name(&*name);
                }
            )*
        }
    );
}

impl Into<gtk::ComboBoxText> for ComboBox {
    fn into(self) -> gtk::ComboBoxText {
        let ComboBox {
            initialize,
            select,
            on_update,
            css,
            name,
        } = self;

        let combo = gtk::ComboBoxText::new();
        let rawoptions = read_stdout_from_command(initialize);
        let options = rawoptions
            .split("\n")
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();

        let active = options
            .iter()
            .position(move |entry| {
                entry.to_string()
                    == read_value_from_command(select.clone(), "".to_string()).to_string()
            })
            .map(|i| i as u32);
        for entry in options {
            combo.append(None, entry);
        }
        combo.set_active(active);
        combo.connect_changed(move |combo| {
            std::env::set_var("DAMA_VAL", combo.get_active_text().unwrap());
            execute_shell_command(on_update.clone())
        });
        add_name!(name, combo);
        add_css!(css, combo);
        combo
    }
}

impl Into<gtk::Scale> for Scale {
    fn into(self) -> gtk::Scale {
        let Scale {
            range,
            initialize,
            on_update,
            css,
            name,
        } = self;

        let scale = gtk::Scale::with_range(gtk::Orientation::Horizontal, range.low, range.high, 5.);
        let initial_value = read_value_from_command::<f64>(initialize, range.low);
        scale.set_size_request(250, 12);
        scale.set_value(initial_value);
        let tx = Watch::new(initial_value);
        let mut rx = tx.clone();
        std::thread::spawn(move || loop {
            std::env::set_var("DAMA_VAL", rx.wait().floor().to_string());
            execute_shell_command(on_update.clone());
        });
        scale.connect_change_value(move |_, _, new_value| {
            tx.clone().set_value(new_value);
            Inhibit(false)
        });
        add_name!(name, scale);
        add_css!(css, scale);
        scale
    }
}

impl Into<gtk::Image> for Image {
    fn into(self) -> gtk::Image {
        let Image { path, css, name } = self;

        let image = gtk::Image::from_file(path);
        image.set_margin_top(10);
        image.set_margin_bottom(10);
        image.set_margin_start(10);
        image.set_margin_end(10);
        add_name!(name, image);
        add_css!(css, image);
        image
    }
}

impl Into<gtk::Label> for Label {
    fn into(self) -> gtk::Label {
        let Label { text, css, name } = self;

        let label = gtk::Label::new(None);
        label.set_markup(&*text);
        label.set_line_wrap(true);
        label.set_xalign(0.0);
        add_name!(name, label);
        add_css!(css, label);
        label
    }
}

impl Into<gtk::CheckButton> for CheckBox {
    fn into(self) -> gtk::CheckButton {
        let CheckBox {
            text,
            initialize,
            on_click,
            css,
            name,
        } = self;

        let checkbox = gtk::CheckButton::with_label(&*text);
        checkbox.set_active(read_value_from_command::<bool>(initialize, false));
        checkbox.connect_toggled(move |checkbox| {
            std::env::set_var("DAMA_VAL", checkbox.get_active().to_string());
            execute_shell_command(on_click.clone());
        });
        add_name!(name, checkbox);
        add_css!(css, checkbox);
        checkbox
    }
}

impl Into<gtk::Button> for Button {
    fn into(self) -> gtk::Button {
        let Button {
            text,
            on_click,
            css,
            name,
        } = self;

        let button = gtk::Button::with_label(&*text);
        button.connect_clicked(move |_| execute_shell_command(on_click.clone()));
        add_name!(name, button);
        add_css!(css, button);
        button
    }
}

use crate::ui_builder::AddFromSerializable;
impl Into<gtk::Notebook> for Notebook {
    fn into(self) -> gtk::Notebook {
        let Notebook {
            children,
            css,
            name,
        } = self;
        let notebook = gtk::Notebook::new();
        notebook.set_tab_pos(gtk::PositionType::Left);
        add_name!(name, notebook);
        add_css!(css, notebook);
        for child in children {
            notebook.add_from(child);
        }
        notebook
    }
}

impl Into<gtk::Box> for Box {
    fn into(self) -> gtk::Box {
        let Box {
            title: _,
            orientation,
            children,
            css,
            name,
        } = self;
        let gtkbox = gtk::Box::new(
            orientation.into(),
            match orientation {
                OrientationSerial::Horizontal => 30,
                _ => 0,
            },
        );
        gtkbox.set_border_width(10);
        // would be nice to just stop listening to draw signals after the first one
        // but gtk does not expose a connect_first_draw() function or similar;
        // there is probably a better way to do this.
        gtkbox.connect_draw(move |gtkbox, _| {
            // only populate the box when drawing, if empty.
            // this way if you have many pages running
            // intensive scripts only the ones you actually use
            // will be loaded.
            if gtkbox.get_children().len() == 0 {
                for child in children.clone() {
                    gtkbox.add_from(child);
                }
                // if the first element is a label make
                // it expand to push other stuff aside
                gtkbox.get_children().iter().next().map(|w| {
                    if w.is::<gtk::Label>() {
                        gtkbox.set_child_packing(
                            w,
                            true, // expand
                            true, // fill
                            12,   // padding
                            gtk::PackType::Start,
                        );
                    }
                });
                gtkbox.show_all();
            }
            Inhibit(false)
        });
        add_name!(name, gtkbox);
        add_css!(css, gtkbox);
        gtkbox
    }
}
