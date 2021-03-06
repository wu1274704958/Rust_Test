use winapi::um::winuser::*;
use winapi::um::winnt::*;
use winapi::um::processthreadsapi::{ OpenProcess };
use winapi::um::memoryapi::{ VirtualAllocEx
                             ,ReadProcessMemory
                             ,WriteProcessMemory
                            ,VirtualFreeEx
};
use winapi::um::handleapi::CloseHandle;
use winapi::um::commctrl::{
    LVM_GETITEMCOUNT,
    LVM_SETITEMPOSITION,
    LVM_GETITEMPOSITION,
    LVM_GETITEMRECT
};

use winapi::shared::minwindef::*;

use winapi::shared::windef::{
    HWND,
    POINT,
    RECT
};


use std::ffi::{ CString};
use core::ops::Drop;
use core::marker::{Sized,Copy};
use std::mem::{ size_of,transmute};

pub struct ProcessVM{
    size:usize,
    ptr:PVOID,
    hProcess:HANDLE
}

impl ProcessVM {
    pub fn new(size:usize,hwnd:HWND)->ProcessVM
    {
        unsafe {
            let dwProcessId:DWORD = 0;
            GetWindowThreadProcessId(hwnd,&dwProcessId as *const DWORD as *mut DWORD);
            let hProcess:HANDLE = OpenProcess(PROCESS_VM_READ | PROCESS_VM_WRITE | PROCESS_VM_OPERATION,
                                                FALSE,dwProcessId);
            let ptr:PVOID = VirtualAllocEx(hProcess,0 as LPVOID,size ,MEM_COMMIT,PAGE_READWRITE);
            ProcessVM{size,ptr,hProcess}
        }
    }
    pub fn write<T>(&self,t: T) -> bool
    where T : Sized + Copy
    {
        if size_of::<T>() > self.size {return false};
        unsafe {
            TRUE == WriteProcessMemory(self.hProcess,self.ptr,transmute(&t),size_of::<T>(),0 as *mut usize)
        }
    }
    pub fn read<T>(&self,t: &mut T) -> bool
        where T : Sized + Copy
    {
        if size_of::<T>() > self.size {return false};
        unsafe {
            TRUE == ReadProcessMemory(self.hProcess,self.ptr,transmute(t),size_of::<T>(),0 as *mut usize)
        }
    }
}

impl Drop for ProcessVM{
    fn drop(&mut self) {
        unsafe {
            VirtualFreeEx(self.hProcess,self.ptr,self.size,MEM_RELEASE);
            CloseHandle(self.hProcess);
        }
    }
}

use std::collections::HashMap;
use std::cell::RefCell;
use std::result::Result;

pub struct SysLv{
    hwnd            : HWND,
    item_num        : u32,
    processVM       : ProcessVM,
    pub W           : u32,
    pub H           : u32,
    offsets          : RefCell<HashMap<u32,(i16,i16,u16,u16)>>
}


macro_rules! MAKE_LPARAM {
    ($l:expr,$h:expr) => {
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
        let rect = unsafe {
            let res = RECT { left: 0, right: 0, top: 0, bottom: 0 };
            GetWindowRect(h, &res as *const _ as *mut _);
            res
        };

        SysLv{hwnd : h ,
            item_num : SysLv::ListView_GetItemCount(h) ,
            processVM : ProcessVM::new(512usize,h) ,
            W : (rect.right - rect.left) as u32,
            H : (rect.bottom - rect.top) as u32,
            offsets : RefCell::new(HashMap::new())
        }
    }
    pub fn size(&self) -> u32 {
        self.item_num
    }

    fn ListView_GetItemCount(hwnd:HWND) ->u32{
        unsafe { SendMessageA(hwnd,LVM_GETITEMCOUNT,0,0) as u32 }
    }
    pub fn set_item_pos(&self,index:usize,x:i32,y:i32){
        if index >= self.item_num as usize { return; }
        unsafe { SendMessageA(self.hwnd,LVM_SETITEMPOSITION,index,MAKE_LPARAM!(x,y) as isize) };
    }
    pub fn get_item_pos(&self,index:usize) -> Result<POINT,&'static str>
    {
        if index >= self.item_num as usize { return Result::Err("Out of bound!!!"); }
        let mut ret = POINT{x:0,y:0};

        unsafe {
           SendMessageA(self.hwnd,LVM_GETITEMPOSITION,index,transmute( self.processVM.ptr) );
        };
        if !self.processVM.read(&mut ret) {
            return Result::Err("Read Process Memory Failed!!!");
        }
        Result::Ok(ret)
    }

    pub fn set_item_pos_center(&self,index:usize,x:i32,y:i32)->Result<(),&'static str> {
        if index >= self.item_num as usize { return Result::Err("Out of bound!!!"); }
        let mut offsets = self.offsets.borrow_mut();
        let offset:(i16,i16,u16,u16) = {
            if offsets.contains_key(&(index as u32)) {
                offsets.get(&(index as u32)).unwrap().clone()
            }else{
                let rect = self.get_item_rect(index,0)?;
                let pos = self.get_item_pos(index)?;
                let half_w = (rect.right - rect.left).abs() / 2;
                let half_h = (rect.bottom - rect.top).abs() / 2;
                let offsetx = if pos.x < 0 && rect.left < 0 {
                    rect.left.abs() - pos.x.abs()
                }else{
                    pos.x - rect.left
                };
                let offsety = if pos.y < 0 && rect.top < 0 {
                    rect.top.abs() - pos.y.abs()
                }else{
                    pos.y - rect.top
                };
                let calc_ = (offsetx as i16,offsety as i16,half_w as u16,half_h as u16);
                offsets.insert(index as u32,calc_.clone());
                calc_
            }
        };

//        println!("rect = {} {} {} {}",rect.left,rect.top,rect.right,rect.bottom);
//        println!("pos = {} {} ",pos.x,pos.y);
//        println!("{} {} ",offsetx,offsety);
//        unsafe { SendMessageA(self.hwnd,LVM_SETITEMPOSITION,index,MAKE_LPARAM!(x - half_w + offsetx , y - half_h + offsety) as isize) };
        unsafe { SendMessageA(self.hwnd,LVM_SETITEMPOSITION,index,MAKE_LPARAM!(x - offset.2 as i32 + offset.0 as i32, y - offset.3 as i32 + offset.1 as i32) as isize) };
        Result::Ok(())
    }

    pub fn get_item_pos_center(&self,index:usize)->Result<POINT,&'static str> {
        if index >= self.item_num as usize { return Result::Err("Out of bound!!!"); }
        let mut offsets = self.offsets.borrow_mut();
        let offset:(i16,i16,u16,u16) = {
            if offsets.contains_key(&(index as u32)) {
                offsets.get(&(index as u32)).unwrap().clone()
            }else{
                let rect = self.get_item_rect(index,0)?;
                let pos = self.get_item_pos(index)?;
                let half_w = (rect.right - rect.left).abs() / 2;
                let half_h = (rect.bottom - rect.top).abs() / 2;
                let offsetx = if pos.x < 0 && rect.left < 0 {
                    rect.left.abs() - pos.x.abs()
                }else{
                    pos.x - rect.left
                };
                let offsety = if pos.y < 0 && rect.top < 0 {
                    rect.top.abs() - pos.y.abs()
                }else{
                    pos.y - rect.top
                };
                let calc_ = (offsetx as i16,offsety as i16,half_w as u16,half_h as u16);
                offsets.insert(index as u32,calc_.clone());
                calc_
            }
        };
        let pos = self.get_item_pos(index)?;
        Result::Ok(POINT{x: pos.x - offset.0 as i32 + offset.2 as i32,y: pos.y - offset.1 as i32 + offset.3 as i32})
    }

    pub fn get_item_rect(&self,index:usize,r#type:u32) -> Result<RECT,&'static str>
    {
        if index >= self.item_num as usize { return Result::Err("Out of bound!!!"); }
        let mut ret: RECT = RECT{left:r#type as i32,right:0,top:0,bottom:0};
        if !self.processVM.write(ret) {
            return Result::Err("Write Process Memory Failed!!!");
        }
        unsafe {
            SendMessageA(self.hwnd,LVM_GETITEMRECT,index,transmute( self.processVM.ptr) );
        };
        if !self.processVM.read(&mut ret) {
            return Result::Err("Read Process Memory Failed!!!");
        }
        Result::Ok(ret)
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
    pub fn refresh_num(&mut self)
    {
        self.item_num = SysLv::ListView_GetItemCount(self.hwnd);
    }
    pub fn as_ptr(&self) -> *mut SysLv{
        self as *const _ as *mut _
    }
}

use std::vec::Vec;

pub struct ItemStateStore<'a>
{
    pervious    :Vec<POINT>,
    sys_lv      :&'a SysLv,
    pub is_reduction:bool
}

impl <'a> ItemStateStore<'a>
{
    pub fn new(sys_lv:&'a SysLv) -> ItemStateStore<'a>
    {
        let mut pervious = Vec::new();
        for i in 0..sys_lv.size(){
            pervious.push(sys_lv.get_item_pos(i as usize).ok().unwrap());
        }
        ItemStateStore{pervious,sys_lv,is_reduction:true}
    }

    pub fn get_pervious(&self)-> &Vec<POINT>
    {
        &(self.pervious)
    }

    fn reduction(&self)
    {
        unsafe { (*self.sys_lv.as_ptr()).refresh_num(); }
        for i in 0..self.pervious.len() {
            if i as u32 >= self.sys_lv.size() { break; }
            self.sys_lv.set_item_pos(i, self.pervious[i].x, self.pervious[i].y);
        }
    }
}

impl<'a> Drop for ItemStateStore<'a>
{
    fn drop(&mut self) {
        if self.is_reduction {
            self.reduction();
        }
    }
}