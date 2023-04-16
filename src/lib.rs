extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Named)]
pub fn derive_named(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);
    derive_named_impl(derive_input).into()
}

fn derive_named_impl(input: DeriveInput) -> TokenStream2 {
    let ident = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let where_predicates = if let Some(where_clause) = where_clause {
        where_clause.predicates.iter().collect::<Vec<_>>()
    } else {
        Default::default()
    };

    let const_params = input
        .generics
        .const_params()
        .map(|param| &param.ident)
        .collect::<Vec<_>>();
    let ty_params = input.generics.type_params().collect::<Vec<_>>();

    let out = quote!(
        impl #impl_generics rust_gpu_bridge::Named for #ident #ty_generics
        where
            #(#where_predicates),*
            #(#ty_params: rust_gpu_bridge::Named),*
    {
            fn module() -> rust_gpu_bridge::String {
                rust_gpu_bridge::ToString::to_string(module_path!())
            }

            fn short_name() -> rust_gpu_bridge::String {
                rust_gpu_bridge::format!("{}::<{}>", stringify!(#ident), {
                    let params: &[rust_gpu_bridge::String] = &[
                        #(rust_gpu_bridge::ToString::to_string(&#const_params)),*
                        #(<#ty_params as rust_gpu_bridge::Named>::short_name()),*
                    ];

                    params
                        .into_iter()
                        .enumerate().map(|(i, param)| if i == 0 {
                            rust_gpu_bridge::ToString::to_string(param)
                        } else {
                            rust_gpu_bridge::ToString::to_string(", ") + &param
                        })
                        .collect::<rust_gpu_bridge::String>()
                })
            }

            fn name() -> rust_gpu_bridge::String {
                let module = Self::module();
                rust_gpu_bridge::format!(
                    "{}{}::<{}>",
                    if module.is_empty() {
                        rust_gpu_bridge::ToString::to_string("")
                    } else {
                        module + "::"
                    },
                    stringify!(#ident),
                    {
                        let params: &[rust_gpu_bridge::String] = &[
                            #(rust_gpu_bridge::ToString::to_string(&#const_params)),*
                            #(<#ty_params as rust_gpu_bridge::Named>::name()),*
                        ];

                        params
                            .into_iter()
                            .enumerate().map(|(i, param)| if i == 0 {
                                rust_gpu_bridge::ToString::to_string(param)
                            } else {
                                rust_gpu_bridge::ToString::to_string(", ") + &param
                            })
                            .collect::<rust_gpu_bridge::String>()
                    }
                )
            }
        }
    );

    //eprintln!("Derive named:\n{out}");

    out
}
