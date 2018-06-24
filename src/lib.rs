#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenTree::Literal};
use syn::{
    parse_str,
    Data::Struct,
    DeriveInput,
    Field,
    Fields::Named,
    FnArg::Captured,
    FnArg,
    Type,
};

#[proc_macro_derive(New, attributes(Sig))]
pub fn prismatic(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let fields = struct_fields(&ast);
    let name = struct_name(&ast);
    let members = struct_members(&fields);
    let types = struct_types(&fields);
    let signature = sig_attribute(&ast);
    let params = sig_params(&signature);

    generate_code(&name, &members, &types, &signature, &params)
}

fn generate_code(name: &Ident, members: &[&Ident], types: &[&Type], signature: &[FnArg], params: &[&Ident]) -> TokenStream {
    let members2 = &(*members);
    let members3 = &(*members);
    let members4 = &(*members);

    let code = quote! {
        impl #name {
            fn new(#(#signature),*) -> Self {
                let mut init = Init::default();
                init.init(#(#params),*);

                Self { #(
                  #members: init.#members2
                      .expect(stringify!(#members3 has not been initialized))
                ),* }
            }
        }

        #[derive(Default)]
        struct Init {
            #(#members4: Option<#types>),*
        }

        #[allow(dead_code)]
        impl Init {
//            fn set_red(&mut self, red: usize) {
//                self.red = Some(red);
//            }
//
//            fn set_green(&mut self, green: usize) {
//                self.green = Some(green);
//            }
//
//            fn red(&self) -> &usize {
//                self.red.as_ref().unwrap()
//            }
        }
    };

    code.into()
}

fn struct_name(ast: &DeriveInput) -> &Ident {
    &ast.ident
}

fn struct_fields(ast: &DeriveInput) -> Vec<&Field> {
    if let Struct(data) = &ast.data {
        if let Named(fields) = &data.fields {
            return fields.named.iter().collect()
        }
    }

    Vec::new()
}

fn struct_types<'a>(fields: &'a [&Field]) -> Vec<&'a Type> {
    fields.iter().map(|f| &f.ty).collect()
}

fn struct_members<'a>(fields: &'a [&Field]) -> Vec<&'a Ident> {
    fields.iter().map(|f| f.ident.as_ref().unwrap()).collect()
}

fn sig_attribute(ast: &DeriveInput) -> Vec<FnArg> {
    let args = attribute_value(ast);
    if args.is_empty() { return Vec::new() }

    args.split(',').map(|p|
        parse_str(p).unwrap_or_else(|_| {
            panic!("failed to parse signature while deriving New")
        })
    ).collect()
}

fn attribute_value(ast: &DeriveInput) -> String {
    let attribute = &ast.attrs.first();
    let empty = proc_macro2::TokenStream::new();
    let tokens = attribute.map_or(&empty, |a| &a.tts);

    for token in tokens.clone() {
        if let Literal(literal) = token {
            let value = literal.to_string();
            return trim_quotes(&value);
        }
    }

    String::new()
}

fn trim_quotes(string: &str) -> String {
    string.trim_matches('"').to_string()
}

fn sig_params(signature: &[FnArg]) -> Vec<&Ident> {
    let options = signature.iter().map(|arg| {
        if let Captured(captured) = arg {
            if let syn::Pat::Ident(pat_ident) = &captured.pat {
                return Some(&pat_ident.ident)
            }
        }

        None
    });

    options.map(|o| o.unwrap()).collect()
}
