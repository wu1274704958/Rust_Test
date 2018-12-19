use nbez::{BezCurve, BezChain, Bez3o, Point2d};
use crate::syslv::{ SysLv,ItemStateStore};
use std::thread::sleep;
use core::time::Duration;
use crate::transform::{ Vec2,Mat2};
use crate::t1::create_item;
use std::fs::remove_file;

const OneByOne:bool = false;

pub fn test()
{
    let tween_vec = {
        let curve: Bez3o<f32> = Bez3o::new(
            Point2d::new(0.0    ,0.0),
            Point2d::new(0.57   ,0.07),
            Point2d::new(0.33    ,1.45),
            Point2d::new(1.0   ,1.0)
        );
        let curve_chain: BezChain<f32, Bez3o<f32>, Vec<Point2d<f32>>> = BezChain::from_container(vec![
            curve.start,
            curve.ctrl0,
            curve.ctrl1,
            curve.end ]);
        let mut res = vec![];
        for curve in curve_chain.iter() {
            let mut t = 0.0f32;
            let zl = 1.0f32 / 40.0f32;

            for _i in 0..40{
                let temp = curve.interp(t).unwrap();
                res.push(Vec2::new(temp.x,temp.y));
                t += zl;
            }
        }
        let last = res.len() - 1;
        res[last].y = 1.0f32;
        res
    };

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
    let p1_ctrl = Point2d::new( p1.x - w * 0.15 , p1.y - h * 0.6);

    let p2 = Point2d::new(w_half *  0.3, h_half * 0.3);
    let p2_ctrl = Point2d::new( p2.x - w * 0.06 , p2.y + h * 0.1);

    let p3 = Point2d::new(p2.x ,p2.y );
    let p3_ctrl = Point2d::new( p3.x - w * 0.2 , p3.y + h * 0.3);

    let p4= Point2d::new(w_half * 1.05  , h);
    let p4_ctrl = Point2d::new( p4.x - w * 0.07 , p4.y + h * 0.05);

    let curve: Bez3o<f32> = Bez3o::new(
        p1,
        p1_ctrl,
        p2_ctrl,
        p2
    );

    let curve2: Bez3o<f32> = Bez3o::new(
        p3,
        p3_ctrl,
        p4_ctrl,
        p4
    );

    let curve_chain: BezChain<f32, Bez3o<f32>, Vec<Point2d<f32>>> = BezChain::from_container(vec![
        curve.start,
        curve.ctrl0,
        curve.ctrl1,

        curve2.start,
        curve2.ctrl0,
        curve2.ctrl1,
        curve2.end
    ]);

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
            t += zl;
        }
    }

    let b = points.len();
    for i in 0..b{
        points.push(Vec2::new(w_half + (w_half - points[i].x)  , points[i].y));
    }

    points.iter_mut().for_each(|it| {
       let mut new = (Mat2::from_scale(Vec2::new(0.8f32,0.8f32)) * (*it - half)) + half;
        *it = new;
    });

    let mut from:Vec<Vec2> = Vec::new();

    for i in 0..sys_lv.size(){
        from.push(sys_lv.get_item_pos_center(i as usize).ok().unwrap().into());
    }

    if !OneByOne {
        for n in tween_vec {
            for i in 0..sys_lv.size() as usize {
                let offset = points[i] - from[i];
                sys_lv.set_item_pos_center(i, (from[i].x + (offset.x * n.y)) as i32, (from[i].y + (offset.y * n.y)) as i32);
            }
        }
    }else{
        for i in 0..sys_lv.size() as usize {
            let offset = points[i] - from[i];
            for n in tween_vec.iter() {
                sys_lv.set_item_pos_center(i, (from[i].x + (offset.x * n.y)) as i32, (from[i].y + (offset.y * n.y)) as i32);
                sleep(Duration::from_millis(9));
            }
        }
    }

    sleep(Duration::from_secs(5));

    fs.iter().for_each(|f|{
        remove_file(f.as_path());
    });
    sleep(Duration::from_secs(2));

}

