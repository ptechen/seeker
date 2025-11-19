use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit, Meta};

#[proc_macro_derive(SeekerEmbedFolder, attributes(folder_path))]
pub fn embed_folder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // 获取结构体名称
    let name = input.ident;

    // 查找 folder_path 属性
    let mut folder_path = String::new();
    for attr in &input.attrs {
        if attr.path().is_ident("folder_path") {
            if let Meta::NameValue(meta_name_value) = &attr.meta {
                if let syn::Expr::Lit(expr_lit) = &meta_name_value.value {
                    if let Lit::Str(lit_str) = &expr_lit.lit {
                        folder_path = lit_str.value();
                    }
                }
            }
        }
    }
    if folder_path.is_empty() {
        panic!("folder_path attribute is required");
    }

    let list = embed_file(&folder_path);
    let expanded = quote! {
        impl bevy::prelude::Plugin for #name {
            fn build(&self, app: &mut bevy::prelude::App) {
                #(#list)*
            }
        }
    };
    TokenStream::from(expanded)
}

fn embed_file(folder_path: &str) -> Vec<proc_macro2::TokenStream> {
    let Ok(read_dir) = std::fs::read_dir(folder_path) else {
        panic!("folder_path is not a valid directory");
    };

    let mut list = vec![];
    for entry in read_dir {
        let Ok(entry) = entry else {
            continue;
        };
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if file_type.is_file() {
            let file_path = entry.path();
            let file_path = file_path.to_string_lossy().replace("./src/", "");
            let token = quote! {
                bevy::asset::embedded_asset!(app, #file_path);
            };
            list.push(token);
        } else if file_type.is_dir() {
            let items = embed_file(&entry.path().to_string_lossy());
            list.extend(items);
        }
    }
    list
}
