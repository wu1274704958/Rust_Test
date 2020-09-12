extern crate encryption;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::intrinsics::copy;

use encryption::Simple;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



#[no_mangle]
pub fn simple_encode(v:*const c_char,out:*mut c_char)
{
    unsafe {
        let in_str = CStr::from_ptr(v).to_str().unwrap();
        let res = Simple::encode(in_str);
        let data = res.as_bytes();
        copy(data.as_ptr() as *const i8,out as *mut i8,data.len());
    }
}

#[no_mangle]
pub fn simple_decode(v:*const c_char,out:*mut c_char)
{
    unsafe {
        let in_str = CStr::from_ptr(v).to_str().unwrap();
        let res = Simple::decode(in_str);
        let data = res.as_bytes();
        copy(data.as_ptr() as *const i8,out as *mut i8,data.len());
    }
}
