extern crate winapi;
extern crate nbez;
use winapi::um::winuser::*;
use winapi::shared::windef::HWND;
use std::vec::Vec;
use std::ffi::{ CString};

struct SysLv{
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

use nbez::{BezCurve, BezChain, Bez3o, Point2d};

const W :u32 = 1366;
const H :u32 = 700;

fn main(){
    let sys_lv = SysLv::new();
    let w_half =  (W / 2) as f32;



    for i in 0..sys_lv.size(){
        sys_lv.set_item_pos(i,-30,-30);
    }


    let curve: Bez3o<f32> = Bez3o::new(
        Point2d::new(  w_half    ,  200f32),
        Point2d::new(   w_half - 100f32   , 200f32 - 420f32),
        Point2d::new( 200f32 - 200f32 ,   300f32 - 100f32),
        Point2d::new(  200f32,   300f32),
    );

    let curve2: Bez3o<f32> = Bez3o::new(
        Point2d::new(  160f32    ,  330f32),
        Point2d::new(   160f32 - 0f32    , 320f32 + 160f32 ),
        Point2d::new( w_half - 100f32   ,   700f32 - 100f32 ),
        Point2d::new(  w_half  ,   700f32),
    );

    let curve3: Bez3o<f32> = Bez3o::new(
        Point2d::new(  w_half    ,  700f32),
        Point2d::new(   w_half + 100f32   ,   700f32 - 100f32  ),
        Point2d::new( W as f32 - 100f32 - 0f32    , 320f32 + 160f32 ),
        Point2d::new(  W as f32 - 100f32     ,  330f32),
    );

    let curve4: Bez3o<f32> = Bez3o::new(
        Point2d::new(   W as f32 - 140f32,   280f32),
        Point2d::new(   W as f32 - 140f32 + 20f32  ,   280f32 - 180f32),
        Point2d::new( w_half + 100f32 ,   200f32 - 420f32),
        Point2d::new( w_half    ,  200f32),
    );

    let mut curve_chain: BezChain<f32, Bez3o<f32>, Vec<Point2d<f32>>> = BezChain::from_container(vec![
        curve.start,
        curve.ctrl0,
        curve.ctrl1,
        curve2.start,
        curve2.ctrl0,
        curve2.ctrl1,
        curve3.start,
        curve3.ctrl0,
        curve3.ctrl1,
        curve4.start,
        curve4.ctrl0,
        curve4.ctrl1,
        curve4.end
    ]);

    let mut points = Vec::new();
    points.reserve(40);

    let mut a = 0;
    for curve in curve_chain.iter() {
        let mut t = 0.0f32;
        let zl = 1f32 / 9.8f32;
        for i in 0..10{
            let temp = curve.interp(t).unwrap();
           // println!("{:?}",temp);
            points.push(temp);
            a += 1;
            t += zl;
        }
    }

    for i in 0..points.len(){
        sys_lv.set_item_pos(i as u32,points[i].x as i32 ,points[i].y as i32);
    }
}
