use nbez::{BezCurve, BezChain, Bez3o, Point2d};
use crate::syslv::{ SysLv,ItemStateStore};
use std::thread::sleep;
use core::time::Duration;
use crate::transform::{ Vec2,Mat2};
use crate::t1::create_item;
use std::fs::remove_file;
use std::env;
use std::str::FromStr;
use std::fs::File;
use std::path::{PathBuf,Path};
use std::io::{Write,Read};
use winapi::shared::windef::POINT;

const ONE_BY_ONE:bool = false;

macro_rules! dp {
    ($a:ident) => {
        println!("{} = {:?}",stringify!($a),$a);
    };
}

#[allow(unused_must_use)]
pub fn test()
{
    let mut n_for_time              :u32 = 0;
    let mut is_clean        = false;
    let mut is_auto_remove  = true;
    let mut is_reduction    = true;
    let mut print_help      = false;
    let mut only_del        = false;
    let mut only_reduction  = false;

    let mut stage = 0;
    env::args().for_each(|it|{
        if stage != 0 {
            match stage {
                1 => {
                    n_for_time = if let Some(res) = u32::from_str(it.as_str()).ok() {
                        res
                    } else {
                        panic!("Input Resolution Failure!!! {} ", it)
                    }
                },
                _ => {}
            }
            stage = 0;
        }else{
            match it.as_str() {
                "-n" => stage = 1,
                "-c" => is_clean = true,
                "-r" => is_reduction = false,
                "-a" => is_auto_remove = false,
                "-h" => print_help = true,
                "-od" => only_del = true,
                "-or" => only_reduction = true,
                _ => {}
            }
        }
    });

    if n_for_time == 0 {n_for_time = 3};
    if n_for_time > 60 { println!("Item num must <= 60"); }
    if print_help {
        println!("--------------------------- Help ---------------------------\n\
  选项说明（以下选项都是可选的）
\t-n 后加数字 1 ~ 60 表示一次移动Item的数量（数字是必须的）默认是3。\n\
\t-c 开始前清理所有桌面图标，默认不清理。\n\
\t-r 结束后不回复图标位置，默认恢复。\n\
\t-a 结束后不自动删除临时占位图标，默认自动删除。\n\
\t-h 打印帮助。\n\
\t-od 只删除最近一次自动创建的占位图标。\n\
\t-or 只回复最近一次图标的位置。\n\
  Tip：使用前请关闭“自动排列图标”及“将图标与网格对齐”选项。");
        return;
    }

    dp!(n_for_time    );
    dp!(is_clean      );
    dp!(is_auto_remove);
    dp!(is_reduction  );
    dp!(print_help    );
    dp!(only_del      );
    dp!(only_reduction);

    let config = load_config();

    let sys_lv = SysLv::new();

    if only_reduction && only_del{
        if let Some(config) = config {
            del_temp_item(&config.1);
            sleep(Duration::from_millis(2000));
            reduction( &sys_lv,&config.0);
        }
        return;
    }

    if only_del{
        if let Some(config) = config {
            del_temp_item(&config.1);
        }
        return;
    }

    if only_reduction{
        if let Some(config) = config {
            reduction(&sys_lv,&config.0);
        }
        return;
    }

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


    let fs = if sys_lv.size() < 60 {
        let fs = create_item(60 - sys_lv.size());
        sleep(Duration::from_secs(3));
        unsafe { (*sys_lv.as_ptr()).refresh_num(); }
        fs
    }else{
        vec![]
    };

    let mut _itemStateStore = ItemStateStore::new(&sys_lv);
    _itemStateStore.is_reduction = is_reduction;

    if is_clean{
        for n in 0..sys_lv.size(){
            sys_lv.set_item_pos(n as usize,-100,-100);
        }
    }


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
       let new = (Mat2::from_scale(Vec2::new(0.8f32,0.8f32)) * (*it - half)) + half;
        *it = new;
    });

    let mut from:Vec<Vec2> = Vec::new();

    for i in 0..sys_lv.size(){
        from.push(sys_lv.get_item_pos_center(i as usize).ok().unwrap().into());
    }
    let mut b = 0usize;
    let len = n_for_time as usize;
    if !ONE_BY_ONE {
        'wai:loop {
            if b >= sys_lv.size() as usize { break; }
                for n in tween_vec.iter() {
                    let mut i = 0;
                    loop {
                        if i >= len { break; }
                        let curr = b + i;
                        if curr >= sys_lv.size() as usize { break; }
                        let offset = points[curr] - from[curr];
                        sys_lv.set_item_pos_center(curr, (from[curr].x + (offset.x * n.y)) as i32, (from[curr].y + (offset.y * n.y)) as i32);
                        i += 1;
                    }
                    sleep(Duration::from_millis(9));
                }
            b += len;
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

    sleep(Duration::from_secs(2));

    if is_auto_remove {
        fs.iter().for_each(|f| {
            remove_file(f.as_path());
        });
    }

    save_config(Some(_itemStateStore.get_pervious()),&fs);

    sleep(Duration::from_secs(2));

}

const config_path :&'static str = ".\\config";

pub fn save_config(pos : Option<&Vec<POINT>>,fs :&Vec<PathBuf>)
{
    let mut confug_file = File::create(Path::new(config_path)).ok().unwrap();

    if let Some(pos_v) = pos {
        confug_file.write(b"[Item_Pos]\r\n");
        pos_v.iter().enumerate().for_each(|it| {
            confug_file.write(format!("{}={},{}\r\n", it.0, it.1.x as i32, it.1.y as i32).as_bytes());
        });
    }

    confug_file.write(b"[Item_Name]\r\n");
    fs.iter().for_each(|it|{
        confug_file.write(it.to_str().unwrap().as_bytes());
        confug_file.write(b"\r\n");
    });

    confug_file.sync_all().unwrap();
}

pub fn load_config() -> Option<(Vec<Vec2>,Vec<String>)>
{
    let mut confug_file = File::open(Path::new(config_path));

    if let Ok(ref mut f) = confug_file{
        let mut pos:Vec<Vec2> = Vec::new();
        let mut paths:Vec<String> = Vec::new();
        let mut res:String = String::new();
        pos.resize(60,Vec2::new(0f32,0f32));
        f.read_to_string(&mut res);
        let mut stage = 0;

        res.lines().for_each(|it|{
            if it != ""{
                if stage != 0 {
                    if it.starts_with("[") && it.ends_with("]") {
                        stage = 0;
                    }else {
                        match stage {
                            1 => {
                                let arr1: Vec<&str> = it.split("=").collect();
                                let index = usize::from_str(arr1[0]).unwrap();
                                let arr2: Vec<&str> = arr1[1].split(",").collect();
                                let x = i32::from_str(arr2[0]).unwrap();
                                let y = i32::from_str(arr2[1]).unwrap();
                                pos[index] = Vec2::new(x as f32, y as f32);
                            },
                            2 => {
                                paths.push(it.to_string());
                            },
                            _ => {}
                        }
                    }
                }
                if stage == 0 {
                    match it {
                        "[Item_Pos]" => stage = 1,
                        "[Item_Name]" => stage = 2,
                        _ => {}
                    }
                }
            }
        });

        Some((pos,paths))
    }else{
        None
    }
}

fn del_temp_item(paths:&Vec<String>)
{
    paths.iter().for_each(|f| {
        //println!("{}",f);
        remove_file(Path::new(f));
    });
}

fn reduction(sys_lv:&SysLv,pos:&Vec<Vec2>)
{
    for n in 0..sys_lv.size(){
        let i = n as usize;
//        let vx = pos[i].x as i32;
//        let vy = pos[i].y as i32;
//        sys_lv.set_item_pos(i,vx,vy);
        sys_lv.set_item_pos(i,pos[i].x as i32,pos[i].y as i32);
    }
}

