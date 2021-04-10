use std::{fmt::Debug, str::FromStr};
use std::process::{Command, Output};


pub fn execute_shell_command(command: String) {
    match std::process::Command::new("sh").arg("-c")
        .arg(command.clone()).output() {
            Ok(_) => (),
            Err(e) => eprint!("{}", e)
        }
}


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
    match &*read_stdout_from_command(command.clone()) {
        "" => { eprint!( "Output was empty for command:\n > {}\n, falling back to default value", 
                         command);
            default } ,
        output => output.split("\n").next()
            .map(|line| match line.parse() {
                     Ok(val) => Some(val), 
                     Err(e) => {eprint!("{:?}",e);None}
                 })
            .flatten().unwrap_or(default)
    }
}
