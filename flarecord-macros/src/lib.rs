use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, ItemImpl, ItemStruct};

#[proc_macro_attribute]
pub fn command(_args: TokenStream, input: TokenStream) -> TokenStream {
    if let Ok(func) = syn::parse::<ItemFn>(input.clone()) {
        return generate_fn_command(func);
    }

    if let Ok(imp) = syn::parse::<ItemImpl>(input.clone()) {
        return generate_impl_command(imp);
    }
    
    if let Ok(strct) = syn::parse::<ItemStruct>(input.clone()) {
        return generate_struct_command(strct);
    }

    syn::Error::new_spanned(
        proc_macro2::TokenStream::from(input),
        "#[command] può essere usato solo su una `fn`, una `struct` o un blocco `impl Command for ...`",
    )
    .to_compile_error()
    .into()
}

fn generate_fn_command(func: ItemFn) -> TokenStream {
    let name = &func.sig.ident;
    let struct_name = syn::Ident::new(&format!("Cmd_{}", name), name.span());
    
    quote! {
        #func
        
        pub struct #struct_name;
        impl Command for #struct_name {
            async fn execute(&self, interaction: CommandInteraction, _ctx: CommandContext) -> BotResult<CommandResponse> {
                #name(interaction, _ctx).await
            }
        }
        
        ::flarecord::inventory::submit! {
            ::flarecord::CommandRegistration {
                constructor: || ::flarecord::prelude::IntoCommand::into_command(#struct_name)
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
                constructor: || ::flarecord::prelude::IntoCommand::into_command(#name)
            }
        }
    }.into()
}

fn generate_impl_command(imp: ItemImpl) -> TokenStream {
    // Estrai il tipo su cui è implementato il trait (es. `Hello`)
    let self_ty = &imp.self_ty;

    quote! {
        #[derive(Default)]
        pub struct #self_ty;

        #imp

        ::flarecord::inventory::submit! {
            ::flarecord::CommandRegistration {
                constructor: || ::flarecord::prelude::IntoCommand::into_command(#self_ty)
            }
        }
    }.into()
}