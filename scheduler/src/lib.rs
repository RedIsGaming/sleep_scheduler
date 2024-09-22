pub mod process;

pub mod commands {
    use std::{env::consts, io, process::{Child, Command}};
    use chrono::TimeDelta;
    
    pub fn task_command(task: String, shutdown: TimeDelta) -> Result<Child, io::Error> {
        match consts::OS {
            "linux" | "macos" => {
                Command::new("sh").args(["-c", &format!("echo {}", task)]).spawn()?;
                Ok(Command::new("sudo").args(["-c", &format!("shutdown -h +{}", shutdown.num_minutes())]).spawn()?)
            }
            "windows" => {
                Command::new("cmd").args(["/c", &format!("echo {}", task)]).spawn()?;
                Ok(Command::new("cmd").args(["/c", &format!("shutdown -s -t {}", shutdown.num_seconds())]).spawn()?)
            }
            _ => panic!("Couldn't find an implementation for OS to run shutdown commands: [Linux, MacOS, Windows].")
        }
    }

    pub fn task_clear() -> Result<Child, io::Error> {
        match consts::OS {
            "linux" | "macos" => Ok(Command::new("sh").args(["-c", "clear"]).spawn()?),
            "windows" => Ok(Command::new("cmd").args(["/c", "cls"]).spawn()?),
            _ => panic!("Couldn't find an implementation for OS to run clear commands: [Linux, MacOS, Windows].")
        }
    }
}
