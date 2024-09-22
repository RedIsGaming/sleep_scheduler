use std::time::Duration;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ProcessState {
    #[default] Waiting,
    Ready,
    Running,
}

#[derive(Debug, Clone)]
pub struct TaskScheduler {
    pub name: String,
    pub state: ProcessState,
    pub job: fn(&TaskScheduler),
    pub time: Time,
}

#[derive(Debug, Default, Clone)]
pub struct Time {
    pub period: u32,
    pub delay: Duration,
    pub count: u32,
    pub utc: String,
}
