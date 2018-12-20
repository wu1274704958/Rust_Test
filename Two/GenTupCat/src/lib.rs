extern crate proc_macro;

use {
    syn::{parse_macro_input,Token,DeriveInput,AttributeArgs},
    quote::*,
    proc_macro2,
    self::proc_macro::TokenStream
};

use std::str::FromStr;
#[proc_macro]
pub fn gen_tup_cat(input: TokenStream) -> TokenStream
{
    let _input:String = input.to_string();
    let ty_num:u32 = u32::from_str(_input.as_str()).ok().unwrap();

    let mut ts = String::new();
    for n in 0..ty_num {
        ts.push_str(format!("T{}",n).as_str() );
        if n != ty_num - 1{
            ts.push(',');
        }
    }

    let ts_:proc_macro2::TokenStream = ts.parse().unwrap();

    let mut vs = String::new();
    for n in 0..ty_num {
        vs.push_str(format!("self.{},",n).as_str() );
    }
    let vs_:proc_macro2::TokenStream = vs.parse().unwrap();

    let tokens = quote!{
        impl <#ts_,T>CatOne<T> for (#ts_) {
            type Ret = (#ts_,T);

            fn cat(self, o: T) -> Self::Ret {
                (#vs_ o)
            }
        }
    };

    //println!("{:?}",tokens.to_string());

    tokens.into()
}
