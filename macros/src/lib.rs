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
        for i in 0..set.count.base10_parse().unwrap() {
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
