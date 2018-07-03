// Copyright 2018 David Roundy <roundyd@physics.oregonstate.edu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate is custom derive for ClapMe. It should not be used
//! directly.

#![recursion_limit="256"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

use syn::*;

fn get_doc_comment(attrs: &[syn::Attribute]) -> String {
    let mut doc_comments: Vec<_> = attrs
        .iter()
        .filter_map(|attr| {
            let path = &attr.path;
            if quote!(#path).to_string() == "doc" {
                attr.interpret_meta()
            } else {
                None
            }
        })
        .filter_map(|attr| {
            use Lit::*;
            use Meta::*;
            if let NameValue(MetaNameValue {ident, lit: Str(s), ..}) = attr {
                if ident != "doc" {
                    return None;
                }
                let value = s.value();
                let text = value
                    .trim_left_matches("//!")
                    .trim_left_matches("///")
                    .trim_left_matches("/*!")
                    .trim_left_matches("/**")
                    .trim_right_matches("*/")
                    .trim();
                if text.is_empty() {
                    Some("\n\n".to_string())
                } else {
                    Some(text.to_string())
                }
            } else {
                None
            }
        })
        .collect();
    if doc_comments.len() > 0 {
        doc_comments.pop().unwrap_or("".to_string())
    } else {
        "".to_string()
    }
}


/// Generates the `ClapMe` impl.
#[proc_macro_derive(ClapMe)]
pub fn clapme(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::Data::*;
    let input: DeriveInput = syn::parse(input).unwrap();

    let name = &input.ident;
    let myimpl = match input.data {
        Struct(DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => {
            let f: Vec<_> = fields.named.clone().into_iter().collect();
            let names = f.iter().rev().map(|x| x.ident.clone().unwrap().to_string());
            let types = f.iter().rev().map(|x| x.ty.clone());
            let idents = f.iter().rev().map(|x| x.ident.clone().unwrap());
            let types2 = f.iter().rev().map(|x| x.ty.clone());
            let types3 = f.iter().rev().map(|x| x.ty.clone());
            let names2 = f.iter().rev().map(|x| x.ident.clone().unwrap().to_string());
            let names3 = f.iter().rev().map(|x| x.ident.clone().unwrap().to_string());
            let docs: Vec<_> = f.iter().rev().map(|x| get_doc_comment(&x.attrs)).collect();
            quote!{
                fn with_clap<T: 'static>(mut info: clapme::ArgInfo,
                                         app: clapme::clap::App,
                                         f: impl FnOnce(clapme::clap::App) -> T)
                                         -> T {
                    info.multiple = false;
                    let prefix: String = match info.name.chars().next() {
                        None | Some('_') => "".to_string(),
                        _ => { let mut x = info.name.to_string(); x.push('-'); x },
                    };
                    let new_req: Vec<String> = Self::requires_flags(info.name);
                    let mut new_req: Vec<&str> = new_req.iter().map(AsRef::as_ref).collect();
                    if info.required {
                        // no use adding dependencies on required flags
                        new_req = Vec::new();
                    }
                    new_req.extend(info.required_flags);
                    #( let argname: String = format!("{}{}", &prefix, #names);
                       let my_req: Vec<&str>
                           = new_req.iter().map(|&s| s).filter(|s| *s != argname).collect();
                       let newinfo = clapme::ArgInfo {
                           name: &argname,
                           help: #docs,
                           required_flags: &my_req,
                           ..info
                       };
                       let f = |app: clapme::clap::App| {
                           <#types>::with_clap(newinfo, app, f)
                       };
                    )*
                    f(app)
                }
                fn from_clap<'a,'b>(name: &str, app: &clapme::clap::ArgMatches) -> Option<Self> {
                    let prefix: String = match name.chars().next() {
                        None | Some('_') => "".to_string(),
                        _ => format!("{}-", name),
                    };
                    Some( #name {
                        #( #idents: <#types2>::from_clap(&format!("{}{}", &prefix, #names2),
                                                       app)?,  )*
                    })
                }
                fn requires_flags(name: &str) -> Vec<String> {
                    let prefix: String = match name.chars().next() {
                        None | Some('_') => "".to_string(),
                        _ => format!("{}-", name),
                    };
                    let mut flags: Vec<String> = Vec::new();
                    #(flags.extend(<#types3>::requires_flags(&format!("{}{}", &prefix, #names3)));)*;
                    flags
                }
            }
        },
        Enum(ref _e) => panic!("FIXME need to handle enum in clapme"),
        _ => panic!("clapme only supports non-tuple structs and enums"),
    };

    let tokens2: proc_macro2::TokenStream = quote!{
        impl ClapMe for #name {
            #myimpl
        }
    };
    tokens2.into()
}

