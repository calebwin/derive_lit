//! This is a crate for automatically generating macros that expand to literal values of your data structure. Here's an example.
//! ```
//! # use derive_lit::VecLit;
//! # #[derive(VecLit)]
//! # struct MyStruct;
//! # impl MyStruct { fn new() -> Self {Self{}} fn push(&mut self, elem: usize) {}}
//! let x: MyStruct = my_struct! [0, 9, 3, 4, 5];
//! ```

extern crate proc_macro;

use heck::*;
use quote::{quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Ident
};


/// A derive for auto-generating a macro to create literal values for vec-like data structures
///
/// The vec-like data structure must have the following methods-
/// - `fn new() -> Self`
/// - `fn push(elem)`
///
/// The auto-generated macro will be of the following form-
/// ```
/// # use derive_lit::VecLit;
/// # #[derive(VecLit)]
/// # struct MyStruct;
/// # impl MyStruct { fn new() -> Self {Self{}} fn push(&mut self, elem: usize) {}}
/// let x: MyStruct = my_struct! [0, 9, 3, 4, 5];
/// ```
#[proc_macro_derive(VecLit)]
pub fn derive_vec_lit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    let macro_name = Ident::new(&name.to_string().to_snake_case(), name.span());
    let struct_name = name.clone();

    if let Data::Struct(_) = data {
    } else {
        // TODO throw error
        panic!("expected a struct")
    }

    let expanded = quote! {
        macro_rules! #macro_name {
            ( $( $elem:expr ),* ) => {
                {
                    let mut temp = #struct_name::new();
                    $(
                        temp.push($elem);
                    )*
                    temp
                }
            };	
        }
    };

    // hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

/// A derive for auto-generating a macro to create literal values for vec-like data structures with a front at right end
///
/// The vec-like data structure must have the following methods-
/// - `fn new() -> Self`
/// - `fn push_front(elem)`
///
/// The auto-generated macro will be of the following form-
/// ```
/// # use derive_lit::VecFrontLit;
/// # #[derive(VecFrontLit)]
/// # struct MyStruct;
/// # impl MyStruct { fn new() -> Self {Self{}} fn push_front(&mut self, elem: usize) {}}
/// let x: MyStruct = my_struct! [0, 9, 3, 4, 5]; // front at right
/// ```
#[proc_macro_derive(VecFrontLit)]
pub fn derive_vec_front_lit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    let macro_name = Ident::new(&name.to_string().to_snake_case(), name.span());
    let struct_name = name.clone();

    if let Data::Struct(_) = data {
    } else {
        // TODO throw error
        panic!("expected a struct")
    }

    let expanded = quote! {
        macro_rules! #macro_name {
            ( $( $elem:expr ),* ) => {
                {
                    let mut temp = #struct_name::new();
                    $(
                        temp.push_front($elem);
                    )*
                    temp
                }
            };	
        }
    };

    // hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

/// A derive for auto-generating a macro to create literal values for set-like data structures
///
/// The set-like data structure must have the following methods-
/// - `fn new() -> Self`
/// - `fn insert(elem)`
///
/// The auto-generated macro will be of the following form-
/// ```
/// # use derive_lit::SetLit;
/// # #[derive(SetLit)]
/// # struct MyStruct;
/// # impl MyStruct { fn new() -> Self {Self{}} fn insert(&mut self, elem: usize) {}}
/// let x: MyStruct = my_struct! {0, 9, 3, 4, 5};
/// ```
#[proc_macro_derive(SetLit)]
pub fn derive_set_lit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    let macro_name = Ident::new(&name.to_string().to_snake_case(), name.span());
    let struct_name = name.clone();

    if let Data::Struct(_) = data {
    } else {
        // TODO throw error
        panic!("expected a struct")
    }

    let expanded = quote! {
        macro_rules! #macro_name {
            ( $( $elem:expr ),* ) => {
                {
                    let mut temp = #struct_name::new();
                    $(
                        temp.insert($elem);
                    )*
                    temp
                }
            };	
        }
    };

    // hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

/// A derive for auto-generating a macro to create literal values for map-like data structures
///
/// The map-like data structure must have the following methods-
/// - `fn new() -> Self`
/// - `fn insert(key, val)`
///
/// The auto-generated macro will be of the following form-
/// ```
/// # use derive_lit::MapLit;
/// # #[derive(MapLit)]
/// # struct MyStruct;
/// # impl MyStruct { fn new() -> Self {Self{}} fn insert(&mut self, key: &'static str, val: usize) {}}
/// let x: MyStruct = my_struct! {
///     "a" => 0,
///     "b" => 0,
///     "c" => 7
/// };
/// ```
#[proc_macro_derive(MapLit)]
pub fn derive_map_lit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data = input.data;

    let macro_name = Ident::new(&name.to_string().to_snake_case(), name.span());
    let struct_name = name.clone();

    if let Data::Struct(_) = data {
    } else {
        // TODO throw error
        panic!("expected a struct")
    }

    let expanded = quote! {
        macro_rules! #macro_name(
		    { $($key:expr => $val:expr),* } => {
		        {
		            let mut temp = #struct_name::new();
		            $(
		                temp.insert($key, $val);
		            )*
		            
		            temp
		        }
		     };
		);
    };

    // hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}