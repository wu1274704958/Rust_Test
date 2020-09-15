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


#[cfg(test)]
mod tests_simple {
    use crate::Simple::{encode, decode};
    use std::ffi::CStr;

    #[test]
    fn it_works() {
        let arr = vec![1,255,128u8,0];
        unsafe {
            let res = encode(&String::from_utf8_unchecked(arr).as_str());
            let r_arr = res.as_bytes();

            assert_eq!(r_arr[0],2);
            assert_eq!(r_arr[1],254);
            assert_eq!(r_arr[2],129);
            assert_eq!(r_arr[3],255);
        }
    }

    #[test]
    fn it_work2() {
        let arr = vec![255,56,0,0u8];
        unsafe {
            let res = encode(&String::from_utf8_unchecked(arr).as_str());
            let r_arr:Vec<u8> = res.as_bytes().into();

            assert_eq!(r_arr[0],0);
            assert_eq!(r_arr[1],55);
            assert_eq!(r_arr[2],1);
            assert_eq!(r_arr[3],255);

            let res = decode(&String::from_utf8_unchecked(r_arr).as_str());
            let r_arr = res.as_bytes();

            assert_eq!(r_arr[0],255);
            assert_eq!(r_arr[1],56);
            assert_eq!(r_arr[2],0);
            assert_eq!(r_arr[3],0u8);
        }
    }
}

