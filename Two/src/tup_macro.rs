use GenTupCat::gen_tup_cat;

pub trait CatOne<T> {
    type Ret;
    fn cat(self,o:T) ->Self::Ret;
}

#[macro_export]
macro_rules! tup_get {
    ($tup:ident,$n:tt) => {
        $tup.$n
    };
}

gen_tup_cat!(2);
gen_tup_cat!(3);
gen_tup_cat!(4);
gen_tup_cat!(5);
gen_tup_cat!(6);

