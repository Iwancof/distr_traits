extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(NormalSample)]
pub fn normal_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    fn init_assoc(fields: syn::punctuated::Punctuated<syn::Field, syn::token::Comma>) -> (Vec<proc_macro2::TokenStream>, proc_macro2::TokenStream) {
        let field_initializers = fields.iter().map(|field| {
            let ty = &field.ty;
            quote! { <#ty as distr_traits::normal::NormalSample>::normal_sample(mean, variance, rng) }
        }).collect::<Vec<_>>();

        let declare_associated = {
            let first_type = &fields.first().unwrap().ty;
            quote! {
                type Mean = <#first_type as distr_traits::normal::NormalSample>::Mean;
                type Variance = <#first_type as distr_traits::normal::NormalSample>::Variance;
            }
        };

        (field_initializers, declare_associated)
    }

    let expanded = match input.data {
        Data::Struct(ref data) => {
            let (self_init, declare_associated) = match &data.fields {
                Fields::Named(ref fields) => {
                    let (field_initializers, declare_associated) = init_assoc(fields.named.clone());

                    let field_initializers = fields.named.iter().zip(field_initializers.iter()).map(|(field, init)| {
                        let field_name = field.ident.as_ref().unwrap();
                        quote! { #field_name: #init }
                    }).collect::<Vec<_>>();

                    let self_init = quote! {
                        #name {
                            #(#field_initializers,)*
                        }
                    };

                    (self_init, declare_associated)
                }
                Fields::Unnamed(ref fields) => {
                    let (field_initializers, declare_associated) = init_assoc(fields.unnamed.clone());

                    let self_init = quote! {
                        #name (
                            #(#field_initializers,)*
                        )
                    };

                    (self_init, declare_associated)
                }
                Fields::Unit => panic!("Found a unit struct"),
            };

            let expanded = quote! {
                impl distr_traits::normal::NormalSample for #name {
                    #declare_associated

                    fn normal_sample(mean: Self::Mean, variance: Self::Variance, rng: &mut impl rand::Rng) -> Self {
                        #self_init
                    }
                }
            };

            TokenStream::from(expanded)
        }
        _ => panic!("Only structs are supported"),
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(UniformSample)]
pub fn uniform_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    fn init_assoc(fields: syn::punctuated::Punctuated<syn::Field, syn::token::Comma>) -> (Vec<proc_macro2::TokenStream>, proc_macro2::TokenStream) {
        let field_initializers = fields.iter().map(|field| {
            let ty = &field.ty;
            quote! { <#ty as distr_traits::uniform::UniformSample>::uniform_sample(rng) }
        }).collect::<Vec<_>>();

        let declare_associated = {
            quote! {
            }
        };

        (field_initializers, declare_associated)
    }

    let expanded = match input.data {
        Data::Struct(ref data) => {
            let (self_init, declare_associated) = match &data.fields {
                Fields::Named(ref fields) => {
                    let (field_initializers, declare_associated) = init_assoc(fields.named.clone());

                    let field_initializers = fields.named.iter().zip(field_initializers.iter()).map(|(field, init)| {
                        let field_name = field.ident.as_ref().unwrap();
                        quote! { #field_name: #init }
                    }).collect::<Vec<_>>();

                    let self_init = quote! {
                        #name {
                            #(#field_initializers,)*
                        }
                    };

                    (self_init, declare_associated)
                }
                Fields::Unnamed(ref fields) => {
                    let (field_initializers, declare_associated) = init_assoc(fields.unnamed.clone());

                    let self_init = quote! {
                        #name (
                            #(#field_initializers,)*
                        )
                    };

                    (self_init, declare_associated)
                }
                Fields::Unit => panic!("Found a unit struct"),
            };

            let expanded = quote! {
                impl distr_traits::uniform::UniformSample for #name {
                    #declare_associated

                    fn uniform_sample(rng: &mut impl rand::Rng) -> Self {
                        #self_init
                    }
                }
            };

            TokenStream::from(expanded)
        }
        _ => panic!("Only structs are supported"),
    };

    TokenStream::from(expanded)
}
