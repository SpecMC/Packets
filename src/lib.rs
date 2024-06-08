use std::ops::RangeInclusive;

use specmc_base::{
    ensure, ensure_tokens,
    parse::{Identifier, Literal, Parse, ParseError},
};
use strtoint::strtoint;

#[derive(Debug)]
pub enum IntegerType {
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
    pub fn range(&self) -> RangeInclusive<isize> {
        macro_rules! range {
            ($ty:ty) => {
                <$ty>::MIN as isize..=<$ty>::MAX as isize
            };
        }

        match self {
            Self::U8 => range!(u8),
            Self::U16 => range!(u16),
            Self::U32 => range!(u32),
            Self::U64 => range!(u64),
            Self::I8 => range!(i8),
            Self::I16 => range!(i16),
            Self::I32 | Self::VarInt => range!(i32),
            Self::I64 | Self::VarLong => range!(i64),
        }
    }

    pub fn check(&self, value: isize) -> bool {
        self.range().contains(&value)
    }
}
impl Parse for IntegerType {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure!(!tokens.is_empty(), ParseError::EndOfFile);

        use IntegerType::*;
        match tokens.pop().unwrap().as_str() {
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
            token => Err(ParseError::InvalidToken {
                token: token.to_string(),
                error: "Invalid integer type".to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub enum BaseType {
    Integer(IntegerType),
    F32,
    F64,
    String(usize),
    // TODO List(usize),
    Nbt,
}
impl Parse for BaseType {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure!(!tokens.is_empty(), ParseError::EndOfFile);
        use BaseType::*;
        match tokens.pop().unwrap().as_str() {
            "f32" => Ok(F32),
            "f64" => Ok(F64),
            "String" => {
                let mut n: usize = 32767;

                if !tokens.is_empty() && tokens.last().unwrap() == "[" {
                    ensure!(tokens.len() >= 3, ParseError::EndOfFile);

                    tokens.pop();
                    n = strtoint(&tokens.pop().unwrap()).map_err(|_| ParseError::EndOfFile)?;
                    tokens.pop();
                }

                Ok(String(n))
            }
            "List" => todo!(),
            "Nbt" => Ok(Nbt),
            token => Ok(Integer(
                IntegerType::parse(&mut vec![token.to_string()]).map_err(|_| {
                    ParseError::InvalidToken {
                        token: token.to_string(),
                        error: "Invalid type".to_string(),
                    }
                })?,
            )),
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Serverbound,
    Clientbound,
}
impl Parse for Direction {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure!(!tokens.is_empty(), ParseError::EndOfFile);

        use Direction::*;
        match tokens.pop().unwrap().as_str() {
            "serverbound" => Ok(Serverbound),
            "clientbound" => Ok(Clientbound),
            token => Err(ParseError::InvalidToken {
                token: token.to_string(),
                error: "Invalid direction".to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Identifier(Identifier),
    Literal(Literal),
}
impl Parse for Value {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure!(!tokens.is_empty(), ParseError::EndOfFile);
        if let Ok(literal) = Literal::parse(tokens) {
            Ok(Value::Literal(literal))
        } else {
            Ok(Value::Identifier(Identifier::parse(tokens)?))
        }
    }
}

#[derive(Debug)]
pub enum Type {
    BaseType(BaseType),
    CustomType(Identifier),
}
impl Parse for Type {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        if let Ok(base_type) = BaseType::parse(tokens) {
            Ok(Type::BaseType(base_type))
        } else {
            Ok(Type::CustomType(Identifier::parse(tokens)?))
        }
    }
}

#[derive(Debug)]
pub struct Field {
    name: Identifier,
    ty: Type,
    value: Option<Value>,
    condition: Option<String>,
}
impl Parse for Field {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        let ty: Type = Type::parse(tokens)?;
        let name: Identifier = Identifier::parse(tokens)?;
        let mut value: Option<Value> = None;

        if !tokens.is_empty() && tokens.last().unwrap() == "if" {
            tokens.pop();
            value = Some(Value::parse(tokens)?);
        }

        Ok(Field {
            name,
            ty,
            value,
            condition: None,
        })
    }
}

#[derive(Debug)]
pub struct FieldList {
    value: Vec<Field>,
}
impl Parse for FieldList {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        let mut value: Vec<Field> = vec![];
        let mut conditions: Vec<String> = vec![];
        let mut bracket_count: usize = 0;
        while !tokens.is_empty() {
            match tokens.last().unwrap().as_str() {
                "}" => {
                    if bracket_count == 0 {
                        break;
                    }
                    tokens.pop();
                    bracket_count -= 1;
                    conditions.pop();
                }
                "if" => {
                    tokens.pop();
                    ensure_tokens!(tokens, "(");

                    let mut condition: String = "(".to_string();
                    let mut paren_count: usize = 1;
                    while !tokens.is_empty() && paren_count != 0 {
                        if tokens.last().unwrap() == "(" {
                            paren_count += 1;
                        } else if tokens.last().unwrap() == ")" {
                            paren_count -= 1;
                        }
                        condition += " ";
                        condition += &tokens.pop().unwrap();
                    }
                    conditions.push(condition);

                    ensure_tokens!(tokens, "{");
                    bracket_count += 1;
                }
                _ => {
                    let mut field: Field = Field::parse(tokens)?;

                    if !conditions.is_empty() {
                        field.condition = Some(conditions.join(" && "));
                    }

                    value.push(field);
                }
            }
        }

        Ok(FieldList { value })
    }
}

#[derive(Debug)]
pub struct CustomType {
    name: Identifier,
    fields: FieldList,
}
impl Parse for CustomType {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure_tokens!(tokens, "type");
        let name: Identifier = Identifier::parse(tokens)?;
        ensure_tokens!(tokens, "{");
        let fields: FieldList = FieldList::parse(tokens)?;
        ensure_tokens!(tokens, "}");

        Ok(CustomType { name, fields })
    }
}

#[derive(Debug)]
pub struct Variant {
    name: Identifier,
    value: Option<isize>,
}
impl Parse for Variant {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        let name: Identifier = Identifier::parse(tokens)?;
        let mut value: Option<isize> = None;

        if !tokens.is_empty() && tokens.last().unwrap() == "=" {
            tokens.pop();
            ensure!(!tokens.is_empty(), ParseError::EndOfFile);
            let _value: String = tokens.pop().unwrap();
            value = Some(strtoint(&_value).map_err(|_| ParseError::InvalidToken {
                token: _value,
                error: "Invalid variant value".to_string(),
            })?);
        }

        Ok(Variant { name, value })
    }
}

#[derive(Debug)]
pub struct Enum {
    name: Identifier,
    ty: IntegerType,
    variants: Vec<Variant>,
}
impl Parse for Enum {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure_tokens!(tokens, "enum");
        let name: Identifier = Identifier::parse(tokens)?;
        ensure_tokens!(tokens, "(");
        let ty: IntegerType = IntegerType::parse(tokens)?;
        ensure_tokens!(tokens, ")", "{");

        let mut variants: Vec<Variant> = vec![];
        let mut i: isize = 0;
        while !tokens.is_empty() && tokens.last().unwrap() != "}" {
            let mut variant: Variant = Variant::parse(tokens)?;

            if let Some(value) = variant.value {
                i = value;
            } else {
                variant.value = Some(i);
            }

            ensure!(
                ty.check(i),
                ParseError::InvalidToken {
                    token: i.to_string(),
                    error: "Enum has incompatible type".to_string(),
                }
            );

            variants.push(variant);
            i += 1;
        }

        ensure!(!tokens.is_empty(), ParseError::EndOfFile);
        tokens.pop();

        Ok(Enum { name, ty, variants })
    }
}

#[derive(Debug)]
pub struct Packet {
    name: Identifier,
    direction: Direction,
    state: Identifier,
    id: u32,
    fields: FieldList,
}
impl Parse for Packet {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure_tokens!(tokens, "packet");
        let name: Identifier = Identifier::parse(tokens)?;
        ensure_tokens!(tokens, "(");
        let direction: Direction = Direction::parse(tokens)?;
        ensure_tokens!(tokens, ",");
        let state: Identifier = Identifier::parse(tokens)?;
        ensure_tokens!(tokens, ",");
        ensure!(!tokens.is_empty(), ParseError::EndOfFile);
        let id: String = tokens.pop().unwrap();
        let id: u32 = strtoint(&id).map_err(|_| ParseError::InvalidToken {
            token: id,
            error: "Invalid packet id".to_string(),
        })?;
        ensure_tokens!(tokens, ")");
        ensure_tokens!(tokens, "{");
        let fields: FieldList = FieldList::parse(tokens)?;
        ensure_tokens!(tokens, "}");

        Ok(Packet {
            name,
            direction,
            state,
            id,
            fields,
        })
    }
}
