use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Hello)]
pub fn hello(_item: TokenStream) -> TokenStream {
    let add_hello = quote! {};
    add_hello.into()
}
