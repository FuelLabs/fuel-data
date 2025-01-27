use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

// ======================== #[derive(Subject)] ========================
#[proc_macro_derive(Subject)]
pub fn derive_subject(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();
    let struct_name_str = struct_name.to_string();

    if !struct_name_str.ends_with("Subject") {
        return syn::Error::new_spanned(&struct_name, "Subject structs must end with 'Subject'")
            .to_compile_error()
            .into();
    }

    let base_name = struct_name_str.trim_end_matches("Subject");
    let (_data_type, data_type_proto) = generate_types(base_name, &struct_name);
    let subject_prefix = base_name.to_lowercase();

    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => fields
            .named
            .iter()
            .map(|f| f.ident.as_ref().unwrap())
            .collect::<Vec<_>>(),
        _ => panic!("Subject requires named fields"),
    };

    let format_fields = fields.iter().map(|f| quote! { &self.#f });
    let format_parts = vec![".{}"; fields.len()];

    quote! {
        impl Subject for #struct_name {
            type DataTypeProto = #data_type_proto;

            fn to_nats_subject(&self) -> String {
                format!(
                    concat!(#subject_prefix, #(#format_parts),*),
                    #(#format_fields),*
                )
            }
        }
    }
    .into()
}

// ======================== #[derive(SubjectFilter)] ========================
#[proc_macro_derive(SubjectFilter)]
pub fn derive_subject_filter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();
    let struct_name_str = struct_name.to_string();

    if !struct_name_str.ends_with("SubjectFilter") {
        return syn::Error::new_spanned(
            &struct_name,
            "SubjectFilter structs must end with 'SubjectFilter'",
        )
        .to_compile_error()
        .into();
    }

    let base_name = struct_name_str.trim_end_matches("SubjectFilter");
    let (data_type, data_type_proto) = generate_types(base_name, &struct_name);
    let subject_struct = Ident::new(&format!("{}Subject", base_name), struct_name.span());
    let subject_prefix = base_name.to_lowercase();

    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => fields
            .named
            .iter()
            .map(|f| f.ident.as_ref().unwrap())
            .collect::<Vec<_>>(),
        _ => panic!("SubjectFilter requires named fields"),
    };

    let field_conversions = fields.iter().map(|field| {
        quote! {
            let #field = self.#field
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "*".to_string());
        }
    });

    let format_parts = vec![".{}"; fields.len()];

    quote! {
        impl SubjectFilter for #struct_name {
            type Subject = #subject_struct;
            type DataType = #data_type;
            type DataTypeProto = #data_type_proto;

            fn to_nats_subject_filter(&self) -> String {
                #(#field_conversions)*
                format!(
                    concat!(#subject_prefix, #(#format_parts),*),
                    #(#fields),*
                )
            }
        }
    }
    .into()
}

// ======================== HELPER FUNCTIONS ========================
fn generate_types(base_name: &str, span: &Ident) -> (Ident, Ident) {
    let singular = base_name.trim_end_matches('s');
    (
        Ident::new(singular, span.span()),
        Ident::new(&format!("{}Proto", singular), span.span()),
    )
}
