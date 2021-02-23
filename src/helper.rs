use std::{fmt::Debug, str::FromStr};
use std::process::{Command, Output};

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
        .arg(command.clone()).spawn() {
            Ok(_) => (),
            Err(e) => eprint!("{}", e)
        }
}
