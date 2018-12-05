extern crate libc;
extern crate dirs;

mod native;
mod current_state;

use current_state::CurrentState;
use dirs::cache_dir;

use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

use std::mem::size_of;

use native::open_file;

fn main() {
    let mut path = cache_dir().expect("Failed to request current user's cache directory");
    path.push("mmap");
    let file = OpenOptions::new().read(true).open(&path).expect(
        "Failed to create the temporary file",
    );
    let size = size_of::<CurrentState>();
    let mapping = open_file::<CurrentState>(file.as_raw_fd(), size).expect("Failed to map file");
    let state: CurrentState = unsafe { mapping.read() };
    println!("{:?}", state);
}
