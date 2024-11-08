use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};



#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident.clone();

    let serialize_impl = match input.data {
        Data::Struct(ref data_struct) => {
            let mut serialize_fields = quote! {};
            #[allow(unused_variables)]
            let mut total_size = 0usize;

            match data_struct.fields {
                Fields::Named(ref fields_named) => {
                    for field in fields_named.named.iter() {
                        let field_name = &field.ident;
                        #[allow(unused)]
                        let field_name_str = field_name.as_ref().unwrap().to_string();
                        serialize_fields.extend(quote! {
                            {
                                let size = self.#field_name.serialize(&mut buf[offset..])?;
                                offset += size;
                            }
                        });
                        total_size += 0;
                    }
                }
                Fields::Unnamed(ref fields_unnamed) => {
                    for (index, _field) in fields_unnamed.unnamed.iter().enumerate() {
                        let index = syn::Index::from(index);
                        serialize_fields.extend(quote! {
                            {
                                let size = self.#index.serialize(&mut buf[offset..])?;
                                offset += size;
                            }
                        });
                        total_size += 0;
                    }
                }
                Fields::Unit => {}
            }

            quote! {
                impl embedded_serialize::Serialize for #name {
                    fn serialize(&self, buf: &mut [u8]) -> Result<usize, embedded_serialize::SerializeError> {
                        let mut offset = 0;
                        #serialize_fields
                        Ok(offset)
                    }
                }
            }
        }
        _ => {
            return syn::Error::new_spanned(
                input.ident,
                "Serialize can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };

    serialize_impl.into()
}


#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident.clone();

    let deserialize_impl = match input.data {
        Data::Struct(ref data_struct) => {
            let mut deserialize_fields = quote! {};
            let mut field_initializations = quote! {};
            #[allow(unused)]
            let mut total_size = 0usize;

            match data_struct.fields {
                Fields::Named(ref fields_named) => {
                    for field in fields_named.named.iter() {
                        let field_name = &field.ident;
                        let field_type = &field.ty;
                        deserialize_fields.extend(quote! {
                            let #field_name = embedded_serialize::Deserialize::deserialize(&buf[offset..])?;
                            offset += embedded_serialize::core::mem::size_of::<#field_type>();
                        });
                        field_initializations.extend(quote! {
                            #field_name,
                        });
                        total_size += 0;
                    }
                }
                Fields::Unnamed(ref fields_unnamed) => {
                    let mut field_names = Vec::new();
                    for (index, field) in fields_unnamed.unnamed.iter().enumerate() {
                        let field_name = syn::Ident::new(&format!("field_{}", index), field.span());
                        let field_type = &field.ty;
                        deserialize_fields.extend(quote! {
                            let #field_name = embedded_serialize::Deserialize::deserialize(&buf[offset..])?;
                            offset += embedded_serialize::core::mem::size_of::<#field_type>();
                        });
                        field_names.push(field_name);
                        total_size += 0; 
                    }

                    field_initializations = quote! {
                        (#(#field_names),*)
                    };
                }
                Fields::Unit => {}
            }

            quote! {
                impl embedded_serialize::Deserialize for #name {
                    fn deserialize(buf: &[u8]) -> Result<Self, embedded_serialize::DeserializeError> {
                        let mut offset = 0;
                        #deserialize_fields
                        Ok(Self {
                            #field_initializations
                        })
                    }
                }
            }
        }
        _ => {
           
            return syn::Error::new_spanned(
                input.ident,
                "Deserialize can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };

    deserialize_impl.into()
}
