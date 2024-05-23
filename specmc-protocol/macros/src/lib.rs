use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, IdentFragment};
use syn::{
    self, braced, parenthesized,
    parse::{Parse, ParseBuffer, ParseStream},
    Ident, LitInt, Token, parse_macro_input,
};

#[derive(Debug)]
enum IntegerType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    VarInt,
    VarLong,
}
impl IntegerType {
    pub fn new(input: &str, span: Span) -> syn::Result<Self> {
        use IntegerType::*;
        match input {
            "u8" => Ok(U8),
            "u16" => Ok(U16),
            "u32" => Ok(U32),
            "u64" => Ok(U64),
            "i8" => Ok(I8),
            "i16" => Ok(I16),
            "i32" => Ok(I32),
            "i64" => Ok(I64),
            "VarInt" => Ok(VarInt),
            "VarLong" => Ok(VarLong),
            _ => Err(syn::Error::new(span, "Invalid type")),
        }
    }
}
impl Parse for IntegerType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse::<Ident>()?;
        IntegerType::new(&ident.to_string(), ident.span())
    }
}

#[derive(Debug)]
enum DefaultType {
    Bool,
    F32,
    F64,
    Nbt,
    // TODO String,
    // TODO List(Box<DefaultType>, )
    Integer(IntegerType),
}
impl Parse for DefaultType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use DefaultType::*;
        let ident: Ident = input.parse::<Ident>()?;
        match ident.to_string().as_str() {
            "bool" => Ok(Bool),
            "f32" => Ok(F32),
            "f64" => Ok(F64),
            "Nbt" => Ok(Nbt),
            _ => Ok(Integer(IntegerType::new(&ident.to_string(), ident.span())?)),
        }
    }
}

struct EnumField {
    name: Ident,
    value: Option<LitInt>,
}
impl Parse for EnumField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let mut value: Option<LitInt> = None;
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            value = Some(input.parse()?);
        }
        Ok(EnumField { name, value })
    }
}

struct Enum {
    name: Ident,
    ty: IntegerType,
    fields: Vec<EnumField>,
}
impl Parse for Enum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![enum]>()?;

        let name: Ident = input.parse()?;

        let content;
        parenthesized!(content in input);
        let ty: IntegerType = content.parse()?;

        let content;
        braced!(content in input);
        let mut fields: Vec<EnumField> = vec![];
        while !content.is_empty() {
            fields.push(content.parse()?);
        }

        Ok(Enum { name, ty, fields })
    }
}

#[proc_macro]
pub fn parse_enum(input: TokenStream) -> TokenStream {
    let input: Enum = parse_macro_input!(input as Enum);

    println!("{:?}", input.name.to_string());
    println!("{:?}", input.ty);
    for field in input.fields {
        println!("{:?}", field.name.to_string());
    }

    quote! {}.into()
}