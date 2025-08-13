// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use proc_macro::TokenStream;
use syn;

use quote::ToTokens;
use std::ops::{Deref, Fn, FnMut};

struct OverridableArgs {
    #[allow(dead_code)]
    mod_token: syn::Token![mod],

    #[allow(dead_code)]
    eq_token: syn::Token![=],

    path: syn::Path,
}

impl syn::parse::Parse for OverridableArgs {
    fn parse(args: syn::parse::ParseStream) -> syn::Result<Self> {
        let mod_token: syn::Token![mod] = args.parse()?;
        let eq_token: syn::Token![=] = args.parse()?;
        let path: syn::Path = args.parse()?;

        Ok(OverridableArgs {
            mod_token,
            eq_token,
            path,
        })
    }
}

pub struct OverridableDataAcc {
    args_ast: Option<OverridableArgs>,
    trait_ast: syn::ItemTrait,
}

impl OverridableDataAcc {
    pub fn new(args: TokenStream, item: TokenStream) -> Self {
        let trait_ast = match syn::parse::<syn::ItemTrait>(item) {
            Ok(ast) => ast,
            Err(_) => {
                panic!("`overridable` attribute-macro must be attached to a trait.");
            }
        };

        let args_string = args.to_string();
        if args_string.is_empty() {
            return Self {
                args_ast: None,
                trait_ast,
            };
        }

        let args_ast = match syn::parse::<OverridableArgs>(args) {
            Ok(args_ast) => Some(args_ast),
            Err(_err) => {
                panic!("`overridable` attribute-macro can take an argument like `mod = module::path` or empty");
            }
        };

        Self {
            args_ast,
            trait_ast,
        }
    }

    pub fn get_trait_name(&self) -> String {
        self.trait_ast.ident.to_string()
    }

    pub fn get_module_path(&self) -> Option<String> {
        if let Some(ast) = &self.args_ast {
            let mut path_string = String::new();

            if ast.path.leading_colon.is_some() {
                path_string.push_str("::");
            }

            let mut iter = ast.path.segments.iter();
            if let Some(seg) = iter.next() {
                path_string.push_str(&seg.ident.to_string());
                for seg in iter {
                    path_string.push_str("::");
                    path_string.push_str(&seg.ident.to_string());
                }
            }

            return Some(path_string);
        }

        None
    }

    pub fn list_trait_search_keys(&self) -> Vec<String> {
        let mut key_vec = Vec::<String>::new();
        key_vec.push(self.trait_ast.ident.to_string());

        if let Some(ast) = &self.args_ast {
            let mut i = ast.path.segments.len();
            while i > 0 {
                i -= 1;

                if let Some(seg) = ast.path.segments.get(i) {
                    if let Some(sub_path) = key_vec.first() {
                        let mut path = seg.to_token_stream().to_string();
                        path.push_str("::");
                        path.push_str(sub_path);
                        key_vec.insert(0, path);
                    }
                }
            }

            if ast.path.leading_colon.is_some() {
                if let Some(sub_path) = key_vec.first() {
                    let mut path = String::from("::");
                    path.push_str(sub_path);
                    key_vec.insert(0, path);
                }
            }
        }

        key_vec
    }

    pub fn for_each_method_registration<F>(&self, mut register: F)
    where
        F: FnMut(String, String, String, bool, String),
    {
        for ti in &self.trait_ast.items {
            match ti {
                syn::TraitItem::Fn(f) => {
                    if is_method(&f.sig) {
                        let name = f.sig.ident.to_string();
                        let sig = f.sig.to_token_stream().to_string();
                        let call = make_method_call(&f.sig);
                        let has_impl = f.default.is_some();
                        let search_key = make_method_search_key(&f.sig);
                        register(name, sig, call, has_impl, search_key);
                    }
                }
                _ => {}
            }
        }
    }
}

fn is_method(f_sig: &syn::Signature) -> bool {
    for fn_arg in &f_sig.inputs {
        match fn_arg {
            syn::FnArg::Receiver(_) => return true,
            _ => {}
        }
    }

    false
}

fn make_method_call(f_sig: &syn::Signature) -> String {
    let mut call_string = f_sig.ident.to_string();
    call_string.push_str("(");

    let mut iter = f_sig.inputs.iter();
    if let Some(fn_arg) = iter.next() {
        match fn_arg {
            syn::FnArg::Receiver(_) => call_string.push_str("self"),
            syn::FnArg::Typed(t) => {
                call_string.push_str(t.pat.to_token_stream().to_string().as_str());
            }
        }

        for fn_arg in iter {
            call_string.push_str(", ");

            match fn_arg {
                syn::FnArg::Receiver(_) => call_string.push_str("self"),
                syn::FnArg::Typed(t) => {
                    call_string.push_str(t.pat.to_token_stream().to_string().as_str());
                }
            }
        }
    }
    call_string.push_str(")");

    call_string
}

fn make_method_search_key(f_sig: &syn::Signature) -> String {
    let mut search_key = f_sig.ident.to_string();
    search_key.push_str("(");

    let mut iter = f_sig.inputs.iter();
    if let Some(fn_arg) = iter.next() {
        match fn_arg {
            syn::FnArg::Receiver(r) => {
                search_key.push_str(r.to_token_stream().to_string().as_str());
            }
            syn::FnArg::Typed(t) => {
                search_key.push_str(t.ty.deref().to_token_stream().to_string().as_str());
            }
        }

        for fn_arg in iter {
            search_key.push_str(", ");

            match fn_arg {
                syn::FnArg::Receiver(r) => {
                    search_key.push_str(r.to_token_stream().to_string().as_str());
                }
                syn::FnArg::Typed(t) => {
                    search_key.push_str(t.ty.deref().to_token_stream().to_string().as_str());
                }
            }
        }
    }
    search_key.push_str(")");

    search_key
}

struct OverrideWithArgs {
    trait_paths: Vec<syn::Path>,
}

impl syn::parse::Parse for OverrideWithArgs {
    fn parse(args: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut trait_paths: Vec<syn::Path> = Vec::new();

        let path: syn::Path = args.parse()?;
        trait_paths.push(path);

        while !args.is_empty() {
            let _ = args.parse::<syn::Token![,]>()?;
            let path = args.parse::<syn::Path>()?;
            trait_paths.push(path);
        }

        Ok(OverrideWithArgs { trait_paths })
    }
}

pub struct OverrideWithDataAcc {
    arg_trait_paths: Vec<syn::Path>,
    impl_trait_path: syn::Path,
    impl_method_keys: Vec<String>,
    overriding_method_impls: Vec<String>,
}

impl OverrideWithDataAcc {
    pub fn new(args: TokenStream, item: TokenStream) -> Self {
        let arg_trait_paths = match syn::parse::<OverrideWithArgs>(args) {
            Ok(args) => args.trait_paths,
            Err(_) => {
                panic!("`override_with` attribute-macro expected trait paths as arguments");
            }
        };

        let item_ast = match syn::parse::<syn::ItemImpl>(item) {
            Ok(ast) => ast,
            Err(_) => {
                panic!("`override_with` attribute-macro must be attached to a `impl` block");
            }
        };

        if let Some((_, impl_trait_path, _)) = item_ast.trait_ {
            let mut impl_method_keys = Vec::<String>::new();

            for ii in &item_ast.items {
                match ii {
                    syn::ImplItem::Fn(f) => {
                        impl_method_keys.push(make_method_search_key(&f.sig));
                    }
                    _ => {}
                }
            }

            return Self {
                arg_trait_paths,
                impl_trait_path,
                impl_method_keys,
                overriding_method_impls: Vec::new(),
            };
        } else {
            panic!("Not trait implementation.");
        }
    }

    pub fn get_impl_trait<T, F>(&self, fetcher: F) -> Option<T>
    where
        F: Fn(&[String]) -> Option<T>,
    {
        let mut key_vec = Vec::<String>::new();

        let mut i = self.impl_trait_path.segments.len();
        while i > 0 {
            i -= 1;

            if let Some(seg) = self.impl_trait_path.segments.get(i) {
                if let Some(sub_path) = key_vec.first() {
                    let mut path = seg.to_token_stream().to_string();
                    path.push_str("::");
                    path.push_str(sub_path);
                    key_vec.insert(0, path);
                } else {
                    key_vec.push(seg.ident.to_string());
                }
            }
        }

        if self.impl_trait_path.leading_colon.is_some() {
            if let Some(sub_path) = key_vec.first() {
                let mut path = String::from("::");
                path.push_str(sub_path);
                key_vec.insert(0, path);
            }
        }

        fetcher(&key_vec)
    }

    pub fn list_impl_method_keys(&self) -> &[String] {
        &self.impl_method_keys
    }

    pub fn list_argument_traits<T, F>(&self, fetcher: F) -> Vec<T>
    where
        F: Fn(&[String], String) -> Option<T>,
    {
        let mut vec = Vec::<T>::new();

        for arg_trait_path in &self.arg_trait_paths {
            let mut key_vec = Vec::<String>::new();

            let mut i = arg_trait_path.segments.len();
            while i > 0 {
                i -= 1;

                if let Some(seg) = arg_trait_path.segments.get(i) {
                    if let Some(sub_path) = key_vec.first() {
                        let mut path = seg.to_token_stream().to_string();
                        path.push_str("::");
                        path.push_str(sub_path);
                        key_vec.insert(0, path);
                    } else {
                        key_vec.push(seg.ident.to_string());
                    }
                }
            }

            if arg_trait_path.leading_colon.is_some() {
                if let Some(sub_path) = key_vec.first() {
                    let mut path = String::from("::");
                    path.push_str(sub_path);
                    key_vec.insert(0, path);
                }
            }

            if let Some(t) = fetcher(&key_vec, arg_trait_path.to_token_stream().to_string()) {
                vec.push(t)
            }
        }

        vec
    }

    pub fn set_overriding_method_impls(&mut self, vec: Vec<String>) {
        self.overriding_method_impls = vec;
    }

    pub fn output_result(&self, item: TokenStream) -> TokenStream {
        let mut impl_ast = syn::parse::<syn::ItemImpl>(item).unwrap();

        for method in &self.overriding_method_impls {
            let fn_ast = syn::parse_str::<syn::ImplItemFn>(&method).unwrap();
            impl_ast.items.push(syn::ImplItem::Fn(fn_ast));
        }

        quote::quote! { #impl_ast }.into()
    }
}
