use GenTupCat::{gen_tup_cat_item,gen_tup_sub};


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

gen_tup_cat_item!(2);
gen_tup_cat_item!(3);
gen_tup_cat_item!(4);

gen_tup_sub!(3);
gen_tup_sub!(4);
gen_tup_sub!(5);
gen_tup_sub!(6);
gen_tup_sub!(7);


#[macro_export]
macro_rules! tup_get {
    ($tup:ident,$n:tt) => {
        $tup.$n
    };
}


impl<T1,T2,E1,E2> CatTup<(E1,E2)> for (T1,T2)
{
    type Ret = (T1,T2,E1,E2);
    #[inline]
    fn cat(self, o: (E1,E2)) -> Self::Ret {
        (self.0,self.1,o.0,o.1)
    }
}

impl<T1,T2,E1,E2,E3> CatTup<(E1,E2,E3)> for (T1,T2)
{
    type Ret = (T1,T2,E1,E2,E3);
    #[inline]
    fn cat(self, o: (E1,E2,E3)) -> Self::Ret {
        (self.0,self.1,o.0,o.1,o.2)
    }
}


