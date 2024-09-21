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
        (tasks.job)();
        tasks.time.count = tasks.time.delay.as_secs_f32() as u32;

        thread::sleep(tasks.time.delay);

        if tasks.state.ne(&ProcessState::Ready) { 
            return tasks;
        }

        tasks.time.count = tasks.time.period;
        tasks.state = ProcessState::Waiting;
        tasks
    }

    fn handler(tasks: &mut TaskScheduler) -> &TaskScheduler {
        if tasks.state.ne(&ProcessState::Waiting) {
            return tasks;
        }

        if tasks.time.count.ne(&0) {
            tasks.time.count -= 1;
        }

        tasks.state = ProcessState::Ready;
        tasks.time.count = tasks.time.period;
        tasks
    }
}

pub trait TaskTime {
    fn new(period: u32, delay: Duration, count: u32) -> Self;
}

impl TaskTime for Time {
    fn new(period: u32, delay: Duration, count: u32) -> Self {
        Self {
            period,
            delay,
            count
        }
    }
}

fn print_task() -> Child {
    commands::task_command("Shutdown task is alive :D!")
}

fn main() -> ! {
    let task = TaskScheduler::new(
        "Shutdown".to_owned(), 
        ProcessState::Waiting, 
        print_task, 
        Time::new(1000, Duration::from_secs(5), 0)
    );

    let tasks: Arc<Mutex<[TaskScheduler; 1]>> = Arc::new(Mutex::new([task; 1]));
    commands::task_clear();

    loop {
        let clone_task = Arc::clone(&tasks);
        
        thread::spawn(move || {
            let mut lock_task = clone_task.lock().unwrap();
            
            for task in lock_task.iter_mut() {
                TaskScheduler::run(task);
                TaskScheduler::handler(task);
                println!("{:?}\n", task);
            }
        }).join().unwrap();
    }
}
