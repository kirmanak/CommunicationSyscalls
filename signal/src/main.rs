extern crate libc;
#[macro_use]
extern crate lazy_static;

mod native;
mod current_state;

use std::thread::sleep;
use std::time::Duration;
use current_state::CurrentState;
use std::sync::Mutex;

use native::signals::register_handler;

use libc::{SIGHUP, SIGINT, SIGTERM, SIGUSR1, SIGUSR2};

lazy_static! {
    static ref state: Mutex<CurrentState> = Mutex::new(CurrentState::new());
}

fn main() {
    let duration = Duration::from_secs(1);
    register_handler(SIGHUP, |_| {
        let server_state = state.lock().expect(
            "Unable to lock state in SIGHUP handler",
        );
        println!("{:?}", server_state.pid);
    }).expect("Unable to register handler of HUP signal");
    register_handler(SIGINT, |_| {
        let server_state = state.lock().expect(
            "Unable to lock state in SIGINT handler",
        );
        println!("{:?}", server_state.uid);
    }).expect("Unable to register handler of INT signal");
    register_handler(SIGTERM, |_| {
        let server_state = state.lock().expect(
            "Unable to lock state in SIGTERM handler",
        );
        println!("{:?}", server_state.gid);
    }).expect("Unable to register handler of TERM signal");
    register_handler(SIGUSR1, |_| {
        let server_state = state.lock().expect(
            "Unable to lock state in SIGUSR1 handler",
        );
        println!("{:?}", server_state.running_time);
    }).expect("Unable to register handler of USR1 signal");
    register_handler(SIGUSR2, |_| {
        let server_state = state.lock().expect(
            "Unable to lock state in SIGUSR2 handler",
        );
        println!("{:?}", (
            server_state.loadavg_1,
            server_state.loadavg_5,
            server_state.loadavg_15,
        ));
    }).expect("Unable to register handler of USR2 signal");
    loop {
        sleep(duration);
        let mut server_state = state.lock().expect("Unable to lock state in loop");
        server_state.update();
    }
}
