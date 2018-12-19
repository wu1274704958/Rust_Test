
use std::vec::Vec;

use crate::syslv::{ SysLv,ItemStateStore};
use std::thread::sleep;
use core::time::Duration;

use std::fs::*;
use std::path::*;
use winapi::um::winbase::GetUserNameA;
use winapi::shared::minwindef::*;
use std::ffi::{CStr};
use nbez::{BezCurve, BezChain, Bez3o, Point2d};

const W :u32 = 1366;
const H :u32 = 768;

use crate::transform::{Vec2,Mat2};

fn create_item(n:u32)-> Vec<PathBuf>{
    let un = get_user_name();
    let mut buf = String::new();
    buf.push_str("C:\\Users\\");
    buf.push_str(un.as_str());
    buf.push_str("\\Desktop\\");
    let path = Path::new(buf.as_str());
    let mut res:Vec<PathBuf> = vec![];
    if path.exists() {
        for i in 0..n{
            let pp = path.join(format!("{}.txt",i));
            if let Ok(_) = File::create(pp.as_path()){
                res.push(pp);
            }
        }
    }
    res
}
#[allow(unused_must_use)]
pub fn test(){
    let sys_lv = SysLv::new();

    let _itemStateStore = ItemStateStore::new(&sys_lv);

    let fs = if sys_lv.size() < 60 {
        let fs = create_item(60 - sys_lv.size());
        sleep(Duration::from_secs(3));
        unsafe { (*sys_lv.as_ptr()).refresh_num(); }
        fs
    }else{
        vec![]
    };

    let w = sys_lv.W as f32;
    let h = sys_lv.H as f32;
    let w_half =  w / 2.0;
    let h_half = h / 2.0;


    let p1 = Point2d::new(w_half, h_half * 0.6);
    let p1_ctrl = Point2d::new( p1.x - w * 0.2 , p1.y - h * 0.5);

    let p2 = Point2d::new(w_half *  0.3, h_half * 0.3);
    let p2_ctrl = Point2d::new( p2.x + w * 0.001 , p2.y + h * 0.2);

    let p3 = Point2d::new(p2.x ,p2.y );
    let p3_ctrl = Point2d::new( p3.x - w * 0.15 , p3.y + h * 0.3);

    let p4 = Point2d::new(w_half * 0.76 , h * 0.9 );
    let p4_ctrl = Point2d::new( p4.x + w * 0.15 , p4.y + h * 0.1);

    let curve: Bez3o<f32> = Bez3o::new(
        p1,
        p1_ctrl,
        p2,
        p2_ctrl
    );

    let curve2: Bez3o<f32> = Bez3o::new(
        p3,
        p3_ctrl,
        p4,
        p4_ctrl
    );

//    let curve3: Bez3o<f32> = Bez3o::new(
//        Point2d::new(  w_half    ,  700f32),
//        Point2d::new(   w_half + 100f32   ,   700f32 - 100f32  ),
//        Point2d::new( W as f32 - 100f32 - 0f32    , 320f32 + 160f32 ),
//        Point2d::new(  W as f32 - 100f32     ,  330f32),
//    );
//
//    let curve4: Bez3o<f32> = Bez3o::new(
//        Point2d::new(   W as f32 - 140f32,   280f32),
//        Point2d::new(   W as f32 - 140f32 + 20f32  ,   280f32 - 180f32),
//        Point2d::new( w_half + 100f32 ,   200f32 - 420f32),
//        Point2d::new( w_half    ,  200f32),
//    );

    let curve_chain: BezChain<f32, Bez3o<f32>, Vec<Point2d<f32>>> = BezChain::from_container(vec![
        curve.start,
        curve.ctrl0,
        curve.ctrl1,

        curve2.start,
        curve2.ctrl0,
        curve2.ctrl1,
        curve2.end
//        curve3.start,
//        curve3.ctrl0,
//        curve3.ctrl1,
//        curve4.start,
//        curve4.ctrl0,
//        curve4.ctrl1,
//        curve4.end
    ]);
    let mut _a = 0;

    let mut points = Vec::new();
    points.reserve(60);

    let half = Vec2::new(w_half as f32,h_half as f32);

    for curve in curve_chain.iter() {
        let mut t = 0.0f32;
        let zl = 1f32 / 15.0f32;

        for _i in 0..15{
            let temp = curve.interp(t).unwrap();
            // println!("{:?}",temp);

            points.push( Vec2::new(temp.x,temp.y));
            _a += 1;
            t += zl;
        }
    }

    let b = points.len();
    for i in 0..b{
        points.push(Vec2::new(w_half + (w_half - points[i].x)  , points[i].y));
    }

    println!("size = {}",points.len());
    let mut scale = 1.2f32;
//    let mut angle = 0.0f32;
    let mut points_curr:Vec<Vec2> = Vec::new();
    points_curr.reserve(60);
    loop {
        if scale < 0.8f32 {break;}
        points_curr.clear();
        points.iter().for_each(|it|{
            let one_step = (*it) - half;
            let two_step = Mat2::from_scale(Vec2::new(scale,scale)) * one_step;
            points_curr.push(two_step + half);
        });

        for i in 0..points_curr.len(){
            sys_lv.set_item_pos_center(i ,points_curr[i].x as i32 ,points_curr[i].y as i32);
        }

//        angle += 0.01f32;
        scale -= 0.01f32;
    }



    sleep(Duration::from_secs(5));

    fs.iter().for_each(|f|{
        remove_file(f.as_path());
    });
    sleep(Duration::from_secs(2));
}

fn get_user_name() ->String
{
    let mut name:[u8;256] = [0;256];
    let mut size:DWORD = 256;
    unsafe {
        let _res = GetUserNameA(name.as_mut_ptr() as *mut i8,&mut size as *mut _);
        let mut n:Vec<u8> = Vec::new();
        for i in 0..size{
            n.push(name[i as usize]);
        }
        let cstr = CStr::from_bytes_with_nul(n.as_ref()).unwrap();
        cstr.to_str().unwrap().to_string()
    }
}

