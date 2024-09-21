use std::{process::Child, sync::{Arc, Mutex}, thread, time::Duration};
use scheduler::{commands, process::*};

pub trait Scheduler {
    fn new(name: String, state: ProcessState, job: fn() -> Child, time: Time) -> Self;
    fn run(tasks: &mut TaskScheduler) -> &TaskScheduler;
    fn handler(tasks: &mut TaskScheduler) -> &TaskScheduler;
}

impl Scheduler for TaskScheduler {
    fn new(name: String, state: ProcessState, job: fn() -> Child, time: Time) -> Self {
        Self {
            name,
            state,
            job,
            time
        }
    }

    fn run(tasks: &mut TaskScheduler) -> &TaskScheduler {
        thread::sleep(tasks.time.delay);
        (tasks.job)();
        tasks.time.period = tasks.time.delay.as_secs_f32() as u32;

        if tasks.state.ne(&ProcessState::Ready) { 
            return tasks;
        }

        tasks.state = ProcessState::Waiting;
        tasks
    }

    fn handler(tasks: &mut TaskScheduler) -> &TaskScheduler {
        if tasks.state.ne(&ProcessState::Waiting) {
            return tasks;
        }

        if tasks.time.period != 0 {
            tasks.time.period -= 1;
        }

        tasks.state = ProcessState::Ready;
        tasks
    }
}

pub trait TaskTime {
    fn new(period: u32, delay: Duration) -> Self;
}

impl TaskTime for Time {
    fn new(period: u32, delay: Duration) -> Self {
        Self {
            period,
            delay,
        }
    }
}

fn print_task() -> Child {
    commands::task_command("Task 1 is alive :D!")
}

fn main() -> ! {
    let task = TaskScheduler::new(
        "Task1".to_owned(), 
        ProcessState::Waiting, 
        print_task, 
        Time::new(2500, Duration::from_secs(5))
    );

    let tasks: Arc<Mutex<[TaskScheduler; 1]>> = Arc::new(Mutex::new([task; 1]));
    commands::task_clear();

    loop {
        let clone_task = Arc::clone(&tasks);
        
        thread::spawn(move || {
            let lock_task = clone_task.lock().unwrap();
            
            for mut task in lock_task.iter() {
                TaskScheduler::run(&mut task);
                TaskScheduler::handler(&mut task);
            }
        }).join().unwrap();
    }
}
