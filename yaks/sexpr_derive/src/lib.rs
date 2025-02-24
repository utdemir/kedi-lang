use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SExpr)]
pub fn derive_sexpr(input: TokenStream) -> TokenStream {
    // Parse the input TokenStream into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Build the impl
    impl_sexpr(&input)
}

fn impl_sexpr(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    match &ast.data {
        syn::Data::Struct(ref data_struct) => {
            match &data_struct.fields {
                // For structs with named fields
                syn::Fields::Named(ref fields_named) => {
                    let field_exprs: Vec<_> = fields_named
                        .named
                        .iter()
                        .map(|f| {
                            let field_name = f.ident.as_ref().unwrap().to_string();
                            let field_ident = &f.ident;
                            quote! {
                                sexpr::SExprTerm::List(vec![
                                    sexpr::SExprTerm::Symbol(#field_name.to_string()),
                                    sexpr::SExpr::to_sexpr(&self.#field_ident),
                                ])
                            }
                        })
                        .collect();

                    let expanded = quote! {
                        impl sexpr::SExpr for #name {
                            fn to_sexpr(&self) -> sexpr::SExprTerm {
                                sexpr::SExprTerm::List(vec![
                                    sexpr::SExprTerm::Symbol(stringify!(#name).to_string()),
                                    #(#field_exprs),*
                                ])
                            }
                        }
                    };

                    TokenStream::from(expanded)
                }
                // For tuple structs
                syn::Fields::Unnamed(ref fields_unnamed) => {
                    let field_indices = 0..fields_unnamed.unnamed.len();
                    let field_exprs: Vec<_> = field_indices
                        .map(|i| {
                            let index = syn::Index::from(i);
                            quote! {
                                sexpr::SExpr::to_sexpr(&self.#index)
                            }
                        })
                        .collect();

                    let expanded = quote! {
                        impl sexpr::SExpr for #name {
                            fn to_sexpr(&self) -> sexpr::SExprTerm {
                                sexpr::SExprTerm::List(vec![
                                    sexpr::SExprTerm::Symbol(stringify!(#name).to_string()),
                                    #(#field_exprs),*
                                ])
                            }
                        }
                    };

                    TokenStream::from(expanded)
                }
                // For unit structs
                syn::Fields::Unit => {
                    let expanded = quote! {
                        impl sexpr::SExpr for #name {
                            fn to_sexpr(&self) -> sexpr::SExprTerm {
                                sexpr::SExprTerm::Symbol(stringify!(#name).to_string())
                            }
                        }
                    };

                    TokenStream::from(expanded)
                }
            }
        }
        syn::Data::Enum(ref data_enum) => {
            let variants = data_enum.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let fields = match &variant.fields {
                    syn::Fields::Named(fields_named) => {
                        let field_names: Vec<_> = fields_named
                            .named
                            .iter()
                            .map(|f| f.ident.as_ref().unwrap())
                            .collect();
                        quote! {
                            #name::#variant_name { #(#field_names),* } => {
                                let mut list = vec![sexpr::symbol(stringify!(#variant_name))];
                                #(list.push(sexpr::SExpr::to_sexpr(&#field_names));)*
                                sexpr::SExprTerm::List(list)
                            }
                        }
                    }
                    syn::Fields::Unnamed(fields_unnamed) => {
                        let field_indices: Vec<syn::Ident> = (0..fields_unnamed.unnamed.len())
                            .map(|i| syn::Ident::new(&format!("f{}", i), Span::call_site()))
                            .collect();
                        quote! {
                            #name::#variant_name(#(#field_indices),*) => {
                                let mut list = vec![sexpr::symbol(stringify!(#variant_name))];
                                #(list.push(sexpr::SExpr::to_sexpr(&#field_indices));)*
                                sexpr::SExprTerm::List(list)
                            }
                        }
                    }
                    syn::Fields::Unit => {
                        quote! {
                            #name::#variant_name => sexpr::symbol(stringify!(#variant_name)),
                        }
                    }
                };
                fields
            });

            let expanded = quote! {
                impl sexpr::SExpr for #name {
                    fn to_sexpr(&self) -> sexpr::SExprTerm {
                        match self {
                            #(#variants),*
                        }
                    }
                }
            };

            TokenStream::from(expanded)
        }
        _ => TokenStream::from(
            syn::Error::new(
                ast.ident.span(),
                "SExpr can only be derived for structs & enums",
            )
            .to_compile_error(),
        ),
    }
}
