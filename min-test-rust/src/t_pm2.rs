
extern crate pm2;

use pm2::def_const;

def_const!{a1 => 2}
def_const!{a2 => 2000i64}
def_const!{a3 => 2000isize}
def_const!{a4 => 2289312083018302}
def_const!{a5 => 2289312083018302298391389122}
def_const!{a6 => 1.26f32}
def_const!{a7 => 1.909090f64}
def_const!{a8 => 1.90}
def_const!{a9 => 2131287391289389865556.1415926765342908192f64}

fn main()
{
    dbg!(a1);
    dbg!(a4);
    dbg!(a5);
    dbg!(a6);
    dbg!(a7);
    dbg!(a8);
    dbg!(a9);
}