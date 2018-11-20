
use std::ptr::NonNull;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

extern crate test1;
use test1::New;

fn test1(){
    let mut a = 90;
    let p = NonNull::new(&mut a as *mut i32);
    match p {
        Some(ptr) => {
            unsafe{ *(ptr.as_ptr()) = 9; }
        },
        None => {
            println!("this ptr is null");
        }
    }

    println!("Hello, world! {} ",a);

    a = 99;
    println!("Hello, world! {} ",a);
}

fn test2(){
    let mut hasher = DefaultHasher::new();

    hasher.write_u32(1989);
    hasher.write_u8(11);
    hasher.write_u8(9);
    hasher.write(b"Huh?");

    println!("Hash is  {:x}   ", hasher.finish());
}

#[derive(New)]
struct Yy{
    a:i32,
    b:i32,
    c:u32
}

macro_rules! erg {
    ($($args:tt)*) => {
        ergodic!($($args)*);
    }
}

macro_rules! ergodic {
    (fn $f:ident) =>{
        println!("fn {}",$f);
    };

    (impl $f:ident) =>{
        println!("impl {}",$f);
    };

    (impl $f:ident $($k:tt)*) =>{
        println!("impl {}",$f);
        ergodic!($($k)*);
    };

    ($k:ident) => {
        {println!("{}",$k);}
    };
    (# $k:ident) => {
        {println!("# {}",$k);}
    };

    ($f:ident $($k:tt)*) => {
        println!("{}",$f);
        ergodic!($($k)*);
    };
    (# $f:ident $($k:tt)*) =>{
        println!("# {}",$f);
        ergodic!($($k)*);
    };

    (#(#$f:ident)*) =>{
        $f.iter().enumerate().for_each(|it|{
            println!("{} {}",it.0,it.1);
        });
    };
    (#(#$f:ident),*) =>{
        $f.iter().enumerate().for_each(|it|{
            print!("{} {},",it.0,it.1);
        });
    };
}

macro_rules! gibberish {
    (4 fn ['spang "whammo"] @_@) => { println!("@_@"); };
}

macro_rules! gibberish {
    (impl $t:tt ) => { println!("impl"); };
}

macro_rules! pki {
    ($ss:ident $f:tt) => {
        $ss.insert_str(0,stringify!($f));
    };

    ($ss:ident $f:tt $($t:tt)*) => {
        pki!($ss $($t)*);
        $ss.insert_str(0,format!("{} ",stringify!($f) ).as_str() );
    };
}

macro_rules! my_pki {

    ($($t:tt)*) => {
        {
            let mut ss = String::new();
            pki!(ss $($t)*);
            ss
        }
    };
}

fn main() {
    //mm!(9+8);
    let s = Yy{a:7,b:9,c:900};
    let v = vec![1,2,3];
    let a = 1;
    let b = 2;
    let c = 3;

    erg!(a impl b #c #b fn a);
    erg!(#(#v)*);
    erg!(#(#v),*);

    println!();
    gibberish!(impl k);

    let ooo = my_pki!{
        impl Hello for kkk{
            fn hello()
            {
               println!("hello");
            }
        }
    };

    println!("ooo = {}",ooo);

    s.new();
}
