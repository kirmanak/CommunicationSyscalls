extern crate libc;

mod native;
mod current_state;

use current_state::CurrentState;

use native::{get_queue, get_message};

fn main() {
    let queue_id = get_queue().expect("Fail to get the queue");
    let state: CurrentState = get_message(queue_id).expect("Fail to read a message from the queue");
    println!("{:?}", state);
}
