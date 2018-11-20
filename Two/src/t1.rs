pub extern crate libc;

use super::std::fmt::Debug;
use super::std::fmt::Formatter;
use super::std::fmt;

struct Pair<K, V> {
    key: K,
    val: V,
}

impl<K, V> Debug for Pair<K, V>
    where
        K: Debug,
        V: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "key = {:?} val = {:?}", self.key, self.val)
    }
}

impl<K, V> Pair<K, V> {
    fn doing(&self) -> Result<&K, &V>
    {
        Ok(&self.key)
    }
}

fn show<T>(t: &T)
    where
        T: Debug,
{
    println!("{:?}", t);
}

pub fn test1() {
    let pair = Pair { key: 5, val: 7 };
    let pair_ptr = Box::new(Pair {
        key: "hhjhj",
        val: "jshjdhsaj",
    });
    show(&pair_ptr);
    show(&pair);

    let a = String::from("hhh");
    let b = &a[1..];
    unsafe {
        let c = b.as_ptr() as *mut u8;
        *c = b'l';
        *(c.offset(1)) = b'k';
    }

    print!("{}", a);
}

fn test2() {
    let a = 3;
    if let 3 = a {
        println!("a = 5");
    }
}

use std::mem;

pub fn test3() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let q = Message::Quit;
    let m = Message::Move { x: 6, y: 7 };

    println!("{}", mem::size_of_val(&q));
    println!("{}", mem::size_of_val(&m));
}


pub fn test4()
{
    let a = Pair { key: 1, val: 3 };
    let b = a.doing();
    match b {
        Ok(ref x) => println!("Ok key = {}", x),
        Err(ref y) =>
            {
                println!("{}", y);

                unsafe {
                    let mut c = (*y) as *const i32;
                    let mut d = c as *mut i32;
                    *d = 78;
                }
                println!("{}", y);
            }
    }
    println!("{:?}", a);
}


pub fn test5()
{
    let a = 4;
    let b = 8;
    let _aa = 9;

    let c = || {
        a + b
    };
    println!("{}", c());
}

