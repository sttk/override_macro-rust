// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use proc_macro::TokenStream;

mod logic;
mod syn_dax;

#[proc_macro_attribute]
pub fn overridable(args: TokenStream, item: TokenStream) -> TokenStream {
    let dax = syn_dax::OverridableDax::new(args, item.clone());
    logic::collect_trait_info(&dax);

    item
}

#[proc_macro_attribute]
pub fn override_with(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut dax = syn_dax::OverrideWithDax::new(args, item.clone());
    logic::override_trait_methods(&mut dax);

    dax.output_result(item)
}
