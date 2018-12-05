use std::fmt;
use native::{get_gid, get_pid, get_uid, get_loadavg, get_time};

/// Represents the current state of the server
#[derive(Clone)]
pub struct CurrentState {
    pub pid: u32,
    pub uid: u32,
    pub gid: u32,
    start_time: i64,
    pub running_time: i64,
    pub loadavg_1: f64,
    pub loadavg_5: f64,
    pub loadavg_15: f64,
}

impl CurrentState {
    /// Creates the initial state of the server
    pub fn new() -> Self {
        let average_load = loadavg();
        CurrentState {
            pid: get_pid(),
            uid: get_uid(),
            gid: get_gid(),
            start_time: get_time(),
            running_time: 0,
            loadavg_1: average_load[0],
            loadavg_5: average_load[1],
            loadavg_15: average_load[2],
        }
    }

    /// Updates variables which are mutable in time
    pub fn update(&mut self) {
        let average_load = loadavg();
        self.loadavg_1 = average_load[0];
        self.loadavg_5 = average_load[1];
        self.loadavg_15 = average_load[2];
        self.running_time = get_time() - self.start_time;
    }
}

fn loadavg() -> Vec<f64> {
    get_loadavg().expect("Fail to request average load")
}

impl fmt::Debug for CurrentState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Current state is: pid = {}, uid = {}, gid = {}, running_time = {}, loadavg = {:?}",
            self.pid,
            self.uid,
            self.gid,
            self.running_time,
            (self.loadavg_1, self.loadavg_5, self.loadavg_15)
        )
    }
}
