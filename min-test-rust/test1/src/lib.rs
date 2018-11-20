extern crate proc_macro;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use {
    syn::{parse_macro_input,Token,DeriveInput},
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