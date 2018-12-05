extern crate libc;

mod native;
mod current_state;

use std::thread::sleep;
use std::time::Duration;
use current_state::CurrentState;
use std::mem::size_of;

use native::shared::*;

fn main() {
    let duration = Duration::from_secs(1);
    let memory_id =
        allocate_mem(size_of::<CurrentState>()).expect("Failed to allocate shared memory");
    let shared_mem = attach_mem(memory_id).expect(
        "Failed to attach the allocated shared memory",
    ) as *mut CurrentState;
    let mut state = CurrentState::new();
    loop {
        unsafe {
            shared_mem.write(state.clone());
        }
        sleep(duration);
        state.update();
    }
}
