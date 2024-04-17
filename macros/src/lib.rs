use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitInt, LitStr, Token,
};

struct ImageSets {
    sets: Vec<ImageSet>,
}

impl Parse for ImageSets {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut sets = Vec::new();

        while input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
            let set = input.parse()?;
            sets.push(set);
        }

        Ok(Self { sets })
    }
}

struct ImageSet {
    prefix: LitStr,
    count: LitInt,
}

impl Parse for ImageSet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let prefix = input.parse()?;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
        let count = input.parse()?;
        Ok(Self { prefix, count })
    }
}

#[proc_macro]
pub fn include_piece_images(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ImageSets);

    let mut filenames = Vec::new();
    for set in input.sets {
        for i in 0..set.count.base10_parse().unwrap_or(0) {
            filenames.push(format!("{}{i:02}.png", set.prefix.value()));
        }
    }

    let output = quote! {
        vec![
            #(
                egui::Image::new(egui::include_image!(#filenames))
            ),*
        ]
    };

    output.into()
}

#[proc_macro_derive(EnumVariantCount)]
pub fn enum_variant_count_derive(input: TokenStream) -> TokenStream {
    let Ok(syn_item) = syn::parse::<syn::DeriveInput>(input) else {
        panic!("Parsing enum token input failed");
    };

    let (enum_token, len) = match syn_item.data {
        syn::Data::Enum(enum_item) => (syn_item.ident, enum_item.variants.len()),
        _ => panic!("EnumVariantCount can only be used with enums!"),
    };

    let expanded = quote! {
      impl #enum_token {
        pub const fn count() -> usize {
            #len
        }
      }
    };

    expanded.into()
}
