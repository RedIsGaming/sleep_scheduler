use std::time::Duration;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ProcessState {
    #[default] Waiting,
    Ready,
    Running,
}

#[derive(Debug, Default, Clone)]
pub struct Time {
    pub period: u32,
    pub delay: Duration,
    pub count: u32,
    pub utc: String,
}

impl Time {
    pub fn new(period: u32, delay: Duration, count: u32, utc: String) -> Self {
        Self {
            period,
            delay,
            count,
            utc
        }
    }
}
