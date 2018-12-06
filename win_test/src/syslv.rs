use winapi::um::winuser::*;
use winapi::shared::windef::HWND;
use std::ffi::{ CString};

pub struct SysLv{
    hwnd : HWND,
    item_num : u32
}

const LVM_FIRST:u32 = 0x1000;

macro_rules! MAKE_LPARAM {
    ($l:ident,$h:ident) => {
        {
            let a:i32 = std::mem::transmute($l );
            let b:i32 = std::mem::transmute($h );
            (a  & 0xffff | (b & 0xffff) << 16)
        }
    };
}

impl SysLv {
    pub fn new() -> SysLv{
        let h = SysLv::find_hwnd();
        SysLv{hwnd : h , item_num : SysLv::ListView_GetItemCount(h) }
    }
    pub fn size(&self) -> u32 {
        self.item_num
    }

    fn ListView_GetItemCount(hwnd:HWND) ->u32{
        unsafe { SendMessageA(hwnd,LVM_FIRST + 4u32,0,0) as u32 }
    }
    pub fn set_item_pos(&self,index:u32,x:i32,y:i32){
        unsafe { SendMessageA(self.hwnd,LVM_FIRST + 15u32,index as usize,MAKE_LPARAM!(x,y) as isize) };
    }

    fn find_hwnd() -> HWND{
        unsafe {
            let a = CString::new("Progman").unwrap();
            let b = CString::new("Program Manager").unwrap();
            let progman: HWND = FindWindowA(a.as_ptr() as *const i8,  b.as_ptr() as *const i8);

            let c = CString::new("SHELLDLL_DefView").unwrap();
            let def_view: HWND = FindWindowExA(progman, 0 as HWND, c.as_ptr() as *const i8, 0 as *const i8);
            let d = CString::new("SysListView32").unwrap();
            let e = CString::new("FolderView").unwrap();
            FindWindowExA(def_view, 0 as HWND, d.as_ptr() as *const i8,  e.as_ptr() as *const i8)
        }
    }
}