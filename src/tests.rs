use super::impl_macro;
use itertools::iproduct;
use proc_macro2::TokenStream;
use quote::quote;

impl_macro!(inner_set_visibility2, remove_visibility2, proc_macro2);

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
            remove_visibility2(quote! { #p #b })
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
            inner_set_visibility2(v.to_owned(), quote! { #p #b }).to_string(),
            quote! { #v #b }.to_string()
        );
    }
}