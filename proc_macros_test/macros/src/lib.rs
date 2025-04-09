use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[proc_macro]
pub fn do_nothing(_input: TokenStream) -> TokenStream {
    quote!(()).into()
}

#[proc_macro_derive(ToJS)]
pub fn derive_to_js(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &input.data {
    //    variants
    // } else {
    //    panic!("ToJS solo funciona con enums");
    // };

    // Generar el código JavaScript
    let js_code = match &input.data {
        syn::Data::Struct(data) => {
            let fields = match &data.fields {
                syn::Fields::Named(fields) => &fields.named,
                _ => return syn::Error::new_spanned(name, "Whatever").to_compile_error().into()
            };
            format!(r#"
        export class {} = {{
            {}
        }}
            "#, 
                name, 
                fields
                    .iter()
                    .map(|field| {
                        let field_name = field.ident.as_ref().unwrap().to_string();
                        format!(r#"
                        #{}

                        get {}() {{
                            return this.#{};
                        }}

                        set {}(value) {{
                            this.#{} = value;
                        }}"#, field_name, field_name, field_name, field_name, field_name)
                    })
                    .collect::<Vec<_>>()
                    .join("\n    "))
        },
        syn::Data::Enum(data) => {
            let variants = &data.variants;
            let variant_names: Vec<_> = variants
                .iter()
                .map(|v| &v.ident)
                .collect();

            format!(r#"
        // Código JavaScript generado automáticamente
        export const {} = Object.freeze({{
            {}
        }});
        "#,
                name,
                variant_names
                    .iter()
                    .map(|v| format!("{}: '{}'", v, v))
                    .collect::<Vec<_>>()
                    .join(",\n    "),
            )
        },
        _ => return syn::Error::new_spanned(name, "ToJS solo funciona con enums o structs").to_compile_error().into()
    };

    // Escribir a un archivo en el directorio de salida
    let out_dir = std::env::var("OUT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(".")
        });

    // Crear el directorio si no existe
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        return syn::Error::new_spanned(
            name,
            format!("No se pudo crear el directorio de salida: {}", e)
        ).to_compile_error().into();
    }

    let js_path = out_dir.join(format!("{}_generated.js", name));
    
    match File::create(&js_path)
        .and_then(|mut file| write!(file, "{}", js_code))
    {
        Ok(_) => TokenStream::new(),
        Err(e) => syn::Error::new_spanned(
            name,
            format!("No se pudo escribir el archivo JS: {}", e)
        ).to_compile_error().into(),
    }

    // let mut file = File::create(&js_path).unwrap();
    // write!(file, "{}", js_code).unwrap();

    // No necesitamos generar código Rust adicional
    // TokenStream::new()
}
