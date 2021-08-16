use std::{fmt::Debug, str::FromStr};
use std::process::{Command, Output};


use std::fs::File;
use std::io::Read;
pub fn get_configuration() -> (String, Option<String>) {
    let base_dirs = directories::BaseDirs::new().unwrap();

    let dot_config_main_file = base_dirs.config_dir().to_path_buf().join("dama/config");
    let home_main_file = base_dirs.home_dir().to_path_buf().join(".dama/config");
    let mut config_buffer = String::new();
    [dot_config_main_file, home_main_file]
        .iter()
        .find_map(|path| if path.is_file() { Some(path) } else { None })
        .map(|path| File::open(path)?.read_to_string(&mut config_buffer));

    let dot_config_css_file = base_dirs.config_dir().to_path_buf().join("dama/style.css");
    let home_css_file = base_dirs.home_dir().to_path_buf().join(".dama/style.css");
    let css_path = [dot_config_css_file, home_css_file]
        .iter()
        .find_map(move |path| 
            if path.is_file() {
                path.clone().into_os_string().into_string().ok()
            }  else {None}
    );

    (config_buffer, css_path)
}



pub fn execute_shell_command(command: String) {
    match std::process::Command::new("sh").arg("-c")
        .arg(command).output() {
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
        output => output.split('\n').next()
            .map(|line| match line.parse() {
                     Ok(val) => Some(val), 
                     Err(e) => {eprint!("{:?}",e);None}
                 })
            .flatten().unwrap_or(default)
    }
}

