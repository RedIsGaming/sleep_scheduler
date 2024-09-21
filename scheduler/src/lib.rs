pub mod process;

pub mod commands {
    use std::{env::consts, process::{Child, Command}};
    
    pub fn task_command(task: &str) -> Child {
        if consts::OS == "linux" || consts::OS == "macos" {
            let command = Command::new("sh")
                .args(["-c", &format!("echo {}", task)])
                .spawn()
                .expect("Couldn't find the echo command");

            return command
        }

        Command::new("cmd").args(["/c", &format!("echo {}", task)]).spawn().expect("Couldn't find the echo command")
    }

    pub fn task_clear() -> Child {
        if consts::OS == "linux" || consts::OS == "macos" {
            let command = Command::new("sh")
                .args(["-c", "clear"])
                .spawn()
                .expect("Couldn't find the clear command");

            return command
        }

        Command::new("cmd").args(["/c", "cls"]).spawn().expect("Couldn't find the cls command")
    }
}
