#![feature(nll)]
mod t1;

mod t2 {
    use std::mem;

    fn test()
    {
        let mut k = "你好";

        let mut kk = &k[0..3];
        println!("size = {}", kk.len());

        let kkk = '你';

        println!("kkk  size = {}", mem::size_of_val(&kkk));

        unsafe {
            let mut p = mem::transmute::<&char, *const u8>(&kkk);// as *const u8;

            for i in 0..4 {
                print!("{} ", *(p.offset(i as isize)));
            }
        }
        println!("{}", kkk);
        unsafe {
            let mut p = kk.as_ptr() as *const u8;

            for i in 0..kk.len() {
                print!("{} ", *(p.offset(i as isize)));
            }
        }
        println!("{}", kk);
    }
}

mod t3 {
    #[derive(Debug)]
    struct Digit(i32);

    pub fn test()
    {
        let mut v = vec![1, 2, 3];
        let mut v: Vec<Digit> = v.into_iter().map(|it: i32| { Digit(it) }).collect();

        v.iter_mut().for_each(|it: &mut Digit| {
            let &mut Digit(ref mut i) = it;
            *i += 1;
        });
        println!("{:?}", v);
    }
}

mod t4 {
    use std::mem;
    use std::cell::RefCell;

    pub fn test()
    {
        let mut a: Option<i32> = Some(8);


        /*while let Some(ref mut n) = a {
            if *n <= 0 {
                a = None;
            } else {
                *n -= 1;
            }
            println!("{:?}", a);
        }*/
    }
}

mod t5 {
    use std::mem;

    trait Test {
        fn tefun(&self) {
            println!("hello test");
        }
    }

    struct Stru {
        a: i32,
        b: i64,
        c: i64,
    }

    impl Test for Stru {
        fn tefun(&self) {
            println!("hello test Stru");
        }
    }

    fn func<T: ? Sized + Test>(t: &T)
    {
        t.tefun();
    }

    pub fn test()
    {
        let a = Box::new(Stru { a: 1, b: 2, c: 6 });
        let b = unsafe{a as Box<Test>};
        func(b.as_ref());
    }
}

mod t6 {
    pub fn test()
    {
        let mut a = 90;
        let b = unsafe{&*(&a as *const i32)};
        a = 8;
        println!("{}", b);
    }
}



mod t7{
    use std::fmt::Display;
    use std::fmt::Formatter;
    use std::fmt::Error;
    use t1::libc::system;
    use t1::libc::getpid;
    use std::clone::Clone;
    use std::marker::Copy;
    use std::mem;
    use std::cell::*;

    struct T7{
        data : i32
    }

    impl T7{
        fn new(d:i32) -> T7
        {
            println!("new {}",d);
            T7{data:d}
        }
    }
    //impl Copy for T7{}   Copy not allowed on types with destructors
    impl Drop for T7{
        fn drop(&mut self) {
            println!("drop {}",self.data);
        }
    }

    impl Clone for T7{
        fn clone(&self)->T7{
            T7{data:self.data}
        }
    }

    impl Display for T7{
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            writeln!(f,"data {}",self.data);Ok(())
        }
    }
    pub fn test()
    {
        let mut a = T7::new(4);
        let mut b = T7::new(9);
        a = b.clone();
        println!("{}",a);
        println!("{}",b);

    
        println!("{}", mem::size_of::<RefCell<i32>>());

        unsafe { println!("{}",getpid());}
        unsafe { system("pause".as_ptr() as *const i8 );}

    }
}

mod t8{

    use std::clone::Clone;
    use std::marker::Copy;
    use std::cmp::PartialEq;
    use std::fmt::Display;

    unsafe fn swap<T>(t1:*mut T,t2:*mut T)
    where T:Clone
    {
        let temp = (*t1).clone();
        *t1 = (*t2).clone();
        *t2 = temp;
    }

    fn adjustHeap<T>(now_i : usize,len:usize,arr:&mut [T])
    where T: Sized + Clone + PartialOrd
    {
        let mut now_i = now_i;
        let temp = arr[now_i].clone();
        let mut k : usize;
        k = now_i * 2 + 1;
        loop{
            if !(k < len) {break;}
            if k + 1< len && arr[k] < arr[k+1]{
                k = k+1;
            }
            if arr[k]>temp{
                arr[now_i]=arr[k].clone();
                now_i = k;
            }else { break; }
            k = k * 2 + 1;
        }
        arr[now_i] = temp;
    }

    fn heapSort<T>(arr:&mut [T])
        where T: Sized + Clone + PartialOrd + Display
    {
        let mut i = arr.len() / 2 - 1;
        loop{
            //printHeap(&arr);
            adjustHeap(i,arr.len(),arr);
            if i == 0{break;}
            i -= 1;
        }
        let mut j = arr.len() - 1;
        loop{
           // printHeap(&arr);
            unsafe {swap(&mut arr[0] ,&mut arr[j])};
            //printHeap(&arr);
            adjustHeap(0,j,arr);
            if j == 0{break;}
            j -= 1;
        }
    }

    fn printHeap<T>(arr:&[T])
    where T:Display
    {
        let mut j:i32 = 1;
        let mut ls = 0;
        let mut lines:i32 = 0;
        while ls <= arr.len() as i32{
            ls = ls + j ;
            lines = lines + 1;
            j = j * 2;
        }
        let mut spaces = ((ls - 1 )/ 2) as i32;
        let mut mid_spaces = 0;
        let mut ns = 1;
        let mut now = 0usize;
        for m in 0..lines{
            for n in 0..spaces{
                print!("\t");
            }
            for n in 0..ns{
                if now >= arr.len(){
                    break;
                }
                print!("{:^5}",arr[now]);

                for s in 0..mid_spaces{
                    print!("\t");
                }
                now = now + 1;
            }
            println!();
            ns = ns * 2;
            mid_spaces = spaces;
            spaces = (spaces - 1) / 2;
        }
    }
    pub fn test()
    {
        let mut a = [5,2,9,4,3,1,8,6];
        heapSort(&mut a);
        printHeap(&a);
       // println!("{:?}",a);
    }
}

fn main() {

    t8::test();
}
