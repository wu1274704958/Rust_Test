


macro_rules! ck_type_parse {
    ($v:ident $ss:tt) => {
        if let Ok(iv) = $v.base10_parse::<$ss>() {
             return Some(stringify!($ss));
        }
    };
    ($v:ident $f:tt $($ss:tt)*) => {
        if let Ok(iv) = $v.base10_parse::<$f>() {
             return Some(stringify!($f));
        }
        ck_type_parse!($v $($ss)*)
    };
}

macro_rules! ck_suffix_sub {
    ($v:ident $ss:tt) => {
        if $v.suffix() == stringify!($ss) {return Some(stringify!($ss))};
    };
    ($v:ident $f:tt $($ss:tt)*) => {
        if $v.suffix() == stringify!($f) {return Some(stringify!($f))};
        ck_suffix_sub!($v $($ss)*);
    };
}

macro_rules! check_suffix {
    ($v:ident $($ss:tt)*) => {
        if $v.suffix().len() > 0{
            ck_suffix_sub!($v $($ss)*);
        }
    };
}