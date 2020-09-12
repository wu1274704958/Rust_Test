pub mod Simple;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::intrinsics::copy;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

