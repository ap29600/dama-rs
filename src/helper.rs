use std::{fmt::Debug, str::FromStr};
use std::process::{Command, Output};


pub fn get_configuration() -> (Option<String>, Option<String>) {
    let base_dirs = directories::BaseDirs::new().unwrap();

    let dot_config_main_file = base_dirs.config_dir().to_path_buf().join("dama/config");
    let home_main_file = base_dirs.home_dir().to_path_buf().join(".dama/config");

    let dot_config_css_file = base_dirs.config_dir().to_path_buf().join("dama/style.css");
    let home_css_file = base_dirs.home_dir().to_path_buf().join(".dama/style.css");

    let config_path = [dot_config_main_file, home_main_file]
        .iter().find_map(|path| 
                         if path.is_file() { 
                             path.clone().into_os_string().into_string().ok()
                         } else { None });

    let css_path = [dot_config_css_file, home_css_file]
        .iter().find_map(|path| 
                         if path.is_file() {
                             path.clone().into_os_string().into_string().ok()
                         }  else {None});

    (config_path, css_path)
}



pub fn execute_shell_command(command: String) -> bool {
    match std::process::Command::new("sh").arg("-c")
        .arg(command).output() {
            Ok(output) => output.status.success(),
            Err(e) => { eprint!("{}", e); false }
        }
}


pub fn read_stdout_from_command (command: &str) -> String {
    match Command::new("sh").arg("-c").arg(command.to_string()).output() {
        Err(e) => { println!("error while running command: {}", e); "".to_string() },
        Ok(Output{status: _ , stdout: utf_8_vec, stderr: _}) => 
            match String::from_utf8(utf_8_vec)
            {
                Err(e) => { println!("error while making a string: {}", e); "".to_string()},
                Ok(string) => string
            }
    }
}


pub fn read_value_from_command <T> (command: &str, default: T) -> T 
    where T: std::str::FromStr, <T as FromStr>::Err: Debug {
    match &*read_stdout_from_command(command) {
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

