// Copyright (C) 2024 Takayuki Sato. All Rights Reserved.
// This program is free software under MIT License.
// See the file LICENSE in this distribution for more details.

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn my_attr_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{attr:?}");
    println!("{item:?}");

    item
}
