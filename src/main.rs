use std::process::Command;

use gtk::{
    RangeExt,
    ButtonExt,
    Inhibit,
    OrientableExt,
    WidgetExt,
};

use gtk::Orientation::Vertical;
use relm::Widget;
use relm_derive::{Msg, widget};

use self::Msg::*;

pub struct Model {
}

#[derive(Msg)]
pub enum Msg {
    Quit,
    RunCommand(String),
}

#[widget]
impl Widget for Win {
    fn init_view(&mut self) {
        self.widgets.brightness.set_range(0., 100.);
        self.widgets.brightness.set_value(50.);

    }

    fn model( _: ()) -> Model {
        Model {
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
            RunCommand(command) => {
                Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .spawn().unwrap();
            }
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                gtk::Notebook {
                    gtk::Box {
                        orientation: Vertical,
                        #[name="brightness"]
                        gtk::Scale{
                            change_value( _ , _ , new_value) => 
                                (RunCommand(format!("echo {} > /sys/class/backlight/amdgpu_bl0/brightness", new_value as u8)), 
                                    Inhibit(false))
                        }
                    },
                    gtk::Box {
                        orientation: Vertical,
                        gtk::Button {
                            label: "hello",
                            clicked => RunCommand("notify-send Hello".to_string()),
                        },
                        gtk::Button {
                            label: "world",
                            clicked => RunCommand("notify-send World".to_string()),
                        }
                    },
                }
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}
