#![allow(unused_variables)]
#![allow(unused_imports)]
extern crate proc_macro;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use {
    syn::{parse_macro_input,Token,DeriveInput,AttributeArgs},
    quote::*,
    proc_macro2,
    self::proc_macro::TokenStream
};

#[proc_macro_derive(New)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let result = match ast.data {
        syn::Data::Struct(ref s) => {
            impl_new(&ast,&s.fields)
        },
        _ => panic!("doesn't work with unions yet"),
    };
    result.into()
}

fn impl_new(ast:&DeriveInput, field:&syn::Fields) -> proc_macro2::TokenStream
{
    let struct_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let mut es:Vec<proc_macro2::TokenStream> = Vec::new();

    if let syn::Fields::Named(ref fs) = *field{
        fs.named.iter().for_each(|it|{
            let name = if let Some(ref temp) = (*it).ident{
                temp
            }else{
                panic!("ident is None!");
            };
            let str_name = quote!{#name}.to_string();
            let st = format!("{} = {{}}",str_name);
            es.push(quote!{ println!(#st,self.#name); } );
        });
    }
    let es = quote!{  #(#es)* };
    quote!{
        impl #impl_generics #struct_name #ty_generics #where_clause {
            fn new(&self){
                #es
            }
        }
    }
}


use syn::NestedMeta::{Meta,Literal};
use syn::Meta::{Word,List,NameValue};
use syn::Lit::*;
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast:AttributeArgs = parse_macro_input!(attr as AttributeArgs);
    let last_n = ast.len() - 1;

    let mut str = String::new();
    str.push('(');
    ast.iter().enumerate().for_each(|it| {
        match *(it.1) {
            Meta(ref meta) => {
                match *meta {
                    Word(ref ident) =>{
                        str.insert_str(str.len(),ident.into_token_stream().to_string().as_str());
                    },
                    List(ref list) =>{
                        println!("List");
                    },
                    NameValue(ref name_value) =>{
                        println!("NameValue");
                    }
                };
            },
            Literal(ref lit) => {

                match *lit {
                    Str(ref s ) =>{
                        str.push('"');
                        str.push_str(s.value().as_str());
                        str.push('"');
                    },
                    _ => {
                        str.push_str("Others");
                    }
                }

            }
        }
        if it.0 < last_n{
            str.push(',');
        }
    });
    str.push(')');
    let ret = quote!{ fn func(){ println!("{}",#str); } };
    println!("{}",ret);
    ret.into()
}

