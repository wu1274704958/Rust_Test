extern crate proc_macro;

use {
    //syn::{parse_macro_input,Token,DeriveInput,AttributeArgs},
    quote::*,
    proc_macro2,
    self::proc_macro::TokenStream
};

use std::str::FromStr;

//FS => Final separator 最后的分隔符
macro_rules! IndexSequenceNoFS {
    ($n:ident,$f:tt,$($head:tt)*) => {
        {
            let mut ts = String::new();
            for n in 0..$n {
                ts.push_str(format!("{}{}",stringify!($($head)*),n).as_str() );
                if n != $n - 1{
                    ts.push_str(format!("{}",stringify!($f)).as_str());
                }
            }
            let ts_:proc_macro2::TokenStream = ts.parse().unwrap();
            ts_
        }
    };
}

//FS => Final separator 最后的分隔符
macro_rules! IndexSequenceWithFS {
    ($n:ident,$f:tt,$($head:tt)*) => {
        {
            let mut ts = String::new();
            for n in 0..$n {
                ts.push_str(format!("{}{}{}",stringify!($($head)*),n,stringify!($f)).as_str() );
            }
            let ts_:proc_macro2::TokenStream = ts.parse().unwrap();
            ts_
        }
    };
}

const name_arr:[&'static str;5] = ["One","Two","Three","Four","Five"];


#[proc_macro]
pub fn gen_tup_trait(input: TokenStream) -> TokenStream
{
    let max_n = name_arr.len();
    let _input:String = input.to_string();
    let mut num:usize = usize::from_str(_input.as_str()).ok().unwrap();
    let ty_num = num;
    num -= 1;
    if num > max_n { panic!("num > {}",max_n); }
    let name:proc_macro2::TokenStream = format!("Cat{}",name_arr[num]).parse().unwrap();
    let ts = IndexSequenceNoFS!(ty_num,,,E);

    let func_name:proc_macro2::TokenStream = format!("cat{}",ty_num).parse().unwrap();

    let tokens = if ty_num <= 1 {
        quote! {
            pub trait #name<#ts> {
                type Ret;
                fn #func_name(self,o:#ts) ->Self::Ret;
            }
        }
    }else{
        quote! {
            pub trait #name<#ts> {
                type Ret;
                fn #func_name(self,o:(#ts)) ->Self::Ret;
            }
        }
    };
    //println!("{}",tokens.to_string());
    tokens.into()
}

#[proc_macro]
pub fn gen_tup_cat(input: TokenStream) -> TokenStream
{
    let max_n = name_arr.len();
    let _input:String = input.to_string();
    let strs:Vec<&str> = _input.split("cat").collect();
    let left = strs[0].trim();
    let right = strs[1].trim();

    let for_ty_num:u32 = u32::from_str(left).ok().unwrap();
    let cat_ty_num:u32 = u32::from_str(right).ok().unwrap();

    let n_index = (cat_ty_num - 1) as usize;
    if n_index > max_n { panic!("num > {}",max_n); }
    let name:proc_macro2::TokenStream = format!("Cat{}",name_arr[n_index]).parse().unwrap();

    let func_name:proc_macro2::TokenStream = format!("cat{}",cat_ty_num).parse().unwrap();

    let fts_ = IndexSequenceNoFS!(for_ty_num,,,T);

    let fvs_ = IndexSequenceWithFS!(for_ty_num,,,self.);

    let cts_ = IndexSequenceNoFS!(cat_ty_num,,,E);

    let cvs_ = IndexSequenceNoFS!(cat_ty_num,,,o.);

    let tokens = if cat_ty_num <= 1 {
        quote! {
            impl <#fts_,#cts_>#name<#cts_> for (#fts_) {
                type Ret = (#fts_,#cts_);

                fn #func_name(self, o: #cts_) -> Self::Ret {
                    (#fvs_ o)
                }
            }
        }
    } else{
        quote! {
            impl <#fts_,#cts_>#name<#cts_> for (#fts_) {
                type Ret = (#fts_,#cts_);

                fn #func_name(self, o: (#cts_)) -> Self::Ret {
                    (#fvs_ #cvs_)
                }
            }
        }
    };
    //println!("{:?}",tokens.to_string());
    tokens.into()
}

#[proc_macro]
pub fn gen_tup_cat_item(input: TokenStream) -> TokenStream{
    let tup_len_str:String = input.to_string();
    let tup_len:u32 = u32::from_str(tup_len_str.trim()).ok().unwrap();
    if tup_len < 2 { panic!("tuple len must > 1"); }

    let ts = IndexSequenceNoFS!(tup_len,,,T);

    let vs = IndexSequenceWithFS!(tup_len,,,self.);

    let tokens = quote!{
        impl<#ts,E1> CatItem<E1> for (#ts)
        {
            type Ret = (#ts,E1);
            #[inline]
            fn cat_item(self,o: E1) -> Self::Ret{
                (#vs o)
            }
        }
    };

    tokens.into()
}

#[proc_macro]
pub fn gen_tup_sub(input: TokenStream) -> TokenStream{
    let tup_len_str:String = input.to_string();
    let tup_len:u32 = u32::from_str(tup_len_str.trim()).ok().unwrap();
    if tup_len < 3 { panic!("tuple len must > 2"); }
    let len_sub_1 = tup_len - 1;

    let ts = IndexSequenceNoFS!(tup_len,,,T);

    let ts_sub_1 = IndexSequenceNoFS!(len_sub_1,,,T);

    let vs = IndexSequenceNoFS!(len_sub_1,,,self.);

    let tokens = quote!{
        impl<#ts> TupSub for (#ts){
            type Ret = (#ts_sub_1);

            fn sub(self) -> Self::Ret {
                (#vs)
            }
        }
    };

    tokens.into()
}

//impl<T1,T2,E1,E2> CatTup<(E1,E2)> for (T1,T2)
//{
//    type Ret = (T1,T2,E1,E2);
//    #[inline]
//    fn cat(self, o: (E1,E2)) -> Self::Ret {
//        (self.0,self.1,o.0,o.1)
//    }
//}
//
//impl<T1,T2,E1,E2,E3> CatTup<(E1,E2,E3)> for (T1,T2)
//{
//    type Ret = (T1,T2,E1,E2,E3);
//    #[inline]
//    fn cat(self, o: (E1,E2,E3)) -> Self::Ret {
//        (self.0,self.1,o.0,o.1,o.2)
//    }
//}

// impl 2 for 2
#[proc_macro]
pub fn gen_tup_cat_tup(input: TokenStream) -> TokenStream {
    let in_tokens:String = input.to_string();
    let temps:Vec<&str> = in_tokens.split("impl").collect();
    let ss:Vec<&str> = temps[1].split("for").collect();

    let impl_t_n = u32::from_str(ss[0].trim()).unwrap();
    let for_t_n = u32::from_str(ss[1].trim()).unwrap();

    if impl_t_n < 2 { panic!("impl type num must > 1"); }
    if for_t_n < 2 { panic!("tuple len must > 1"); }

    let impl_ts = IndexSequenceNoFS!(impl_t_n,,,E);
    let impl_vs = IndexSequenceWithFS!(impl_t_n,,,o.);

    let for_ts = IndexSequenceNoFS!(for_t_n,,,T);
    let for_vs = IndexSequenceWithFS!(for_t_n,,,self.);

    let tokens = quote!{
        impl<#for_ts,#impl_ts> CatTup<(#impl_ts)> for (#for_ts)
        {
            type Ret = (#for_ts,#impl_ts);
            #[inline]
            fn cat(self, o: (#impl_ts)) -> Self::Ret {
                (#for_vs #impl_vs)
            }
        }
    };
    //println!("{}",tokens.to_string());
    tokens.into()
}

#[proc_macro]
pub fn gen_tup_print(input: TokenStream) -> TokenStream {
    let tup_len_str:String = input.to_string();
    let tup_len:u32 = u32::from_str(tup_len_str.trim()).ok().unwrap();
    if tup_len < 2 { panic!("tuple len must > 1"); }

    let ts = IndexSequenceNoFS!(tup_len,,,T);

    let vs = IndexSequenceNoFS!(tup_len,,,self.);

    let mut fmtstr = String::new();
    fmtstr.push('(');
    for n in 0..tup_len{
        fmtstr.push_str("{:?}");
        if n < tup_len - 1 { fmtstr.push(','); }
    }
    fmtstr.push(')');

    let mut wherestr = String::new();

    for n in 0..tup_len{
        wherestr.push_str(format!("T{}:Debug",n).as_str());
        if n < tup_len - 1 { wherestr.push(','); }
    }

    let where_ :proc_macro2::TokenStream = wherestr.parse().unwrap();

    let tokens = quote!{
        impl<#ts> TupPrint for (#ts)
        where #where_
        {
            fn print(&self) {
                println!(#fmtstr,#vs);
            }
        }
    };

    //println!("{}",tokens.to_string());

    tokens.into()

}