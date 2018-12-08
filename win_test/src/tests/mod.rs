use winapi::um::winuser::*;
use winapi::um::winnt::*;
use crate::syslv::ProcessVM;
use std::ffi::CString;
use winapi::um::winuser::FindWindowA;
use winapi::um::winuser::FindWindowExA;

use winapi::shared::windef::{
    HWND,
    POINT,
    RECT
};

#[test]
fn test_processVM()
{
    let hwnd = unsafe {
        let a = CString::new("Progman").unwrap();
        let b = CString::new("Program Manager").unwrap();
        let progman: HWND = FindWindowA(a.as_ptr() as *const i8, b.as_ptr() as *const i8);

        let c = CString::new("SHELLDLL_DefView").unwrap();
        let def_view: HWND = FindWindowExA(progman, 0 as HWND, c.as_ptr() as *const i8, 0 as *const i8);
        let d = CString::new("SysListView32").unwrap();
        let e = CString::new("FolderView").unwrap();
        FindWindowExA(def_view, 0 as HWND, d.as_ptr() as *const i8, e.as_ptr() as *const i8)
    };
    let p = ProcessVM::new(60,hwnd);
    let rect = RECT{left:1,right:2,top:3,bottom:4};
    let mut rect2 = RECT{left:0,right:0,top:0,bottom:0};
    p.write(rect);
    p.read(&mut rect2);
    assert_eq!(rect2.left,1);
    assert_eq!(rect2.right,2);
    assert_eq!(rect2.top,3);
    assert_eq!(rect2.bottom,4);
}
