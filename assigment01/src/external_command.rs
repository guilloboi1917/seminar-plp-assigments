use std::process::{Command, Output};

pub fn show_sysinfo() {
    let os = std::env::consts::OS;
    println!("You are running {}\n", os);
    // Make it work for linux, mac and windows
    let output: Output = if os == "linux" || os == "macos" {
        println!("Executing \"uname -a\"");
        Command::new("uname")
            .arg("-a")
            .output()
            .expect("Failed to run command uname -a")
    } else {
        println!("Executing \"systeminfo\"");
        Command::new("systeminfo")
            .output()
            .expect("Failed to run command systeminfo")
    };

    let stdout_string = String::from_utf8(output.stdout).unwrap();
    let stdout_array: Vec<&str> = stdout_string.split("\n").collect();

    println!("Command stdout: \n");

    for (i, line) in stdout_array.iter().enumerate() {
        if i >= 10 {
            break;
        }
        println!("({}) - {}", i, line);
    }
}
