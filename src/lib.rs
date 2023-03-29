pub mod monitor;
use serde::Serialize;
#[derive(Debug, Default, Serialize, Clone)]
pub struct Monitor {
    pub current_running_processes: usize,
    pub total_finished_process: usize,
}
