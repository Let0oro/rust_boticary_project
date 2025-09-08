use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};


#[proc_macro_attribute]
pub fn make_safe_if_args(_attr: TokenStream, item: TokenStream) -> TokenStream {
    
  let input = parse_macro_input!(item as ItemFn);

  let vis = &input.vis;
  let sig = &input.sig;
  let block = &input.block;

  let generated = quote! {
    #vis #sig {
      if let Some(name) = value {#block;}
      else {panic!("Expected at least one argument");}
    }
  };

  generated.into()
}