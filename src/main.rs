#![allow(non_snake_case)]
#[allow(dead_code)]
mod t1;
#[allow(dead_code)]
mod t2 {
    use std::mem;

    fn test()
    {
        let k = "你好";

        let kk = &k[0..3];
        println!("size = {}", kk.len());

        let kkk = '你';

        println!("kkk  size = {}", mem::size_of_val(&kkk));

        unsafe {
            let p = mem::transmute::<&char, *const u8>(&kkk);// as *const u8;

            for i in 0..4 {
                print!("{} ", *(p.offset(i as isize)));
            }
        }
        println!("{}", kkk);
        unsafe {
            let p = kk.as_ptr() as *const u8;

            for i in 0..kk.len() {
                print!("{} ", *(p.offset(i as isize)));
            }
        }
        println!("{}", kk);
    }
}
#[allow(dead_code)]
mod t3 {
    #[derive(Debug)]
    struct Digit(i32);

    pub fn test()
    {
        let v = vec![1, 2, 3];
        let mut v: Vec<Digit> = v.into_iter().map(|it: i32| { Digit(it) }).collect();

        v.iter_mut().for_each(|it: &mut Digit| {
            let &mut Digit(ref mut i) = it;
            *i += 1;
        });
        println!("{:?}", v);
    }
}
#[allow(dead_code)]
mod t4 {
    use std::ptr::NonNull;
    pub fn test()
    {
        let mut a: Option<i32> = Some(8);
        let ptr = NonNull::new(&mut a as *mut Option<_>);
        while let Some(ref mut n) = a {
            if *n <= 0 {
                //a = None;
                if let Some(mut p) = ptr{
                    let mut temp_p = unsafe {p.as_mut()};
                    *temp_p = None;
                }
                break;
            } else {
                *n -= 1;
            }
        }

        println!("{:?}", a);
    }
}
#[allow(dead_code)]
mod t5 {

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
        let b = a as Box<Test>;
        func(b.as_ref());
    }
}
#[allow(unused_assignments)]
#[allow(dead_code)]
mod t6 {
    pub fn test()
    {
        let mut a = 90;
        let b = unsafe{&*(&a as *const i32)};
        a = 8;
        println!("{}", b);
    }
}

#[allow(dead_code)]
#[allow(unused_assignments)]
mod t7{
    use std::fmt::Display;
    use std::fmt::Formatter;
    use std::fmt::Error;
    use t1::libc::system;
    use t1::libc::getpid;
    use std::clone::Clone;
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
        let b = T7::new(9);
        a = b.clone();
        println!("{}",a);
        println!("{}",b);

    
        println!("{}", mem::size_of::<RefCell<i32>>());

        unsafe { println!("{}",getpid());}
        unsafe { system("pause".as_ptr() as *const i8 );}
    }
}
#[allow(dead_code)]
mod t8{

    use std::clone::Clone;
    use std::fmt::Display;

    unsafe fn swap<T>(t1:*mut T,t2:*mut T)
    where T:Clone
    {
        let temp = (*t1).clone();
        *t1 = (*t2).clone();
        *t2 = temp;
    }

    fn adjust_heap<T>(mut now_i : usize,len:usize,arr:&mut [T])
    where T: Sized + Clone + PartialOrd
    {
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

    fn heap_sort<T>(arr:&mut [T])
        where T: Sized + Clone + PartialOrd + Display
    {
        let mut i = arr.len() / 2 - 1;
        loop{
            //printHeap(&arr);
            adjust_heap(i,arr.len(),arr);
            if i == 0{break;}
            i -= 1;
        }
        let mut j = arr.len() - 1;
        loop{
           // printHeap(&arr);
            unsafe {swap(&mut arr[0] ,&mut arr[j])};
            //printHeap(&arr);
            adjust_heap(0,j,arr);
            if j == 0{break;}
            j -= 1;
        }
    }

    fn print_heap<T>(arr:&[T])
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
        for _m in 0..lines{
            for _n in 0..spaces{
                print!("     ");
            }
            for _n in 0..ns{
                if now >= arr.len(){
                    break;
                }
                print!("{:^5}",arr[now]);

                for _s in 0..mid_spaces{
                    print!("     ");
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
        print_heap(&a);
        heap_sort(&mut a);
        print_heap(&a);
        //println!("{:?}",a);
    }
}
#[allow(dead_code)]
mod t9{
    use std::any::Any;
    use std::mem::transmute;

    fn kkk(t:&Any)
    {
        println!("{}",t.is::<i32>());
    }

    struct PP{
        p:i32,
        s:&'static str
    }

    trait A{
        fn a(&self);
    }

    impl A for PP{
        fn a(&self){
            println!("{}",self.s);
        }
    }

    impl PP{
        pub fn pp(&mut self,n_s:&'static str){
            self.s = n_s;
            println!(" p = {}",self.p);
        }
    }

    pub fn test()
    {
        let a : Box<_> = Box::new(PP{p:7,s:"sssss"});
        (*a).a();

        let b = unsafe{ transmute::<&Box<_>,*const Box<PP>>(&a) as *mut Box<PP> };
        unsafe { (**b).pp("aaaaa");}

        (*a).a();

        /*let mut a = String::from("ssss");
        let b = 2;
        kkk(&b);
        println!("{}",a);*/

        let a_any : &Any = &a;
        println!("{}",a_any.is::<Box<PP>>());

    }
}

#[allow(dead_code)]
mod t10{
    use std::collections::hash_map::HashMap;
    fn func(arr:&Vec<i32>,target:i32)->(usize,usize)
    {
        let mut j:usize = 0;
        let mut res:(usize,usize) = (0,0);
        'wai: loop{
            if j >= arr.len() - 1{ break;}
            let mut i:usize = j + 1;
            'nei: loop{
                if i >= arr.len(){ break;}
                if arr[i] + arr[j] == target{
                    res.0 = i;
                    res.1 = j;
                    break 'wai;
                }
                i += 1;
            }
            j += 1;
        }
        return res;
    }
    fn func2(arr:&Vec<i32>,target:i32)->(usize,usize)
    {
        let mut j:usize = 0;
        let mut res:(usize,usize) = (0,0);
        let mut map:HashMap<i32,usize> = HashMap::new();
        loop{
            if !(j < arr.len()){ break;}
            let other = target - arr[j];
            if map.contains_key(&other) {
                if let Some(v) = map.get(&other) {
                    res.0 = v.clone();
                    res.1 = j;
                }
                break;
            }
            map.insert(arr[j],j);
            j += 1;
        }
        return res;
    }
    fn run<T>(f:T) ->()
    where T: Fn(&Vec<i32>,i32)->(usize,usize)
    {
        let target = 9;
        let arr = vec![2,7,11,15];
        let res = f(&arr,target);
        assert_eq!(arr[res.0] + arr[res.1] , target,"断言失败！{}",target);

        let target = 22;
        let res = f(&arr,target);
        assert_eq!(arr[res.0] + arr[res.1] , target,"断言失败！{}",target);
    }
    pub fn test()
    {
        run(func2);
    }
}
#[allow(dead_code)]
mod t11{
    use std::cmp::PartialEq;
    use std::fmt::Debug;
    use std::fmt::Formatter;
    use std::fmt::Error;
    struct ListNode {
        pub val: u32,
        pub next: Option<Box<ListNode>>
    }
    impl ListNode{
        pub fn new(val:u32,next:Option<Box<ListNode>>) ->ListNode{
            ListNode{val,next}
        }
        pub fn as_ptr(&self)-> *mut ListNode{
            self as *const ListNode as *mut ListNode
        }
    }

    impl PartialEq for ListNode{
        fn eq(&self, other: &ListNode) -> bool {
            let mut p1 = self as *const ListNode;
            let mut p2 = other as *const ListNode;
            let mut res = true;
            unsafe {
                loop {
                    if p1.is_null() && p2.is_null(){ break;}
                    if p1.is_null() || p2.is_null(){ res = false;break;}
                    if (*p1).val != (*p2).val{
                        res = false;
                    }
                    p1 = if let Some(ref ptr) = (*p1).next{
                        ptr.as_ptr()
                    }else{
                        0 as *const ListNode
                    };
                    p2 = if let Some(ref ptr) = (*p2).next{
                        ptr.as_ptr()
                    }else{
                        0 as *const ListNode
                    };
                }
            }
            res
        }
    }
    impl Debug for ListNode{
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            let mut ptr:*const ListNode = self as *const ListNode;
            write!(f,"{}",self.val);
            unsafe {
                while let Some(ref p) = (*ptr).next {
                    let temp =  p.as_ref() ;
                    write!(f,",{}",temp.val);
                    ptr = temp as *const ListNode;
                }
            }
            Ok(())
        }
    }

    fn func(n1:&ListNode,n2:&ListNode)-> ListNode
    {
        let mut res:ListNode = ListNode::new(0,None);
        let mut ptr_res = &mut res as *mut ListNode;
        let mut p1 = n1 as *const ListNode;
        let mut p2 = n2 as *const ListNode;
        let mut jw = false;
        unsafe {
            loop {
                let a = if p1.is_null() { 0 } else { (*p1).val };
                let b = if p2.is_null() { 0 } else { (*p2).val };
                let mut c = if jw {
                    jw = false;
                    a + b + 1
                } else { a + b };
                if c > 9 {
                    jw = true;
                    c -= 10;
                }
                (*ptr_res).val = c;
                if !p1.is_null() {
                    p1 = if let Some(ref next) = (*p1).next {
                        next.as_ref() as *const ListNode
                    } else {
                        0 as *const ListNode
                    };
                }
                if !p2.is_null() {
                    p2 = if let Some(ref next) = (*p2).next {
                        next.as_ref() as *const ListNode
                    } else {
                        0 as *const ListNode
                    };
                }
                if !p1.is_null() || !p2.is_null() {
                    let temp = Box::new(ListNode::new(0, None));
                    let temp_ptr = temp.as_ptr();
                    (*ptr_res).next = Some(temp);
                    ptr_res = temp_ptr;
                } else {
                    break;
                }
            }
            if jw{
                (*ptr_res).next = Some(Box::new(ListNode::new(1, None)));
            }
        }
        res
    }

    pub fn test()
    {
        let n1 = ListNode::new(2,
                                   Some(Box::new(ListNode::new(4,
                                   Some(Box::new(ListNode::new(3,
                                   Some(Box::new(ListNode::new(2,None
                                   ))))))))));

        let n2 = ListNode::new(5,
                                   Some(Box::new(ListNode::new(6,
                                   Some(Box::new(ListNode::new(4,None
                                   )))))));

        let n3 = ListNode::new(7,
                                   Some(Box::new(ListNode::new(0,
                                   Some(Box::new(ListNode::new(8,
                                   Some(Box::new(ListNode::new(2,None
                                   ))))))))));
        println!("{:?}",n1);
        println!("{:?}",n2);
        let res= func(&n1,&n2);
        println!("{:?}",res);
        assert_eq!(res,n3,"断言失败！");
    }
}



fn main() {
    t11::test();
}
