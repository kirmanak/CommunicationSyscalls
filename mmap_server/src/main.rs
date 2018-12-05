extern crate libc;
extern crate dirs;
extern crate simple_signal;

mod native;
mod current_state;

use simple_signal::{set_handler, Signal};

use std::thread::sleep;
use std::time::Duration;
use current_state::CurrentState;
use dirs::cache_dir;

use std::process::exit;

use std::fs::{OpenOptions, remove_file};
use std::os::unix::io::AsRawFd;

use std::mem::size_of;

use native::memory_mapped::create_file;

fn main() {
    let duration = Duration::from_secs(1);
    let mut path = cache_dir().expect("Failed to request current user's cache directory");
    path.push("mmap");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&path)
        .expect("Failed to create the temporary file");
    let size = size_of::<CurrentState>();
    file.set_len(size as u64).expect(
        "Failed to reserve enough space",
    );
    let mapping = create_file::<CurrentState>(file.as_raw_fd(), size).expect(
        "Failed to map file",
    );
    let mut state = CurrentState::new();
    set_handler(&[Signal::Int, Signal::Term], move |_| {
        remove_file(&path).expect("Failed to remove the temporary file");
        exit(0);
    });
    loop {
        unsafe {
            mapping.write(state.clone());
        }
        sleep(duration);
        state.update();
    }
}
