use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
  parse_macro_input, parse_quote,
  token::{Comma, Dot2},
  visit_mut::VisitMut,
  ExprStruct,
};

struct Handcrafter;

impl VisitMut for Handcrafter {
  fn visit_expr_struct_mut(&mut self, struct_expr: &mut ExprStruct) {
    syn::visit_mut::visit_expr_struct_mut(self, struct_expr);

    // Check existing ..rest
    if struct_expr.dot2_token.is_none() && struct_expr.rest.is_none() {
      // Make sure fields have trailing comma
      if !struct_expr.fields.empty_or_trailing() {
        struct_expr.fields.push_punct(Comma::default());
      }

      // Add ..Default::default()
      struct_expr.dot2_token = Some(Dot2::default());
      struct_expr.rest = Some(Box::new(parse_quote! {
          ::core::default::Default::default()
      }));
    }
  }
}

#[proc_macro]
pub fn handcraft(tokens: TokenStream) -> TokenStream {
  let mut item = parse_macro_input!(tokens);

  let mut hc = Handcrafter {};

  hc.visit_expr_struct_mut(&mut item);
  item.into_token_stream().into()
}
