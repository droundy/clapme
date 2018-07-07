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


fn one_field_name(f: syn::Fields) -> proc_macro2::TokenStream {
    match f {
        syn::Fields::Named(ref fields) => {
            let f: Vec<_> = fields.named.clone().into_iter().collect();
            let names = f.iter().map(|x| x.ident.clone().unwrap().to_string());
            let types = f.iter().map(|x| x.ty.clone());
            quote! {
                {
                    let mut flagname: Option<String> = None;
                    #(
                        let thisname = format!("{}{}", prefix, #names);
                        let reqs = <#types as ClapMe>::requires_flags(&thisname);
                        if let Some(x) = reqs.first() {
                            flagname = Some(x.clone());
                        }
                    )*
                    flagname.expect("enum must have one required field!")
                }
            }
        },
        _ => {
            panic!("ClapMe only supports named fields so far!")
        },
    }
}

fn return_with_fields(f: syn::Fields,
                      name: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    match f {
        syn::Fields::Named(ref fields) => {
            let f: Vec<_> = fields.named.clone().into_iter().collect();
            let names = f.iter().map(|x| x.ident.clone().unwrap().to_string());
            let types = f.iter().map(|x| x.ty.clone());
            let idents = f.iter().map(|x| x.ident.clone().unwrap());
            quote! {
                return Some( #name {
                    #( #idents:
                        <#types as ClapMe>::from_clap(&format!("{}{}", &prefix, #names),
                                                      matches)?,  )*
                });
            }
        },
        _ => {
            panic!("ClapMe only supports named fields so far!")
        },
    }
}

fn with_clap_fields(f: syn::Fields) -> proc_macro2::TokenStream {
    match f {
        syn::Fields::Named(ref fields) => {
            let f: Vec<_> = fields.named.clone().into_iter().collect();
            let names = f.iter().rev().map(|x| x.ident.clone().unwrap().to_string());
            let types = f.iter().rev().map(|x| x.ty.clone());
            let names1 = f.iter().rev().map(|x| x.ident.clone().unwrap().to_string());
            let types1 = f.iter().rev().map(|x| x.ty.clone());

            let docs: Vec<_> = f.iter().rev().map(|x| get_doc_comment(&x.attrs)).collect();
            quote!{
                let mut flags: Vec<String> = Vec::new();
                if !info.required {
                    // only add dependencies on flags required by this
                    // set of fields, but not absolutely required.
                    #(flags.extend(<#types1 as ClapMe>::requires_flags(&format!("{}{}", &prefix, #names1)));)*;
                }
                let mut new_req: Vec<&str> = flags.iter().map(AsRef::as_ref).collect();
                new_req.extend(info.required_flags);

                #( let argname: String = format!("{}{}", &prefix, #names);
                   let my_req: Vec<&str>
                   = new_req.iter().map(|&s| s).filter(|s| *s != argname).collect();
                   let newinfo = clapme::ArgInfo {
                       name: &argname,
                       help: #docs,
                       required_flags: &my_req,
                       required_unless_one: info.required_unless_one.clone(),
                       conflicted_flags: info.conflicted_flags.clone(),
                       ..info
                   };
                   let f = |app: clapme::clap::App| {
                       <#types as ClapMe>::with_clap(newinfo, app, f)
                   };
                )*
            }
        },
        _ => {
            panic!("ClapMe only supports named fields so far!")
        },
    }
}

/// Generates the `ClapMe` impl.
#[proc_macro_derive(ClapMe)]
pub fn clapme(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use syn::Data::*;
    let input: DeriveInput = syn::parse(input).unwrap();

    let name = &input.ident;
    let find_prefix = quote!{
        let prefix: String = match name.chars().next() {
            None | Some('_') => "".to_string(),
            _ => format!("{}-", name),
        };
    };
    let myimpl = match input.data {
        Struct(DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => {
            let f: Vec<_> = fields.named.clone().into_iter().collect();
            let types3 = f.iter().rev().map(|x| x.ty.clone());
            let names3 = f.iter().rev().map(|x| x.ident.clone().unwrap().to_string());
            let with_clap_stuff = with_clap_fields(syn::Fields::Named(fields.clone()));
            let return_struct = return_with_fields(syn::Fields::Named(fields.clone()),
                                                   quote!(#name));
            quote!{
                fn with_clap<T>(mut info: clapme::ArgInfo,
                                app: clapme::clap::App,
                                f: impl FnOnce(clapme::clap::App) -> T)
                                -> T {
                    info.multiple = false;
                    let name = info.name;
                    #find_prefix
                    #with_clap_stuff
                    f(app)
                }
                fn from_clap<'a,'b>(name: &str, matches: &clapme::clap::ArgMatches) -> Option<Self> {
                    #find_prefix
                    #return_struct
                }
                fn requires_flags(name: &str) -> Vec<String> {
                    #find_prefix
                    let mut flags: Vec<String> = Vec::new();
                    #(flags.extend(<#types3 as ClapMe>::requires_flags(&format!("{}{}", &prefix, #names3)));)*;
                    flags
                }
            }
        },
        Enum(ref e) => {
            let v: Vec<_> = e.variants.iter().collect();
            let names: Vec<_> = v.iter().map(|x| x.ident.to_string()).collect();
            println!("variant names are {:?}", names);
            let fields: Vec<_> = v.iter().map(|x| x.fields.clone()).collect();
            let with_claps: Vec<_> = v.iter().map(|v| with_clap_fields(v.fields.clone())).collect();
            println!("variant with_claps are {:?}", with_claps);
            let one_field: Vec<_> = fields.iter().map(|f| one_field_name(f.clone())).collect();
            let one_field2 = one_field.clone();
            let one_field3 = one_field.clone();
            let return_enum = v.iter().map(|v| {
                let variant_name = v.ident.clone();
                return_with_fields(v.fields.clone(), quote!(#name::#variant_name))
            });
            let s = quote! {
                fn with_clap<T>(mut info: clapme::ArgInfo,
                                app: clapme::clap::App,
                                f: impl FnOnce(clapme::clap::App) -> T)
                                -> T {
                    let name = info.name;
                    #find_prefix
                    info.multiple = false;

                    let mut conflicts: Vec<String> = Vec::new();
                    #( conflicts.push(#one_field2); )*

                    let original_conflicted = info.conflicted_flags.clone();
                    let original_required_unless = info.required_unless_one.clone();
                    let am_required = info.required || original_required_unless.len() > 0;
                    info.required = false;
                    #(
                        let myself = #one_field3;
                        info.required_unless_one = original_required_unless.clone();
                        info.conflicted_flags = original_conflicted.clone();
                        conflicts.iter().filter(|s| **s != myself).map(|s| {
                            info.conflicted_flags.push(s.clone());
                            if am_required {
                                info.required_unless_one.push(s.clone());
                            }
                        }).count();

                        #with_claps
                    )*
                    f(app)
                }
                fn from_clap<'a,'b>(name: &str, matches: &clapme::clap::ArgMatches) -> Option<Self> {
                    #find_prefix
                    #(
                        if matches.is_present(#one_field) {
                            #return_enum
                        }
                    )*
                    panic!("Some version of the enum should be present!")
                }
            };
            println!("{}", s);
            s
        },
        _ => panic!("ClapMe only supports non-tuple structs and enums"),
    };

    let tokens2: proc_macro2::TokenStream = quote!{
        impl ClapMe for #name {
            #myimpl
        }
    };
    tokens2.into()
}
