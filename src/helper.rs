use std::{fmt::Debug, str::FromStr};
use std::process::{Command, Output};
use std::io::Read;

use crate::structs::SerializableWidget;

pub fn read_stdout_from_command (command: String) -> String {
    match Command::new("sh").arg("-c").arg(command).output() {
        Err(e) => { println!("error while running command: {}", e); "".to_string() },
        Ok(Output{status: _ , stdout: utf_8_vec, stderr: _}) => 
            match String::from_utf8(utf_8_vec)
            { 
                Err(e) => { println!("error while making a string: {}", e); "".to_string()},
                Ok(string) => string
            }
    }
}


pub fn read_value_from_command <T> (command: String, default: T) -> T 
    where T: std::str::FromStr, <T as FromStr>::Err: Debug {
    match Command::new("sh").arg("-c").arg(command).output() {
        Err(e) => { println!("error while running command: {}", e); default },
        Ok(Output{status: _ , stdout: utf_8_vec, stderr: _}) => 
            match String::from_utf8(utf_8_vec.iter()
                                    .take_while(|&c| {*c != '\n' as u8})
                                    .map(|&c| c).collect()) 
                                    // this is very ugly, but it avoids 
                                    // parse() failing with newlines
            { 
                Err(e) => { println!("error while making a string: {}", e); default },
                Ok(string) => 
                    match string.parse() {
                        Err(e) => { 
                            println!("error while parsing string \"{}\" {:?}",string,  e); 
                            default 
                        },
                        Ok(value) => value
                    }
            }
    }
}


pub fn execute_shell_command(command: String) {
    match std::process::Command::new("sh").arg("-c")
        .arg(command.clone()).output() {
            Ok(_) => (),
            Err(e) => eprint!("{}", e)
        }
}

pub fn generate_fallback_layout(text: String) -> SerializableWidget {
    SerializableWidget::Box(
        "Error".to_string(), 
        gtk::Orientation::Horizontal, 
        vec![ SerializableWidget::Label(text) ])
}


pub fn deserialize_from_file (s : &str ) -> SerializableWidget {
    let mut buf = String::new();
    let inner_buf = std::fs::File::open(s.clone())
        .ok().map( |mut f| {
            f.read_to_string(&mut buf).unwrap();
            &*buf
        });

    match inner_buf  {
        Some("") => generate_fallback_layout(
            format!("the config file for this page was empty: {}", s)), 
        Some(widget_string) => 
            match { if s.ends_with(".yml") {
                serde_yaml::from_str(widget_string).ok()
            } else if s.ends_with(".json") {
                serde_json::from_str(widget_string).ok()
            } else {
                Some(generate_fallback_layout(format!("this file does not have a supported extension: {};\nSupported file extensions are json, yml.", s)))
            } } {
                Some(widget) => widget,
                None => 
                    generate_fallback_layout(format!("there was an error parsing this file: {}", s))
            },
        None => generate_fallback_layout(
            format!("it seems no config file was found for this page: {}", s))
    }
}
