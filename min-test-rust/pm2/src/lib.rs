extern crate proc_macro;
extern crate syn;
#[macro_use]
mod macros;

use syn::{
  Lit
};
use std::any::Any;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn parser_ty(s:&str)-> Option<&'static str>
{
    let ts = syn::parse_str::<syn::Lit>(s);
    if ts.is_err()
    {
        println!("err lit str");
        return None;
    }else {
        match ts.unwrap() {
            Lit::Int(ref v) => {
                check_suffix!(v i32 i64 i128 i8 i16 isize);
                ck_type_parse!(v i32 i64 i128);
                None
            },
            Lit::Float(ref v ) => {
                check_suffix!(v f32 f64);
                ck_type_parse!(v f32 f64);
                None
            },
            _ => { None }
        }
    }
}

#[proc_macro]
pub fn def_const(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let _input:String = input.to_string();
    let mut strs:Vec<&str> = _input.split("=>").collect();

    strs = strs.iter().map(|str|->&str{
        str.trim()
    }).collect();
    let ty = parser_ty(strs[1]).unwrap();

    let tokens = format!(" const {}:{} = {};",strs[0],ty,strs[1]);
    dbg!(tokens.as_str());

    tokens.parse().unwrap()
}