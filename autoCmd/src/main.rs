use std::time::{SystemTime, Duration};
use crate::AutoExcOp::{PreExc, Sleep};
use std::convert::TryFrom;
use std::error::Error;
use chrono::DateTime;
use chrono::Local;

fn main() {
    let args:Vec<String> = std::env::args().collect();
    if args.len() < 2{
        println!("Bad args");return;
    }
    let local: DateTime<Local> = Local::now();
    dbg!(local.format("%Y-%m-%d %H:%M:%S").to_string());
    let t  = DateTime::parse_from_str("2021-08-07_21:27:00_+08:00", "%Y-%m-%d_%H:%M:%S_%z");
    let nt:SystemTime = t.unwrap().into();
    if let Ok(dur ) = nt.duration_since(local.into())
    {
        dbg!(dur);
    }
    dbg!(t);
}


pub enum AutoExcOp{
    Sleep(Duration),
    PreExc(u8),
    None,
}

fn auto(t:SystemTime,f:Box<dyn Fn()>,n:i32)
{
    loop{
        let now = SystemTime::now();
        let dur = if let Ok(d) = t.duration_since(now)
        {
            let s = d.as_secs();
            if s > 60 {
                AutoExcOp::Sleep(Duration::new(59,0))
            }else if s > 1 {
                AutoExcOp::Sleep(Duration::from_millis(1))
            }else{
                let ms = d.as_millis();
                if ms > 1 {
                    AutoExcOp::Sleep(Duration::from_micros(990))
                }else{
                    let micros = d.as_micros();
                    if micros > 2 {
                        AutoExcOp::Sleep(Duration::from_micros(1))
                    }else if micros > 1{
                        AutoExcOp::PreExc(2)
                    }else {
                        AutoExcOp::None
                    }
                }
            }
        }else{
            AutoExcOp::None
        };
        match dur {
            AutoExcOp::Sleep(d) => { Sleep(d); }
            PreExc(i) => {
                for _ in 0..i{
                    f();
                }
            }
            AutoExcOp::None => {
                for _ in 0..n{
                    f();
                }
                return;
            }
        }

    }
}
