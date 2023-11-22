use std::io::Write;
use std::path::Path;

mod sixel;

#[no_mangle]
pub extern "C" fn encode_to(
    filepath: *const libc::c_char,
    width: u32,
    height: u32,
) -> *const libc::c_char {
    let path = unsafe { std::ffi::CStr::from_ptr(filepath).to_str().unwrap() };
    let path = Path::new(path);

    let size = (width, height);
    let result = sixel::show(path, size).unwrap();

    return std::ffi::CString::new(result).unwrap().into_raw();
}
