pub mod monitor;
use serde::Serialize;
use std::sync::RwLock;
#[derive(Debug, Default, Serialize, Clone)]
pub struct Monitor {
    pub current_running_processes: usize,
    pub total_finished_process: usize,
}

lazy_static::lazy_static! {
    pub static ref MONITOR: RwLock<Monitor> = RwLock::new(Monitor::default());
}
