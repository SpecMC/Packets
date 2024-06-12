use specmc_base::{
    ensure, ensure_tokens,
    parse::{Identifier, Literal, Parse, ParseError},
};

use crate::base::IntegerType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub name: Identifier,
    pub value: Option<isize>,
}
impl Parse for Variant {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        let name: Identifier = Identifier::parse(tokens)?;
        let mut value: Option<isize> = None;

        if !tokens.is_empty() && tokens.last().unwrap() == "=" {
            tokens.pop();
            let _value: Literal = Literal::parse(tokens)?;
            let Literal::Integer(_value) = _value else {
                return Err(ParseError::InvalidToken {
                    token: format!("{_value:?}"),
                    error: "Invalid variant value".to_string(),
                });
            };
            value = Some(_value);
        }

        Ok(Variant { name, value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Enum {
    pub name: Identifier,
    pub ty: IntegerType,
    pub variants: Vec<Variant>,
}
impl Parse for Enum {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure_tokens!(tokens, "enum");
        let name: Identifier = Identifier::parse(tokens)?;
        ensure_tokens!(tokens, "(");
        let ty: IntegerType = IntegerType::parse(tokens)?;
        ensure_tokens!(tokens, ")", "{");

        let mut variants: Vec<Variant> = vec![];
        let mut values: Vec<isize> = vec![];
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

            ensure!(
                !values.contains(&i),
                ParseError::InvalidToken {
                    token: i.to_string(),
                    error: "Enum has duplicate value".to_string(),
                }
            );

            variants.push(variant);
            values.push(i);
            i += 1;
        }

        ensure_tokens!(tokens, "}");

        Ok(Enum { name, ty, variants })
    }
}

#[cfg(test)]
mod tests {
    use specmc_base::tokenize;

    use crate::test_parse;

    use super::*;

    #[test]
    fn test_variant() {
        let mut tokens: Vec<String> = tokenize!("A = 42 B C = -123 D = 1.5");

        test_parse!(
            tokens,
            Variant,
            Ok(Variant {
                name: Identifier("A".to_string()),
                value: Some(42),
            })
        );
        test_parse!(
            tokens,
            Variant,
            Ok(Variant {
                name: Identifier("B".to_string()),
                value: None,
            })
        );
        test_parse!(
            tokens,
            Variant,
            Ok(Variant {
                name: Identifier("C".to_string()),
                value: Some(-123),
            })
        );

        test_parse!(
            tokens,
            Variant,
            Err(ParseError::InvalidToken {
                token: "Float(1.5)".to_string(),
                error: "Invalid variant value".to_string(),
            })
        );
        assert!(tokens.is_empty());
        test_parse!(tokens, Variant, Err(ParseError::EndOfFile));
    }

    #[test]
    fn test_enum() {
        let mut tokens: Vec<String> = tokenize!(
            "
            enum A(VarInt) {
                A = 42
                B
                C = -123
                D = 1
            }
            enum B(u8) {
                A = 254
                B
                C
            }
            enum C(i32) {
                A = 42
                B = 40
                C
                D
            }
            enum D(u64) {
                A = 42.0
            }
            "
        );

        test_parse!(
            tokens,
            Enum,
            Ok(Enum {
                name: Identifier("A".to_string()),
                ty: IntegerType::VarInt,
                variants: vec![
                    Variant {
                        name: Identifier("A".to_string()),
                        value: Some(42),
                    },
                    Variant {
                        name: Identifier("B".to_string()),
                        value: Some(43),
                    },
                    Variant {
                        name: Identifier("C".to_string()),
                        value: Some(-123),
                    },
                    Variant {
                        name: Identifier("D".to_string()),
                        value: Some(1),
                    },
                ]
            })
        );

        test_parse!(
            tokens,
            Enum,
            Err(ParseError::InvalidToken {
                token: "256".to_string(),
                error: "Enum has incompatible type".to_string(),
            })
        );
        assert!(tokens.pop().unwrap() == "}");
        test_parse!(
            tokens,
            Enum,
            Err(ParseError::InvalidToken {
                token: "42".to_string(),
                error: "Enum has duplicate value".to_string(),
            })
        );
        assert!(tokens.pop().unwrap() == "}");
        test_parse!(
            tokens,
            Enum,
            Err(ParseError::InvalidToken {
                token: "Float(42.0)".to_string(),
                error: "Invalid variant value".to_string(),
            })
        );
        assert!(tokens.pop().unwrap() == "}");
        assert!(tokens.is_empty());
        test_parse!(tokens, Enum, Err(ParseError::EndOfFile));
    }
}
