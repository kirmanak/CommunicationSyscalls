use libc::{strncpy, strlen, c_char};

/// Makes a copy of the provided raw string
/// # Panics
/// If the string contains non-unicode characters
pub unsafe fn copy_raw(source: *const c_char) -> String {
    let len = strlen(source);
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
    strncpy(buffer.as_mut_ptr() as *mut c_char, source, len);
    buffer.set_len(len);
    String::from_utf8(buffer).expect("A raw string contains a non-unicode character!")
}
