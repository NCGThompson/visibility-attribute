use std::iter::Peekable;

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

fn inner_set_visibility(
    input: proc_macro2::TokenStream,
    annotated_item: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let mut out_stream = input;
    out_stream.extend(remove_visibility(annotated_item));
    out_stream
}

/// Removes the visibility modifier from a TokenTree iterable.
///
/// This function should be agnostic to spans.
fn remove_visibility(
    input: impl IntoIterator<Item = proc_macro2::TokenTree>,
) -> Peekable<impl Iterator<Item = proc_macro2::TokenTree>> {
    let mut tt_iter = input.into_iter().peekable();

    if tt_iter
        .next_if(|x| match x {
            proc_macro2::TokenTree::Ident(y) => *y.to_string() == *"pub",
            _ => false,
        })
        .is_none()
    {
        return tt_iter;
    }

    tt_iter.next_if(|x| match x {
        proc_macro2::TokenTree::Group(y) => y.delimiter() == proc_macro2::Delimiter::Parenthesis,
        _ => false,
    });

    tt_iter
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::iproduct;
    use proc_macro2::TokenStream;
    use quote::quote;

    fn get_sample_streams() -> (Box<[TokenStream]>, Box<[TokenStream]>) {
        (
            Box::new([
                TokenStream::new(),
                quote! { pub },
                quote! { pub(crate) },
                quote! { pub(super) },
                quote! { pub(super::super) },
                quote! { pub(super::super::super) },
                quote! { pub() },
                quote! { pub(! this is ; nonsense) },
            ]),
            Box::new([
                TokenStream::new(),
                quote! { 5 },
                quote! { let mut four = 2.add(2) },
                quote! { fn add(a: i32, b: i32) -> i32 { a + b } },
                quote! { [] },
                quote! { {super} },
            ]),
        )
    }

    #[test]
    fn remove_visibility_test() {
        let (prefixes, bases) = get_sample_streams();
        let comb = iproduct!(bases.iter(), prefixes.iter());

        for (b, p) in comb {
            assert_eq!(
                remove_visibility(quote! { #p #b })
                    .collect::<TokenStream>()
                    .to_string(),
                b.to_string()
            );
        }
    }

    #[test]
    fn inner_set_visibility_test() {
        let (prefixes, bases) = get_sample_streams();
        let comb = iproduct!(prefixes.iter(), bases.iter(), prefixes.iter());

        for (v, b, p) in comb {
            assert_eq!(
                inner_set_visibility(v.to_owned(), quote! { #p #b }).to_string(),
                quote! { #v #b }.to_string()
            );
        }
    }
}
