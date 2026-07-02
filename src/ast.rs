use syn::File;
use quote::quote;
use anyhow::Result;

pub fn compress_rust_code(raw_code: &str) -> Result<String> {
    // Parse the raw text into a structural AST
    let ast: File = syn::parse_file(raw_code)?;
    
    // Convert the AST back to a TokenStream and into a string.
    // This inherently strips all standard comments and normalizes whitespace.
    let minified = quote!(#ast).to_string();
    
    Ok(minified)
}