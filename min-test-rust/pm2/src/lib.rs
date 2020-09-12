extern crate proc_macro;
extern crate syn;
extern crate encryption;
#[macro_use]
mod macros;

use encryption::Simple;

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

fn parser_ty_ex(s:&str)-> Option<(&'static str,String)>
{
    let ts = syn::parse_str::<syn::Lit>(s);
    if ts.is_err()
    {
        println!("err lit str");
        return None;
    }else {
        match ts.unwrap() {
            Lit::Str(ref v) => {
                if v.suffix().len() >= 0{
                    match v.suffix() {
                        "encode_s" => {
                            let mut s = Simple::encode(v.value().as_str());
                            s.insert(0,'"');
                            s.push('"');
                            Some(("&'static str",s))
                        },
                        "decode_s" => {
                            let mut s = Simple::decode(v.value().as_str());
                            s.insert(0,'"');
                            s.push('"');
                            Some(("&'static str",s))
                        },
                        _ => {
                            None
                        }
                    }
                }else {
                    Some(("&'static str",String::from(s)))
                }
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
    let ty = parser_ty(strs[1]);
    let mut tokens:String;
    if ty.is_some() {
        tokens = format!(" const {}:{} = {};",strs[0],ty.unwrap(),strs[1]);
    }else{
        let ty = parser_ty_ex(strs[1]);
        if ty.is_some() {
            let ty_and_v = &ty.unwrap();
            tokens = format!(" const {}:{} = {};",strs[0],ty_and_v.0,ty_and_v.1);
        }else {
            tokens = String::from("");
        }
    }
    dbg!(tokens.as_str());
    tokens.parse().unwrap()
}