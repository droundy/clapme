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
            let names = f.iter().map(|x| snake_case_to_kebab(&x.ident.clone().unwrap().to_string()));
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
        syn::Fields::Unit => {
            quote!{
                name.to_string()
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
            let names = f.iter().map(|x| snake_case_to_kebab(&x.ident.clone().unwrap().to_string()));
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
        syn::Fields::Unit => {
            quote!( return Some( #name ); )
        },
        _ => {
            panic!("ClapMe only supports named fields so far!")
        },
    }
}

fn with_clap_fields(f: syn::Fields, mdoc: Option<String>) -> proc_macro2::TokenStream {
    match f {
        syn::Fields::Named(ref fields) => {
            let f: Vec<_> = fields.named.clone().into_iter().collect();
            let names = f.iter().rev().map(|x| snake_case_to_kebab(&x.ident.clone().unwrap().to_string()));
            let types = f.iter().rev().map(|x| x.ty.clone());
            let names1 = names.clone();
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
        syn::Fields::Unit => {
            let doc = mdoc.unwrap();
            quote!{
                let newinfo = info.clone();
                let f = |app: clapme::clap::App| {
                    let conflicts: Vec<_> = newinfo.conflicted_flags.iter().map(AsRef::as_ref).collect();
                    let ruo: Vec<_> = newinfo.required_unless_one.iter().map(AsRef::as_ref).collect();
                    if ruo.len() > 0 {
                        f(app.arg(clapme::clap::Arg::with_name(&name).long(&name)
                                  .requires_all(newinfo.required_flags)
                                  .conflicts_with_all(&conflicts)
                                  .required_unless_one(&ruo)
                                  .help(#doc)))
                    } else {
                        f(app.arg(clapme::clap::Arg::with_name(&name).long(&name)
                                  .requires_all(newinfo.required_flags)
                                  .conflicts_with_all(&conflicts)
                                  .help(#doc)))
                    }
                };
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
    let generics = &input.generics;
    let find_prefix = quote!{
        let prefix: String = match name.chars().next() {
            None | Some('_') | Some('-') => "".to_string(),
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
            let with_clap_stuff = with_clap_fields(syn::Fields::Named(fields.clone()),
                                                   None);
            let return_struct = return_with_fields(syn::Fields::Named(fields.clone()),
                                                   quote!(#name));
            quote!{
                fn with_clap<ClapMeT>(mut info: clapme::ArgInfo,
                                app: clapme::clap::App,
                                f: impl FnOnce(clapme::clap::App) -> ClapMeT)
                                -> ClapMeT {
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
            let vnames: Vec<_> = e.variants.iter().map(|v| camel_case_to_kebab(&v.ident.to_string())).collect();
            let vnames2 = vnames.clone();
            let vnames3 = vnames.clone();
            let vnames4 = vnames.clone();
            let vnames5 = vnames.clone();
            let vnames6 = vnames.clone();
            // println!("variant names are {:?}", names);
            let fields: Vec<_> = v.iter().map(|x| x.fields.clone()).collect();
            let with_claps: Vec<_> = v.iter().map(|v| {
                let d = get_doc_comment(&v.attrs);
                with_clap_fields(v.fields.clone(), Some(d))
            }).collect();
            // println!("variant with_claps are {:?}", with_claps);
            let one_field: Vec<_> = fields.iter().map(|f| one_field_name(f.clone())).collect();
            let one_field2 = one_field.clone();
            let one_field3 = one_field.clone();
            let return_enum = v.iter().map(|v| {
                let variant_name = v.ident.clone();
                return_with_fields(v.fields.clone(), quote!(#name::#variant_name))
            });
            let s = quote! {
                fn with_clap<ClapMeT>(mut info: clapme::ArgInfo,
                                app: clapme::clap::App,
                                f: impl FnOnce(clapme::clap::App) -> ClapMeT)
                                -> ClapMeT {
                    let name = info.name;
                    #find_prefix
                    let orig_prefix = prefix.clone();
                    info.multiple = false;

                    let mut conflicts: Vec<String> = Vec::new();
                    #(
                        let name = format!("{}-{}", info.name, #vnames3);
                        let prefix = format!("{}{}-", orig_prefix, #vnames4);
                        conflicts.push(#one_field2);
                    )*

                    let original_conflicted = info.conflicted_flags.clone();
                    let original_required_unless = info.required_unless_one.clone();
                    let am_required = info.required || original_required_unless.len() > 0;
                    info.required = false;
                    #(
                        let name = format!("{}-{}", info.name, #vnames);
                        let prefix = format!("{}{}-", orig_prefix, #vnames2);
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
                    let orig_prefix = prefix;
                    let orig_name = name;
                    #(
                        let name = format!("{}-{}", orig_name, #vnames5);
                        let prefix = format!("{}{}-", orig_prefix, #vnames6);
                        if matches.is_present(#one_field) {
                            #return_enum
                        }
                    )*
                    panic!("Some version of the enum should be present!")
                }
            };
            s
        },
        _ => panic!("ClapMe only supports non-tuple structs and enums"),
    };

    let generic_types = input.generics.type_params();
    let bounds = quote!{
        <#(#generic_types: clapme::ClapMe),*>
    };

    let tokens2: proc_macro2::TokenStream = quote!{
        impl#bounds ClapMe for #name#generics {
            #myimpl
        }
    };
    // println!("\n\nXXXX\n\nXXXX\n\n{}", tokens2);
    tokens2.into()
}

fn camel_case_to_kebab(name: &str) -> String {
    if name.contains('_') {
        let mut out = name.to_string().replace("_", "-");
        if out.chars().last() == Some('-') {
            out.pop();
        }
        out
    } else {
        let mut out = String::new();
        let mut am_on_cap = true;
        for c in name.chars() {
            if !am_on_cap && c.is_ascii_uppercase() {
                out.push('-');
            }
            am_on_cap = c.is_ascii_uppercase();
            out.push(c.to_ascii_lowercase());
        }
        out
    }
}

fn snake_case_to_kebab(name: &str) -> String {
    name.to_string().replace("_", "-")
}
