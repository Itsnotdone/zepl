use proc_macro::*;
use scene::SScene;
use syn::{parse::Parse, parse_macro_input, Ident, LitStr, Token};

#[proc_macro_attribute]
pub fn service(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);

    let tokens = quote::quote!(
        #[no_mangle]
        #item
    );

    tokens.into()
}

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);

    let tokens = quote::quote!(
        #[no_mangle]
        #item
    );

    tokens.into()
}

struct LoadScene {
    scene_loader: Ident,
    path: LitStr,
}

impl Parse for LoadScene {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let scene_loader: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let path: LitStr = input.parse()?;

        Ok(Self { scene_loader, path })
    }
}

#[proc_macro]
pub fn load_scene(tokens: TokenStream) -> TokenStream {
    let LoadScene { scene_loader, path } = parse_macro_input!(tokens as LoadScene);

    let source = std::fs::read_to_string(path.value()).unwrap();
    let scene = serde_yaml::from_str::<SScene>(&source).unwrap();

    let mut tokens = TokenStream::new();

    tokens.extend::<TokenStream>(quote::quote!(Scene::new()).into());

    for entry in &scene.entries {
        let source = std::fs::read_to_string(entry.path.clone()).unwrap();
        let name = entry.name.clone();
        tokens.extend::<TokenStream>(
            quote::quote!(
                .with_entity(#name, EntityDeserializer::deserialize_from_source(#source, #scene_loader.get_registry()))
            ).into()
        );
    }

    tokens
}
