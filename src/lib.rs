use heck::ToUpperCamelCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident};

#[proc_macro_derive(FromEnv)]
pub fn derive_from_env(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!("Only support struct with named fields");
    };

    let loads = fields.iter().map(|field| {
        let inner = &field.ident.clone().unwrap();
        let ty = &field.ty;
        let key = Ident::new(&inner.to_string().to_uppercase(), name.span());
        let error = Ident::new(&inner.to_string().to_upper_camel_case(), name.span());
        quote! {
            #inner: dotenvy::var(stringify!(#key))
                .map(|#inner| #inner.parse::<#ty>())?
                .map_err(|_| FromEnvError::#error)?
        }
    });

    let errors = fields.iter().map(|field| {
        let inner = &field.ident.clone().unwrap();
        let name = Ident::new(&inner.to_string().to_upper_camel_case(), name.span());
        quote! {
            #name
        }
    });

    let s = quote! {
        impl #name {
            pub fn from_env() -> Result<Self, FromEnvError> {
                Ok(Self {
                    #(#loads,)*
                })
            }
        }

        pub enum FromEnvError {
            #(#errors),*,
            DotEnvy(dotenvy::Error),
        }

        impl From<dotenvy::Error> for FromEnvError {
            fn from(e: dotenvy::Error) -> Self {
                Self::DotEnvy(e)
            }
        }
    };

    s.into()
}
