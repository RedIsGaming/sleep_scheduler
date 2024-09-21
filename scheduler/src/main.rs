use std::{process::Child, sync::Arc, thread, time::Duration};
use scheduler::commands;

#[allow(unused)]
#[derive(Debug, Default, Clone, Copy)]
enum ProcessState {
    #[default] Waiting,
    Ready,
    Running,
    Completed,
}

#[allow(unused)]
#[derive(Debug, Clone)]
struct TaskScheduler {
    name: String,
    state: ProcessState,
    job: fn() -> Child,
    time: Time,
}

impl TaskScheduler {
    fn new(name: String, state: ProcessState, job: fn() -> Child, time: Time) -> Self {
        Self {
            name,
            state,
            job,
            time
        }
    }

    fn run(tasks: &Arc<Vec<TaskScheduler>>) {
        for task in tasks.iter() {
            (task.job)();
        }

        let delay = <Vec<TaskScheduler> as Clone>::clone(tasks)
            .into_iter()
            .map(|x| x.time.delay);

        thread::sleep(delay.max().unwrap());
    }
}

#[allow(unused)]
#[derive(Debug, Default, Clone)]
struct Time {
    period: u32,
    delay: Duration,
}

impl Time {
    fn new(period: u32, delay: Duration) -> Self {
        Self {
            period,
            delay,
        }
    }
}

fn print_task1() -> Child {
    commands::task_command("Task 1 is alive :D!")
}

fn print_task2() -> Child {
    commands::task_command("Task 2 is alive :D!")
}

fn print_task3() -> Child {
    commands::task_command("Task 3 is alive :D!")
}

fn main() -> ! {
    let task1 = TaskScheduler::new(
        "Task1".to_owned(), 
        ProcessState::Waiting, 
        print_task1, 
        Time::new(5000, Duration::from_millis(1000))
    );

    let task2 = TaskScheduler::new(
        "Task2".to_owned(), 
        ProcessState::Waiting, 
        print_task2, 
        Time::new(5000, Duration::from_millis(500))
    );

    let task3 = TaskScheduler::new(
        "Task3".to_owned(), 
        ProcessState::Waiting, 
        print_task3, 
        Time::new(5000, Duration::from_millis(750))
    );

    let tasks: Arc<Vec<TaskScheduler>> = Arc::new(vec![task1, task2, task3]);
    commands::task_clear();

    loop {
        let task = Arc::clone(&tasks);
        
        thread::spawn(move || {
            TaskScheduler::run(&task);
        }).join().unwrap();
    }
}
