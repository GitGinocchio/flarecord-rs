use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ItemStruct};

#[proc_macro_attribute]
pub fn command(_args: TokenStream, input: TokenStream) -> TokenStream {
    // 1. Proviamo a parsare come funzione
    if let Ok(func) = syn::parse::<ItemFn>(input.clone()) {
        return generate_fn_command(func);
    }
    
    // 2. Proviamo a parsare come struct
    if let Ok(strct) = syn::parse::<ItemStruct>(input) {
        return generate_struct_command(strct);
    }

    panic!("#[command] può essere usato solo su struct o fn")
}

fn generate_fn_command(func: ItemFn) -> TokenStream {
    let name = &func.sig.ident;
    let struct_name = syn::Ident::new(&format!("Cmd_{}", name), name.span());
    
    quote! {
        #func
        
        pub struct #struct_name;
        impl Command for #struct_name {
            async fn execute(&self, req: Request, env: Env) -> Result<Response> {
                #name(req, env).await
            }
        }
        
        ::flarecord::inventory::submit! {
            ::flarecord::CommandRegistration {
                constructor: || ::std::sync::Arc::new(#struct_name)
            }
        }
    }.into()
}

fn generate_struct_command(strct: ItemStruct) -> TokenStream {
    let name = &strct.ident;
    quote! {
        #strct
        ::flarecord::inventory::submit! {
            ::flarecord::CommandRegistration {
                constructor: || ::std::sync::Arc::new(#name)
            }
        }
    }.into()
}