use std::{any::Any, sync::{Arc, Mutex}, thread, time::Duration};
use scheduler::{commands, process::*};
use chrono::{format::{DelayedFormat, StrftimeItems}, Duration as UtcDuration, NaiveTime, Utc};

#[derive(Debug, Clone)]
pub struct TaskScheduler {
    pub state: ProcessState,
    pub job: fn(&TaskScheduler),
    pub time: Time,
}

impl TaskScheduler {
    fn new(state: ProcessState, job: fn(&TaskScheduler), time: Time) -> Self {
        Self {
            state,
            job,
            time
        }
    }

    fn run(&mut self) -> &TaskScheduler {
        (self.job)(self);
        self.time.count = self.time.delay.as_secs_f32() as u32;

        if self.state.ne(&ProcessState::Ready) { 
            return self;
        }

        self.time.count = self.time.period;
        self.state = ProcessState::Waiting;
        self
    }

    fn handler(&mut self) -> &TaskScheduler {
        if self.state.ne(&ProcessState::Waiting) {
            return self;
        }

        if self.time.count.ne(&0) {
            self.time.count -= 1;
        }

        self.state = ProcessState::Ready;
        self.time.count = self.time.period;
        self
    }

    fn print_task(&self) {
        let convert = convert_time(&self.time.utc);
        let time_difference = convert - convert_time(sys_time().to_string().as_str());
        let shutdown = vec![time_difference];
        
        shutdown.into_iter().filter(|x| x.num_seconds().ge(&0)).for_each(|x| {
            commands::task_command(
                format!("Goodbye :D. Shutdown task planned on: {} - {}.", sys_time(), convert), x
            ).unwrap();
        });
    }
}

fn sys_time() -> DelayedFormat<StrftimeItems<'static>> {
    (Utc::now() + UtcDuration::hours(2)).format("%H:%M:%S")
}

fn convert_time(utc: &str) -> NaiveTime {
    NaiveTime::parse_from_str(utc, "%H:%M:%S").unwrap_or_default()
}

fn main() -> Result<(), Box<dyn Any + Send>> {
    let task = TaskScheduler::new(
        ProcessState::Waiting, 
        TaskScheduler::print_task, 
        Time::new(500, Duration::from_secs(30), 0, "23:00:00".to_owned())
    );

    let tasks: Arc<Mutex<[TaskScheduler; 1]>> = Arc::new(Mutex::new([task; 1]));
    commands::task_clear().unwrap();
    
    loop {
        let clone_task = Arc::clone(&tasks);

        thread::spawn(move || {
            let mut lock_task = clone_task.lock().unwrap();
            
            for task in lock_task.iter_mut() {
                task.run();
                task.handler();
                thread::sleep(task.time.delay);
            }
        }).join().unwrap();
    }
}
