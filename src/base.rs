use std::{ops::RangeInclusive, option, string};

use specmc_base::{
    ensure_tokens,
    parse::{Identifier, Literal, Parse, ParseError},
};
use strtoint::strtoint;

use crate::types::Type;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BaseType {
    Bool,
    Integer(IntegerType),
    F32,
    F64,
    String {
        length: Option<usize>,
    },
    List {
        ty: Box<Type>,
        // length_ty: IntegerType,
        length: Option<usize>,
    },
    Nbt,
    // Option(Box<Type>),
}
impl Parse for BaseType {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        use BaseType::*;
        match tokens.pop().ok_or(ParseError::EndOfFile)?.as_str() {
            "bool" => Ok(Bool),
            "f32" => Ok(F32),
            "f64" => Ok(F64),
            "String" => {
                let mut length: option::Option<usize> = None;

                if !tokens.is_empty() && tokens.last().unwrap() == "[" {
                    tokens.pop();
                    let _length: Literal = Literal::parse(tokens)?;
                    let Literal::Integer(_length) = _length else {
                        return Err(ParseError::InvalidToken {
                            token: format!("{_length:?}"),
                            error: "Invalid list length".to_string(),
                        });
                    };
                    length = Some(_length as usize);
                    ensure_tokens!(tokens, "]");
                }

                Ok(String { length })
            }
            "List" => {
                ensure_tokens!(tokens, "[");
                let ty: Box<Type> = Box::new(Type::parse(tokens)?);
                let mut length: option::Option<usize> = None;
                if !tokens.is_empty() && tokens.last().unwrap() == ";" {
                    tokens.pop();
                    let _length: string::String = tokens.pop().ok_or(ParseError::EndOfFile)?;
                    length = Some(strtoint(&_length).map_err(|_| ParseError::InvalidToken {
                        token: _length,
                        error: "Invalid list length".to_string(),
                    })?);
                }
                ensure_tokens!(tokens, "]");
                Ok(List { ty, length })
            }
            "Nbt" => Ok(Nbt),
            // "Option" => {
            //     ensure_tokens!(tokens, "[");
            //     let ty: Box<Type> = Box::new(Type::parse(tokens)?);
            //     ensure_tokens!(tokens, "]");
            //     Ok(Option(ty))
            // }
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

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Length of a list or nbt
    Length(Identifier),
    /// A literal value
    Literal(Literal),
    /// Some identifier
    Identifier(Identifier),
}
impl Parse for Value {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        if tokens.last().ok_or(ParseError::EndOfFile)? == "len" {
            tokens.pop();
            ensure_tokens!(tokens, "(");
            let length: Identifier = Identifier::parse(tokens)?;
            ensure_tokens!(tokens, ")");
            Ok(Value::Length(length))
        } else if let Ok(literal) = Literal::parse(tokens) {
            Ok(Value::Literal(literal))
        } else {
            Ok(Value::Identifier(Identifier::parse(tokens)?))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub ty: Type,
    pub name: Identifier,
    pub value: Option<Value>,
    pub conditions: Vec<String>,
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
            conditions: vec![],
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldList(pub Vec<Field>);
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

                    let mut condition: String = "".to_string();
                    let mut paren_count: usize = 1;
                    while !tokens.is_empty() && paren_count != 0 {
                        if tokens.last().unwrap() == "(" {
                            paren_count += 1;
                        } else if tokens.last().unwrap() == ")" {
                            paren_count -= 1;
                        } else {
                            if !condition.is_empty() {
                                condition += " ";
                            }
                            condition += &tokens.last().unwrap();
                        }
                        tokens.pop();
                    }
                    conditions.push(condition);

                    ensure_tokens!(tokens, "{");
                    bracket_count += 1;
                }
                token => {
                    tokens.push(token.to_string());
                    let mut field: Field = Field::parse(tokens)?;

                    if !conditions.is_empty() {
                        field.conditions = conditions.clone();
                    }

                    value.push(field);
                }
            }
        }

        Ok(FieldList(value))
    }
}

#[cfg(test)]
mod tests {
    use specmc_base::tokenize;

    use crate::test_parse;

    use super::*;

    #[test]
    fn test_integer_type() {
        let mut tokens: Vec<String> =
            tokenize!("u8 u16 u32 u64 i8 i16 i32 i64 VarInt VarLong Unknown");

        test_parse!(tokens, IntegerType, Ok(IntegerType::U8));
        test_parse!(tokens, IntegerType, Ok(IntegerType::U16));
        test_parse!(tokens, IntegerType, Ok(IntegerType::U32));
        test_parse!(tokens, IntegerType, Ok(IntegerType::U64));
        test_parse!(tokens, IntegerType, Ok(IntegerType::I8));
        test_parse!(tokens, IntegerType, Ok(IntegerType::I16));
        test_parse!(tokens, IntegerType, Ok(IntegerType::I32));
        test_parse!(tokens, IntegerType, Ok(IntegerType::I64));
        test_parse!(tokens, IntegerType, Ok(IntegerType::VarInt));
        test_parse!(tokens, IntegerType, Ok(IntegerType::VarLong));

        test_parse!(
            tokens,
            IntegerType,
            Err(ParseError::InvalidToken {
                token: "Unknown".to_string(),
                error: "Invalid integer type".to_string(),
            })
        );
        assert_eq!(tokens.pop().unwrap(), "Unknown");
        assert!(tokens.is_empty());
        test_parse!(tokens, IntegerType, Err(ParseError::EndOfFile));
    }

    #[test]
    fn test_base_type() {
        let mut tokens: Vec<String> =
            tokenize!("bool VarInt f32 f64 String String[42] List[i32] List[u8; 42] Nbt Unknown");

        test_parse!(tokens, BaseType, Ok(BaseType::Bool));
        test_parse!(tokens, BaseType, Ok(BaseType::Integer(IntegerType::VarInt)));
        test_parse!(tokens, BaseType, Ok(BaseType::F32));
        test_parse!(tokens, BaseType, Ok(BaseType::F64));
        test_parse!(tokens, BaseType, Ok(BaseType::String { length: None }));
        test_parse!(tokens, BaseType, Ok(BaseType::String { length: Some(42) }));
        test_parse!(
            tokens,
            BaseType,
            Ok(BaseType::List {
                ty: Box::new(Type::BaseType(BaseType::Integer(IntegerType::I32))),
                length: None,
            })
        );
        test_parse!(
            tokens,
            BaseType,
            Ok(BaseType::List {
                ty: Box::new(Type::BaseType(BaseType::Integer(IntegerType::U8))),
                length: Some(42),
            })
        );
        test_parse!(tokens, BaseType, Ok(BaseType::Nbt));
        // test_parse!(
        //     tokens,
        //     BaseType,
        //     Ok(BaseType::Option(Box::new(Type::BaseType(
        //         BaseType::Integer(IntegerType::VarInt)
        //     ))))
        // );

        test_parse!(
            tokens,
            BaseType,
            Err(ParseError::InvalidToken {
                token: "Unknown".to_string(),
                error: "Invalid type".to_string(),
            })
        );
        assert_eq!(tokens.pop().unwrap(), "Unknown");
        assert!(tokens.is_empty());
        test_parse!(tokens, BaseType, Err(ParseError::EndOfFile));
    }

    #[test]
    fn test_value() {
        let mut tokens: Vec<String> = tokenize!("len(iden) 42.0 iden");

        test_parse!(
            tokens,
            Value,
            Ok(Value::Length(Identifier("iden".to_string())))
        );
        test_parse!(tokens, Value, Ok(Value::Literal(Literal::Float(42.0))));
        test_parse!(
            tokens,
            Value,
            Ok(Value::Identifier(Identifier("iden".to_string())))
        );

        assert!(tokens.is_empty());
        test_parse!(tokens, Value, Err(ParseError::EndOfFile));
    }

    #[test]
    fn test_field() {
        let mut tokens: Vec<String> = tokenize!(
            "
            i32 first_field
            Nbt second_field = 42.0
            i64 third_field = len(list)
            List[i32] list
            "
        );

        test_parse!(
            tokens,
            Field,
            Ok(Field {
                ty: Type::BaseType(BaseType::Integer(IntegerType::I32)),
                name: Identifier("first_field".to_string()),
                value: None,
                conditions: vec![],
            })
        );
        test_parse!(
            tokens,
            Field,
            Ok(Field {
                ty: Type::BaseType(BaseType::Nbt),
                name: Identifier("second_field".to_string()),
                value: Some(Value::Literal(Literal::Float(42.0))),
                conditions: vec![],
            })
        );
        test_parse!(
            tokens,
            Field,
            Ok(Field {
                ty: Type::BaseType(BaseType::Integer(IntegerType::I64)),
                name: Identifier("third_field".to_string()),
                value: Some(Value::Length(Identifier("list".to_string()))),
                conditions: vec![],
            })
        );
        test_parse!(
            tokens,
            Field,
            Ok(Field {
                ty: Type::BaseType(BaseType::List {
                    ty: Box::new(Type::BaseType(BaseType::Integer(IntegerType::I32))),
                    length: None
                }),
                name: Identifier("list".to_string()),
                value: None,
                conditions: vec![],
            })
        );

        assert!(tokens.is_empty());
        test_parse!(tokens, Field, Err(ParseError::EndOfFile));
    }

    #[test]
    fn test_field_list() {
        let mut tokens: Vec<String> = tokenize!(
            "
            bool cond
            if (cond) {
                i32 number
            }
            if (!cond) {
                u64 other
            }
            "
        );

        test_parse!(
            tokens,
            FieldList,
            Ok(FieldList(vec![
                Field {
                    ty: Type::BaseType(BaseType::Bool),
                    name: Identifier("cond".to_string()),
                    value: None,
                    conditions: vec![],
                },
                Field {
                    ty: Type::BaseType(BaseType::Integer(IntegerType::I32)),
                    name: Identifier("number".to_string()),
                    value: None,
                    conditions: vec!["cond".to_string()],
                },
                Field {
                    ty: Type::BaseType(BaseType::Integer(IntegerType::U64)),
                    name: Identifier("other".to_string()),
                    value: None,
                    conditions: vec!["!cond".to_string()],
                },
            ]))
        );

        assert!(tokens.is_empty());
        test_parse!(tokens, FieldList, Ok(FieldList(vec![])));
    }
}
