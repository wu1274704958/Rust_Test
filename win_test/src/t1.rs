
use std::vec::Vec;

use syslv::SysLv;

use nbez::{BezCurve, BezChain, Bez3o, Point2d};

const W :u32 = 1920;
const H :u32 = 1080;

pub fn test(){
    let sys_lv = SysLv::new();

    let w = W as f32;
    let h = H as f32;
    let w_half =  w / 2.0;
    let h_half = h / 2.0;


    for i in 0..sys_lv.size(){
        sys_lv.set_item_pos(i,-30,-30);
    }

    let p1 = Point2d::new(w_half, h_half * 0.6);
    let p1_ctrl = Point2d::new( p1.x - w * 0.2 , p1.y - h * 0.4);

    let p2 = Point2d::new(w_half *  0.3, h_half * 0.3);
    let p2_ctrl = Point2d::new( p2.x + w * 0.001 , p2.y + h * 0.2);

    let p3 = Point2d::new(p2.x,p2.y);
    let p3_ctrl = Point2d::new( p3.x - w * 0.2 , p3.y + h * 0.3);

    let p4 = Point2d::new(w_half * 0.86 , h * 0.9);
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
    points.reserve(40);

    for curve in curve_chain.iter() {
        let mut t = 0.0f32;
        let zl = 1f32 / 9.8f32;
        for _i in 0..10{
            let temp = curve.interp(t).unwrap();
            // println!("{:?}",temp);
            points.push(temp);
            _a += 1;
            t += zl;
        }
    }

    println!("size = {} ",points.len());

    for i in 0..points.len(){
        sys_lv.set_item_pos(i as u32,points[i].x as i32 ,points[i].y as i32);
    }
    let b = points.len();
    for i in 0..points.len(){
        sys_lv.set_item_pos((i + b) as u32,(w_half + (w_half - points[i].x)) as i32 ,points[i].y as i32);
    }
}