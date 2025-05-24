#![doc = include_str!("../README.md")]

mod input;
mod with_ext;
mod with_traits;

use input::GridArgs;
use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Generates types for grid navigation.
///
/// ```rust,ignore
/// use typed_grid::*;
///
/// typed_grid!(2, 2);
///
/// impl Moved for i32 {
///     fn moved(&mut self, p: Position) {
///         *self += 1;
///         println!("MOVED: {p:?} {self}")
///     }
/// }
/// ```
#[proc_macro]
pub fn typed_grid(input: TokenStream) -> TokenStream {
    let GridArgs { cols, rows } = parse_macro_input!(input as GridArgs);

    with_traits::typed_grid(cols, rows).into()
}

/// Generates types for grid navigation with extension methods.
///
/// ```rust,ignore
/// use typed_grid::*;
///
/// typed_grid_ext!(2, 2);
///
/// impl Moved for i32 {
///     fn moved(&mut self, p: Position) {
///         *self += 1;
///         println!("MOVED: {p:?} {self}")
///     }
/// }
/// ```
#[proc_macro]
pub fn typed_grid_ext(input: TokenStream) -> TokenStream {
    let GridArgs { cols, rows } = parse_macro_input!(input as GridArgs);

    with_ext::typed_grid_ext(cols, rows).into()
}
