extern crate libc;

mod native;
mod current_state;

use std::thread::sleep;
use std::time::Duration;
use current_state::CurrentState;

use native::queue::{get_queue, send_message};

fn main() {
    let duration = Duration::from_secs(1);
    let queue_id = get_queue().expect("Fail to get the queue");
    let mut state = Box::new(CurrentState::new());
    loop {
        send_message(queue_id, state.clone()).expect("Fail to write a message to the queue");
        sleep(duration);
        state.update();
    }
}
