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
    println!("{}",ty_num);

    let tokens = quote!{  const a:u32 = #ty_num; };

    tokens.into()
}
