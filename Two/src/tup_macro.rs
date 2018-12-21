use GenTupCat::{gen_tup_cat_item,gen_tup_sub,gen_tup_cat_tup};
use std::fmt::Debug;
use std::fmt::Formatter;

pub trait CatItem<E> {
    type Ret;
    fn cat_item(self,o:E) -> Self::Ret;
}

pub trait CatTup<E> {
    type Ret;
    fn cat(self,o:E) -> Self::Ret;
}

pub trait TupSub{
    type Ret;
    fn sub(self)-> Self::Ret;
}

pub trait TupPrint {
    fn print(&self);
}

impl<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14> TupPrint for (T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14)
where T1:Debug,
      T2:Debug,
      T3:Debug,
      T4:Debug,
      T5:Debug,
      T6:Debug,
      T7:Debug,
      T8:Debug,
      T9:Debug,
      T10:Debug,
      T11:Debug,
      T12:Debug,
      T13:Debug,
      T14:Debug
{
    fn print(&self) {
        println!("({:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?})",
        self.0,self.1,self.2,self.3,self.4,self.5,self.6,self.7,self.8,self.9,self.10,self.11,self.12,self.13);
    }
}

gen_tup_cat_item!(2);
gen_tup_cat_item!(3);
gen_tup_cat_item!(4);
gen_tup_cat_item!(5);
gen_tup_cat_item!(6);
gen_tup_cat_item!(7);
gen_tup_cat_item!(8);
gen_tup_cat_item!(9);
gen_tup_cat_item!(10);
gen_tup_cat_item!(11);
gen_tup_cat_item!(12);
gen_tup_cat_item!(13);

gen_tup_sub!(3);
gen_tup_sub!(4);
gen_tup_sub!(5);
gen_tup_sub!(6);
gen_tup_sub!(7);
gen_tup_sub!(8);
gen_tup_sub!(9);
gen_tup_sub!(10);
gen_tup_sub!(11);
gen_tup_sub!(12);
gen_tup_sub!(13);
gen_tup_sub!(14);
gen_tup_sub!(15);
gen_tup_sub!(16);
gen_tup_sub!(17);


#[macro_export]
macro_rules! tup_get {
    ($tup:ident,$n:tt) => {
        $tup.$n
    };
}

gen_tup_cat_tup!(impl 2 for     2);
gen_tup_cat_tup!(impl 3 for     2);
gen_tup_cat_tup!(impl 4 for     2);
gen_tup_cat_tup!(impl 5 for     2);
gen_tup_cat_tup!(impl 6 for     2);
gen_tup_cat_tup!(impl 7 for     2);
gen_tup_cat_tup!(impl 8 for     2);
gen_tup_cat_tup!(impl 9 for     2);
gen_tup_cat_tup!(impl 10 for    2);
gen_tup_cat_tup!(impl 11 for    2);

gen_tup_cat_tup!(impl 2 for     3);
gen_tup_cat_tup!(impl 3 for     3);
gen_tup_cat_tup!(impl 4 for     3);
gen_tup_cat_tup!(impl 5 for     3);
gen_tup_cat_tup!(impl 6 for     3);
gen_tup_cat_tup!(impl 7 for     3);
gen_tup_cat_tup!(impl 8 for     3);
gen_tup_cat_tup!(impl 9 for     3);
gen_tup_cat_tup!(impl 10 for    3);
gen_tup_cat_tup!(impl 11 for    3);

gen_tup_cat_tup!(impl 2 for     4);
gen_tup_cat_tup!(impl 3 for     4);
gen_tup_cat_tup!(impl 4 for     4);
gen_tup_cat_tup!(impl 5 for     4);
gen_tup_cat_tup!(impl 6 for     4);
gen_tup_cat_tup!(impl 7 for     4);
gen_tup_cat_tup!(impl 8 for     4);
gen_tup_cat_tup!(impl 9 for     4);
gen_tup_cat_tup!(impl 10 for    4);
gen_tup_cat_tup!(impl 11 for    4);

gen_tup_cat_tup!(impl 2 for     5);
gen_tup_cat_tup!(impl 3 for     5);
gen_tup_cat_tup!(impl 4 for     5);
gen_tup_cat_tup!(impl 5 for     5);
gen_tup_cat_tup!(impl 6 for     5);
gen_tup_cat_tup!(impl 7 for     5);
gen_tup_cat_tup!(impl 8 for     5);
gen_tup_cat_tup!(impl 9 for     5);
gen_tup_cat_tup!(impl 10 for    5);
gen_tup_cat_tup!(impl 11 for    5);

gen_tup_cat_tup!(impl 2 for     6);
gen_tup_cat_tup!(impl 3 for     6);
gen_tup_cat_tup!(impl 4 for     6);
gen_tup_cat_tup!(impl 5 for     6);
gen_tup_cat_tup!(impl 6 for     6);
gen_tup_cat_tup!(impl 7 for     6);
gen_tup_cat_tup!(impl 8 for     6);
gen_tup_cat_tup!(impl 9 for     6);
gen_tup_cat_tup!(impl 10 for    6);
gen_tup_cat_tup!(impl 11 for    6);

gen_tup_cat_tup!(impl 2 for     7);
gen_tup_cat_tup!(impl 3 for     7);
gen_tup_cat_tup!(impl 4 for     7);
gen_tup_cat_tup!(impl 5 for     7);
gen_tup_cat_tup!(impl 6 for     7);
gen_tup_cat_tup!(impl 7 for     7);
gen_tup_cat_tup!(impl 8 for     7);
gen_tup_cat_tup!(impl 9 for     7);
gen_tup_cat_tup!(impl 10 for    7);
gen_tup_cat_tup!(impl 11 for    7);

gen_tup_cat_tup!(impl 2 for     8);
gen_tup_cat_tup!(impl 3 for     8);
gen_tup_cat_tup!(impl 4 for     8);
gen_tup_cat_tup!(impl 5 for     8);
gen_tup_cat_tup!(impl 6 for     8);
gen_tup_cat_tup!(impl 7 for     8);
gen_tup_cat_tup!(impl 8 for     8);
gen_tup_cat_tup!(impl 9 for     8);
gen_tup_cat_tup!(impl 10 for    8);
gen_tup_cat_tup!(impl 11 for    8);

gen_tup_cat_tup!(impl 2 for     9);
gen_tup_cat_tup!(impl 3 for     9);
gen_tup_cat_tup!(impl 4 for     9);
gen_tup_cat_tup!(impl 5 for     9);
gen_tup_cat_tup!(impl 6 for     9);
gen_tup_cat_tup!(impl 7 for     9);
gen_tup_cat_tup!(impl 8 for     9);
gen_tup_cat_tup!(impl 9 for     9);
gen_tup_cat_tup!(impl 10 for    9);
gen_tup_cat_tup!(impl 11 for    9);

gen_tup_cat_tup!(impl 2 for     10);
gen_tup_cat_tup!(impl 3 for     10);
gen_tup_cat_tup!(impl 4 for     10);
gen_tup_cat_tup!(impl 5 for     10);
gen_tup_cat_tup!(impl 6 for     10);
gen_tup_cat_tup!(impl 7 for     10);
gen_tup_cat_tup!(impl 8 for     10);
gen_tup_cat_tup!(impl 9 for     10);
gen_tup_cat_tup!(impl 10 for    10);
gen_tup_cat_tup!(impl 11 for    10);

gen_tup_cat_tup!(impl 2 for     11);
gen_tup_cat_tup!(impl 3 for     11);
gen_tup_cat_tup!(impl 4 for     11);
gen_tup_cat_tup!(impl 5 for     11);
gen_tup_cat_tup!(impl 6 for     11);
gen_tup_cat_tup!(impl 7 for     11);
gen_tup_cat_tup!(impl 8 for     11);
gen_tup_cat_tup!(impl 9 for     11);
gen_tup_cat_tup!(impl 10 for    11);
gen_tup_cat_tup!(impl 11 for    11);
