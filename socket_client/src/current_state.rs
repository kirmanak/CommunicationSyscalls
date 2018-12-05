use std::fmt;

/// Represents the current state of the server
#[derive(Clone, Serialize, Deserialize)]
pub struct CurrentState {
    pid: u32,
    uid: u32,
    gid: u32,
    start_time: i64,
    running_time: i64,
    loadavg_1: f64,
    loadavg_5: f64,
    loadavg_15: f64,
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
