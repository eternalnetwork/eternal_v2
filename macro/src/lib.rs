use darling::{FromField, FromDeriveInput};
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn;

#[derive(FromField, Default)]
#[darling(default, attributes(method), forward_attrs(allow, doc, cfg))]
struct MethodOpts {
    pub name: String,
}

#[derive(FromField, Default)]
#[darling(default, attributes(property), forward_attrs(allow, doc, cfg))]
struct PropertyOpts {
    pub name: String,
}


#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(standerd), forward_attrs(allow, doc, cfg))]
struct StanderdOpts {
    pub name: String
}


#[proc_macro_derive(SmartContract, attributes(method, standerd, property))]
pub fn derive_smart_contract(item: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(item).unwrap();
    let std = StanderdOpts::from_derive_input(&ast).unwrap();

    let name = &ast.ident;

    let mut methods: Vec<(String, String)> = vec![];
    match ast.data {
        // Only process structs
        syn::Data::Struct(ref data_struct) => {
            // Check the kind of fields the struct contains
            match data_struct.fields {
                // Structs with named fields
                syn::Fields::Named(ref fields_named) => {
                    // Iterate over the fields
                    for field in fields_named.named.iter() {
                        let opts = MethodOpts::from_field(field).unwrap().name;
                        let name = field.ident.clone().unwrap().to_string();
                        methods.push((name, opts));
                        let opts = PropertyOpts::from_field(field).unwrap();
                    }
                }

                // Struct with unnamed fields
                _ => (),
            }
        }

        // Panic when we don't have a struct
        _ => panic!("Must be a struct"),
    }

    let functions: Vec<_> = methods
        .iter()
        .map(|field| {
            let name_idnt = format_ident!("{}", field.0);
            let val_idnt = format_ident!("{}", field.1);
            quote! {
                #name_idnt: self.#val_idnt,
            }
        })
        .collect();

    let std = std.name;
    let std_name = format_ident!("{}", std);
    println!("{}", std);

    let gen = quote! {
        use eternal_vm::SmartContract;

        impl SmartContract for #name {
            fn deploy(&self) -> SC {
                SC::new(
                    SmartContractStanderd::from(#std),
                    SmartContractApi::#std_name {
                        #(#functions)*
                        publisher: String::new(),
                        total_suply: 100_000,
                    },
                )
            }
        }
    };

    gen.into()
}
