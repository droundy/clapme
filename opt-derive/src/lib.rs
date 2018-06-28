// Copyright 2018 David Roundy <roundyd@physics.oregonstate.edu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate is custom derive for ClapMe. It should not be used
//! directly.

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

use proc_macro2::{Span, TokenStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::*;

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
            for f in fields.named.clone() {
                println!("f is {:?}", f.ident.as_ref().unwrap());
            }
            let f: Vec<_> = fields.named.clone().into_iter().collect();
            let idents = f.iter().map(|x| x.ident.clone().unwrap().to_string());
            let types = f.iter().map(|x| x.ty.clone());
            let f2: Vec<_> = fields.named.clone().into_iter().collect();
            let idents2 = f2.iter().map(|x| x.ident.clone().unwrap());
            let types2 = f2.iter().map(|x| x.ty.clone());
            let idents3 = fields.named.clone().into_iter()
                .map(|x| x.ident.clone().unwrap().to_string());
            quote!{
                fn augment_clap<'a, 'b>(mut info: clapme::ArgInfo<'a>,
                                        app: clapme::clap::App<'a,'b>)
                                        -> clapme::clap::App<'a,'b> {
                    info.multiple = false;
                    #( info.name = #idents; let app = #types::augment_clap(info, app); )*
                    app
                }
                fn from_clap<'a,'b>(_name: &str, app: &clapme::clap::ArgMatches) -> Option<Self> {
                    Some( #name {
                        #( #idents2: #types2::from_clap(#idents3, app)?,  )*
                    })
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
    println!("myimpl is {}", myimpl);
    tokens2.into()
}
