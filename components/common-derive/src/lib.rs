use proc_macro::TokenStream;

use syn::{Attribute,
          Data,
          DataStruct,
          DeriveInput,
          Expr,
          ExprLit,
          Field,
          Fields,
          Ident,
          Lit,
          Meta,
          MetaNameValue,
          parse_macro_input};

use quote::quote;

#[proc_macro_derive(GenConfig)]
pub fn gen_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields),
                                  .. }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let field_tokens =
        fields.iter().map(|f| {
                         let field_ident = f.ident.clone().unwrap();
                         let field_doc = get_field_doc(&f.attrs);
                         let field_value = get_field_value(&field_ident);

                         if is_hidden_field(f) {
                             quote! {}
                         } else if is_flattened_field(f) {
                             quote! {
                                 config_string += self.#field_ident.gen_config().as_str();
                             }
                         } else {
                             quote! {
                                 config_string += #field_doc;
                                 #field_value
                             }
                         }
                     });

    let struct_ident = input.ident;

    let gen_config_tokens = quote! {

        #[automatically_derived]
        impl #struct_ident {
            pub fn gen_config(&self) -> String {
                let mut config_string = String::new();

                #(#field_tokens)*

                config_string
            }
        }
    };

    // TokenStream::new()
    TokenStream::from(gen_config_tokens)
}

fn get_field_value(ident: &Ident) -> proc_macro2::TokenStream {
    quote! {
            let field_value = match toml::Value::try_from(&self.#ident) {
                Ok(value) => format!("{} = {}\n\n", stringify!(#ident), value),
                Err(e) => {
                    format!("# {} = \n\n", stringify!(#ident))
                }
            };
            config_string += field_value.as_str();
    }
}

fn get_field_doc(attrs: &[Attribute]) -> String {
    let mut doc_string = String::new();
    for attr in attrs {
        let Attribute { meta, .. } = attr;
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta
           && path.segments[0].ident == "doc"
           && let Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) = value
        {
            doc_string += &format!("### {}\n", lit.value());
        }
    }
    doc_string
}

fn is_hidden_field(field: &Field) -> bool {
    for attr in &field.attrs {
        if attr.path().is_ident("arg")
           && let Ok(ts) = attr.parse_args::<proc_macro2::TokenStream>()
        {
            let tokens: Vec<_> = ts.into_iter().collect();
            for i in 0..tokens.len().saturating_sub(2) {
                if let (proc_macro2::TokenTree::Ident(id),
                        proc_macro2::TokenTree::Punct(eq),
                        proc_macro2::TokenTree::Ident(val)) =
                    (&tokens[i], &tokens[i + 1], &tokens[i + 2])
                   && id == "hide"
                   && eq.as_char() == '='
                   && val == "true"
                {
                    return true;
                }
            }
        }
    }
    false
}

fn is_flattened_field(field: &Field) -> bool {
    for attr in &field.attrs {
        if attr.path().is_ident("command")
           && let Ok(ts) = attr.parse_args::<proc_macro2::TokenStream>()
        {
            let tokens: Vec<_> = ts.into_iter().collect();
            for i in 0..tokens.len() {
                // Only match `flatten` as a bare path, not as a value (e.g. `x = flatten`)
                let preceded_by_eq = i > 0
                                     && matches!(&tokens[i - 1],
                                proc_macro2::TokenTree::Punct(p) if p.as_char() == '=');
                if let proc_macro2::TokenTree::Ident(id) = &tokens[i]
                   && id == "flatten"
                   && !preceded_by_eq
                {
                    return true;
                }
            }
        }
    }
    false
}
