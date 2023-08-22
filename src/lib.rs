use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn coitusinterruptus(ptr: *const i8) {
    println!("{}", unsafe { CStr::from_ptr(ptr).to_str().unwrap() });
}

#[no_mangle]
pub extern "C" fn nums(x: f64) -> *const i8 {
    let c_str = CString::new(x.to_string()).unwrap();
    let ptr = c_str.as_ptr();
    ptr
}

#[no_mangle]
pub extern "C" fn exit(code: f64) -> ! {
    // RedditLang uses f64's internally, must convert to i32
    std::process::exit(code as i32)
}
