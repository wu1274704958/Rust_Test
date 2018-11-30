#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_mut)]
#![allow(dead_code)]
use std::ptr::NonNull;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::any::{ Any,TypeId};

extern crate test1;
use test1::New;
use test1::route;
use test1::def_const;

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
        $f.iter().for_each(|it|{
            println!("{}",it);
        });
    };
    (#(#$f:ident),*) =>{
        $f.iter().for_each(|it|{
            print!("{},",it);
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
    ($ss:ident { $($f:tt)* } $($t:tt)* ) =>{
        $ss.push('{');
        {
            let inner = my_pki!($($f)*);
            $ss.insert_str($ss.len(),inner.as_str());
        }
        $ss.push('}');
        pki!($ss $($t)*);
    };

    ($ss:ident ( $($f:tt)* ) $($t:tt)* ) =>{
        $ss.push('(');
        {
            let inner = my_pki!($($f)*);
            $ss.insert_str($ss.len(),inner.as_str());
        }
        $ss.push(')');
        pki!($ss $($t)*);
    };

    ($ss:ident # $f:ident $($t:tt)*) => {
        if $crate::is_string(&$f)
        {
            $ss.push_str(format!("{} ",$f ).as_str());
        }else{
            $ss.push_str(format!("\"{}\" ",$f ).as_str());
        }
        pki!($ss $($t)*);
    };

    ($ss:ident $f:tt) => {
        $ss.insert_str($ss.len(),stringify!($f));
    };

    ($ss:ident) => {};

    ($ss:ident #(#$v:tt)* ) => {
        $v.iter().for_each(|it|{
           if $crate::is_string(it){
                $ss.push_str(it.as_str());
           }else{
                $ss.push_str( format!("\"{}\"",it).as_str() );
           }
        });
    };

    ($ss:ident $f:tt $($t:tt)*) => {
        $ss.insert_str($ss.len(),format!("{} ",stringify!($f) ).as_str() );
        pki!($ss $($t)*);
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


fn is_string(t:&Any) ->bool
{
    t.is::<String>()
}

fn test3()
{
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

    let struct_name = "KKK".to_string();
    let func_name:String = String::from("hello");

    let mut v:Vec<String> = Vec::new();
    v.push(my_pki!{ println!("Hello {}",a); });
    v.push(my_pki!{ println!("Hello {}",b); });
    v.push(my_pki!{ println!("Hello {}",c); });


    let ooo = my_pki!{
        impl Hello for #struct_name{
            fn #func_name()
            {
               #(#v)*
            }
        }
    };
    println!("ooo = {}",ooo);
    s.new();
}

#[route(GET, "/",true)]
fn func2()
{

}

def_const!{ aa => 2 }
def_const!{ bb => 3 }
def_const!{ cc => 4 }
//not stable procedural macros cannot be expanded to expressions (see issue #54727)
//fn test4(){
//    let hm = hashmap!{ { "a" => 1,"b" => 2,"c" => 3 } };
//    assert_eq!(hm["c"], 3);
//    let hm = hashmap!{ "a": 1, "b": 2,};
//    assert_eq!(hm["a"], 1);
//}

fn test4(){
    func2();
    println!("aa = {} bb = {} cc = {} ",aa,bb,cc);
}

fn main() {
    test4();
}
