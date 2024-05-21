extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn must_use_with_msg(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute arguments
    let args = parse_macro_input!(attr as AttributeArgs);
    let msg = if let Some(NestedMeta::Lit(Lit::Str(lit_str))) = args.first() {
        lit_str.value()
    } else {
        "Default message here".to_string()
    };

    // Parse the input item (the function)
    let input = parse_macro_input!(item as ItemFn);

    // Extract the function components
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    // Generate the new function with the must_use attribute
    let gen = quote! {
        #[must_use = #msg]
        #vis #sig #block
    };
    gen.into()
}

