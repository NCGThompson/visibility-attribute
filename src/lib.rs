//! # Documentation: Conditional Compilation
//! 
//! `#[set_visibility]` by itself isn't very useful. It is designed
//! to be used with [`cfg_attr`](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute)
//! instead. For example, this will only compile in doc tests:
//! ```
//! mod squaring {
//!     use visibility_attribute::set_visibility;
//!     #[cfg_attr(not(test), set_visibility(pub(super)))]
//!     fn square(num: i32) -> i32 {
//!         num * num
//!     }
//! }
//! assert_eq!(squaring::square(5), 25);
//! ```
//! [`doctest`](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#include-items-only-when-collecting-doctests)
//! as a predicate in particular is useful for writing internal
//! documentation for private functions.
//! 
//! The reason for relying on `cfg_attr` instead of building the logic
//! into `set_visibility` is so that the procedural macro doesn't have to
//! run when it isn't used. If the predicate is dependent on a test
//! (e.g. `test` or `doctest`), a feature, or a platform (e.g.
//! `target_family = "wasm"`), the crate doesn't have to be compiled
//! either. See [Cargo's documentation](https://doc.rust-lang.org/cargo/reference/index.html)
//! for details.
//! 
//! ### Development dependency
//! When only using the `visibility_attribute` crate for doc tests
//! or related targets, list it under
//! [`[dev-dependencies]`](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies)
//! rather than `[dependencies]` in `Cargo.toml`
//! ```
//! #[cfg(doctest)]
//! use visibility_attribute::set_visibility;
//! #[cfg_attr(doctest, set_visibility(pub(super)))]
//! fn square(num: i32) -> i32 {
//!     num * num
//! }
//! ```
//! `Cargo.toml`:
//! ``` toml
//! [dev-dependencies]
//! visibility_attribute = { git = "https://url.git" }
//! ```

#[cfg(test)]
mod tests;

#[proc_macro_attribute]
/// Replace the visibility modifier with the input.
/// 
/// # Examples
/// ```
/// mod squaring {
///     use visibility_attribute::set_visibility;
///     #[set_visibility(pub(super))]
///     fn square(num: i32) -> i32 {
///         num * num
///     }
/// }
/// 
/// assert_eq!(squaring::square(5), 25);
/// ```
/// ``` compile_fail
/// mod squaring {
///     use visibility_attribute::set_visibility;
///     #[set_visibility]
///     pub(super) fn square(num: i32) -> i32 {
///         num * num
///     }
/// }
/// 
/// assert_eq!(squaring::square(5), 25); // shouldn't compile!
/// ```
pub fn set_visibility(
    input: proc_macro::TokenStream,
    annotated_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    inner_set_visibility(input.into(), annotated_item.into()).into()
}

macro_rules! impl_macro {
    ($sv:ident, $rv:ident, $pmc:ident) => {
        /// Implements the actual logic.
        fn $sv(
            input: $pmc::TokenStream,
            annotated_item: $pmc::TokenStream,
        ) -> $pmc::TokenStream {
            let mut out_stream = input;
            out_stream.extend($rv(annotated_item));
            out_stream
        }

        /// Removes the visibility modifier from a TokenTree iterable.
        ///
        /// It returns an iterator rather than a TokenStream.
        /// This function should be agnostic to spans.
        fn $rv(
            input: impl IntoIterator<Item = $pmc::TokenTree>,
        ) -> impl Iterator<Item = $pmc::TokenTree> {
            let mut tt_iter = input.into_iter().peekable();

            if tt_iter
                .next_if(|x| match x {
                    $pmc::TokenTree::Ident(y) => *y.to_string() == *"pub",
                    _ => false,
                })
                .is_none()
            {
                return tt_iter;
            }

            tt_iter.next_if(|x| match x {
                $pmc::TokenTree::Group(y) => y.delimiter() == $pmc::Delimiter::Parenthesis,
                _ => false,
            });

            tt_iter
        }
    };
}
use impl_macro; 

impl_macro!(inner_set_visibility, remove_visibility, proc_macro);
