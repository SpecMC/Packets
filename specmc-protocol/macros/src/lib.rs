use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, TokenStreamExt};
use syn::{
    self, braced, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitInt, Token,
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
    // TODO List(Box<DefaultType>, ),
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

struct ProtocolDef {
    enums: Vec<Enum>,
    // TODO types, packets
}
impl Parse for ProtocolDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut enums: Vec<Enum> = vec![];
        while !input.is_empty() {
            if input.peek(Token![enum]) {
                enums.push(input.parse()?);
            } else {
                return Err(syn::Error::new(input.span(), "Invalid token"));
            }
        }
        Ok(ProtocolDef { enums })
    }
}

#[proc_macro]
pub fn protocol_def(input: TokenStream) -> TokenStream {
    let input: ProtocolDef = parse_macro_input!(input as ProtocolDef);
    let mut enum_names = vec![];
    let mut enum_types = vec![];
    let mut enum_fields = vec![];
    let mut enum_field_values = vec![];

    for e in input.enums {
        enum_names.push(e.name);
        enum_types.push(e.ty);
        let mut i: isize = 0;
        let mut fields = vec![];
        let mut field_values = vec![];
        for field in e.fields {
            fields.push(field.name);
            if let Some(value) = &field.value {
                i = value.base10_parse().unwrap();
            }
            field_values.push(i);
            i += 1;
        }
        enum_fields.push(fields);
        enum_field_values.push(field_values);
    }

    quote! {
        #(
            enum #enum_names {
                #(
                    #enum_fields = #enum_field_values
                ),*
            }
        )*
    }
    .into()
}
