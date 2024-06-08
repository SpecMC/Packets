use std::ops::RangeInclusive;

use specmc_base::{
    ensure, ensure_tokens,
    parse::{Identifier, Literal, Parse, ParseError},
};
use strtoint::strtoint;

use crate::types::Type;

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

        use IntegerType::*;
        match self {
            U8 => range!(u8),
            U16 => range!(u16),
            U32 => range!(u32),
            U64 => range!(u64),
            I8 => range!(i8),
            I16 => range!(i16),
            I32 | VarInt => range!(i32),
            I64 | VarLong => range!(i64),
        }
    }

    pub fn check(&self, value: isize) -> bool {
        self.range().contains(&value)
    }
}
impl Parse for IntegerType {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        use IntegerType::*;
        match tokens.pop().ok_or(ParseError::EndOfFile)?.as_str() {
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
            token => {
                tokens.push(token.to_string());
                Err(ParseError::InvalidToken {
                    token: token.to_string(),
                    error: "Invalid integer type".to_string(),
                })
            }
        }
    }
}

#[derive(Debug)]
pub enum BaseType {
    Integer(IntegerType),
    F32,
    F64,
    String { length: usize },
    List { ty: Box<Type>, length: Value },
    Nbt,
}
impl Parse for BaseType {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        use BaseType::*;
        match tokens.pop().ok_or(ParseError::EndOfFile)?.as_str() {
            "f32" => Ok(F32),
            "f64" => Ok(F64),
            "String" => {
                let mut length: usize = 32767;

                if !tokens.is_empty() && tokens.last().unwrap() == "[" {
                    tokens.pop();
                    length = strtoint(&tokens.pop().ok_or(ParseError::EndOfFile)?)
                        .map_err(|_| ParseError::EndOfFile)?;
                    ensure_tokens!(tokens, "]");
                }

                Ok(String { length })
            }
            "List" => {
                ensure_tokens!(tokens, "[");
                let ty: Box<Type> = Box::new(Type::parse(tokens)?);
                ensure_tokens!(tokens, ";");
                let length: Value = Value::parse(tokens)?;
                if matches!(length, Value::Literal(_)) {
                    ensure!(
                        matches!(length, Value::Literal(Literal::Integer(_))),
                        ParseError::InvalidToken {
                            token: format!("{length:?}"),
                            error: "List length must be an integer".to_string()
                        }
                    );
                }
                ensure_tokens!(tokens, "]");
                Ok(List { ty, length })
            }
            "Nbt" => Ok(Nbt),
            token => {
                tokens.push(token.to_string());
                Ok(Integer(IntegerType::parse(tokens).map_err(|_| {
                    ParseError::InvalidToken {
                        token: token.to_string(),
                        error: "Invalid type".to_string(),
                    }
                })?))
            }
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
        if let Ok(literal) = Literal::parse(tokens) {
            Ok(Value::Literal(literal))
        } else {
            Ok(Value::Identifier(Identifier::parse(tokens)?))
        }
    }
}

#[derive(Debug)]
pub struct Field {
    pub name: Identifier,
    pub ty: Type,
    pub value: Option<Value>,
    pub condition: Option<String>,
}
impl Parse for Field {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        let ty: Type = Type::parse(tokens)?;
        let name: Identifier = Identifier::parse(tokens)?;
        let mut value: Option<Value> = None;

        if !tokens.is_empty() && tokens.last().unwrap() == "=" {
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
    pub inner: Vec<Field>,
}
impl Parse for FieldList {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        let mut value: Vec<Field> = vec![];
        let mut conditions: Vec<String> = vec![];
        let mut bracket_count: usize = 0;
        while !tokens.is_empty() {
            match tokens.pop().unwrap().as_str() {
                "}" => {
                    if bracket_count == 0 {
                        tokens.push("}".to_string());
                        break;
                    }
                    bracket_count -= 1;
                    conditions.pop();
                }
                "if" => {
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
                token => {
                    tokens.push(token.to_string());
                    let mut field: Field = Field::parse(tokens)?;

                    if !conditions.is_empty() {
                        field.condition = Some(conditions.join(" && "));
                    }

                    value.push(field);
                }
            }
        }

        Ok(FieldList { inner: value })
    }
}
