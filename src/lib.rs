use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

mod derive_input;
use derive_input::DataRecordOpts;

/// A procedural macro for generating:
/// - A trait with the specified name (required).
/// - A constructor with the specified name (optional, defaults to "new").
/// - One getter method per named field in the struct.
///
/// Usage:
///
///   #[derive(DataRecord)]
///   #[datarecord(name = "MyCustomTrait", constructor_name = "build")]
///   pub struct Example { ... }
///
/// * `name` is required.
/// * `constructor_name` is optional and defaults to "new" if not provided.
/// * `impl_getter` is optional and defaults to false if not provided.
/// * `impl_const` is optional and defaults to false if not provided.
///
/// Attributes can be applied to the generated trait and its methods using the following:
/// * `datarecord_getter_attr` applies to the getter trait definition.
/// * `datarecord_getter_impl_attr` applies to the getter trait implementation.
/// * `datarecord_const_attr` applies to the constructor trait definition.
/// * `datarecord_const_impl_attr` applies to the constructor trait implementation.
/// * `datarecord_const_impl_method_attr` applies to the constructor method.
///
#[proc_macro_derive(
    DataRecord,
    attributes(
        datarecord,
        datarecord_getter_attr,
        datarecord_getter_impl_attr,
        datarecord_const_attr,
        datarecord_const_impl_attr,
        datarecord_const_impl_method_attr
    )
)]
pub fn generate_data_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = DataRecordOpts::from_derive_input(&input).unwrap();

    let struct_name = &opts.ident;
    let getter_trait_ident = syn::Ident::new(&(opts.name.clone() + "Getter"), struct_name.span());
    let const_trait_ident = syn::Ident::new(&(opts.name + "Constructor"), struct_name.span());
    let constructor_ident = syn::Ident::new(&opts.constructor_name, struct_name.span());

    let getter_trait_attrs = opts.attrs.getter_attrs;
    let getter_trait_impl_attrs = opts.attrs.getter_impl_attrs;
    let const_trait_attrs = opts.attrs.const_attrs;
    let const_trait_impl_attrs = opts.attrs.const_impl_attrs;
    let const_trait_impl_method_attrs = opts.attrs.const_impl_method_attrs;

    // Ensure struct has named fields
    let data_struct = match input.data {
        Data::Struct(ds) => ds,
        _ => {
            return syn::Error::new_spanned(
                &input.ident,
                "DataClass can only be used on structs with named fields.",
            )
            .to_compile_error()
            .into();
        }
    };
    let fields = match data_struct.fields {
        Fields::Named(ref named) => &named.named,
        _ => {
            return syn::Error::new_spanned(
                &input.ident,
                "DataRecord requires named fields (no tuple or unit structs).",
            )
            .to_compile_error()
            .into();
        }
    };

    let trait_methods = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;
        quote! {
            fn #field_name(&self) -> #field_type;
        }
    });

    let constructor_params = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_type = &f.ty;
        quote! { #field_name: #field_type }
    });

    let getter_impl = match opts.impl_getter {
        true => {
            // Generate the trait impl (return clone of each field)
            let trait_impls = fields.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                let field_type = &f.ty;
                quote! {
                    fn #field_name(&self) -> #field_type {
                        self.#field_name.clone()
                    }
                }
            });

            quote! {
            #(#getter_trait_impl_attrs)*
            impl #getter_trait_ident for #struct_name {
                #(#trait_impls)*
            }}
        }
        false => quote! {},
    };

    let const_impl = match opts.impl_const {
        true => {
            let constructor_params = constructor_params.clone();
            let constructor_body = fields.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                quote! { #field_name }
            });
            quote! {
                #(#const_trait_impl_attrs)*
                impl #const_trait_ident for #struct_name {
                    #(#const_trait_impl_method_attrs)*
                    fn #constructor_ident(#(#constructor_params),*) -> Self {
                        Self { #(#constructor_body),* }
                    }
                }
            }
        }
        false => quote! {},
    };

    // Put it all together
    let expanded = quote! {
        #(#getter_trait_attrs)*
        pub trait #getter_trait_ident: Send + Sync {
            #(#trait_methods)*
        }
        #(#const_trait_attrs)*
        pub trait #const_trait_ident: Send + Sync {
            fn #constructor_ident(#(#constructor_params),*) -> Self;
        }
        #getter_impl
        #const_impl

    };

    expanded.into()
}
