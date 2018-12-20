
#[macro_export]
macro_rules! tup_get {
    ($tup:ident,$n:tt) => {
        $tup.$n
    };
}

pub fn tup_cat<T,T2>(t:(T,T),t2:T2) -> (T,T,T2)
{
    (t.0,t.1,t2)
}
