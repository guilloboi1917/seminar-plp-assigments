use std::process::{Command, Output};

pub fn show_sysinfo() {
    // Make it work for both linux and windows
    let output: Output = if cfg!(target_os = "linux") {
        println!("You are running linux, will execute uname -a");
        Command::new("uname")
            .arg("-a")
            .output()
            .expect("Failed to run command uname -a")
    } else {
        println!("You are running windows, will execute systeminfo");
        Command::new("systeminfo")
            .output()
            .expect("Failed to run command systeminfo")
    };

    let stdout_string = String::from_utf8(output.stdout).unwrap();
    let stdout_array: Vec<&str> = stdout_string.split("\n").collect();

    println!("Command stdout: \n\n");

    for (i, line) in stdout_array.iter().enumerate() {
        if i >= 10 {
            break;
        }
        println!("({}) - {}", i, line);
    }
}
