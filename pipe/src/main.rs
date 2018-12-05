extern crate libc;

mod native;

use native::{execute_in_child, create_pipe, as_stdin, execute};

use std::env::args;
use std::path::PathBuf;
use std::io::Write;
use std::fs::{File, read};
use std::os::unix::io::FromRawFd;

fn main() {
    let first = args().next().expect("First argument was not present");
    let path = args().skip(1).next().expect(
        &format!("Usage: {} file_name", first),
    );
    let path = PathBuf::from(path);
    let content = read(path).expect("Unable to read from the file");
    let (read_end, write_end) = create_pipe().expect("Unable to create a pipe");
    {
        let mut write_end = unsafe { File::from_raw_fd(write_end) };
        content.into_iter().step_by(2).for_each(|symbol| {
            write_end.write(&[symbol]).expect(
                "Unable to write symbol to the pipe",
            );
        });
    }
    execute_in_child(|| {
        let read_end = unsafe { File::from_raw_fd(read_end) };
        as_stdin(read_end).expect("Unable to open read_end as stdin");
        execute("wc").expect("Unable to represent program name in C string")
    }).expect("Unable to run actions in child");
}
