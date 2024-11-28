trait MyTrait {
    fn one() -> Self;
    fn zero() -> Self;
    fn negative_one() -> Self;
}

macro_rules! impl_my_trait_for_primitive {
    ($($t:ty)*) => {
        $(
            impl MyTrait for $t {
                fn one() -> Self {
                    1
                }
                fn zero() -> Self {
                    0
                }
                fn negative_one() -> Self {
                    -1
                }
            }
        )*
    }
}

impl_my_trait_for_primitive!(i8 i16 i32 i64 i128 isize);

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_my_trait(&ast)
}

fn impl_my_trait(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MyTrait for #name {
            fn one() -> Self {
                1
            }
            fn zero() -> Self {
                0
            }
            fn negative_one() -> Self {
                -1
            }
        }
    };
    gen.into()
}

