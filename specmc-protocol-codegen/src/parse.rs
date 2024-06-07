use std::str::Chars;

use thiserror::Error;

macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
}

macro_rules! ensure_tokens {
    ($tokens:ident, $($token:expr),+) => {
        $(
            ensure!(
                $tokens.last().unwrap() == $token,
                ParseError::InvalidToken {
                    token: $tokens.last().unwrap().clone(),
                    error: format!("Expected {}", $token),
                }
            );
            $tokens.pop();
        )+
    };
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected EOF")]
    EndOfFile,

    #[error("Invalid token: {token}")]
    InvalidToken { token: String, error: String },
}

pub trait Parse
where
    Self: Sized,
{
    /// Parse a list of tokens into an object, consuming the tokens as needed.
    /// The token list is consumed in reverse order.
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError>;
}

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
pub enum DefaultType {
    Integer(IntegerType),
    F32,
    F64,
    String(usize),
    // TODO List(usize),
    Nbt,
}
impl Parse for DefaultType {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure!(!tokens.is_empty(), ParseError::EndOfFile);

        use DefaultType::*;
        match tokens.pop().unwrap().as_str() {
            "f32" => Ok(F32),
            "f64" => Ok(F64),
            "String" => {
                let mut n: usize = 32767;

                if !tokens.is_empty() && tokens.last().unwrap() == "[" {
                    ensure!(tokens.len() >= 3, ParseError::EndOfFile);

                    tokens.pop();
                    n = tokens
                        .pop()
                        .unwrap()
                        .parse()
                        .map_err(|_| ParseError::EndOfFile)?;
                    tokens.pop();
                }

                Ok(String(n))
            }
            // TODO "List" => Ok(List),
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
pub struct Identifier {
    name: String,
}
impl Parse for Identifier {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure!(!tokens.is_empty(), ParseError::EndOfFile);

        let name: String = tokens.pop().unwrap();

        ensure!(
            !name.is_empty(),
            ParseError::InvalidToken {
                token: name,
                error: "Empty identifier".to_string()
            }
        );

        let mut chars: Chars = name.chars();
        ensure!(
            chars.next().unwrap().is_ascii_alphabetic(),
            ParseError::InvalidToken {
                token: name,
                error: "Identifiers must start with a letter".to_string()
            }
        );
        ensure!(
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_'),
            ParseError::InvalidToken {
                token: name,
                error: "Identifiers can only contain letters, numbers, and underscores".to_string()
            }
        );

        Ok(Identifier { name })
    }
}

#[derive(Debug)]
pub struct Enum {
    name: Identifier,
    ty: IntegerType,
    variants: Vec<(String, isize)>,
}
impl Parse for Enum {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure!(tokens.len() >= 6, ParseError::EndOfFile);

        ensure_tokens!(tokens, "enum");
        let name: Identifier = Identifier::parse(tokens)?;
        ensure_tokens!(tokens, "(");
        let ty: IntegerType = IntegerType::parse(tokens)?;
        ensure_tokens!(tokens, ")", "{");

        let mut variants: Vec<(String, isize)> = Vec::new();
        let mut i: isize = 0;
        while !tokens.is_empty() && tokens.last().unwrap() != "}" {
            ensure!(!tokens.is_empty(), ParseError::EndOfFile);
            let name: String = tokens.pop().unwrap();

            if let Some("=") = tokens.last().map(String::as_str) {
                ensure!(tokens.len() >= 2, ParseError::EndOfFile);
                tokens.pop();
                let value: String = tokens.pop().unwrap();
                i = value.parse().map_err(|_| ParseError::InvalidToken {
                    token: value,
                    error: "Invalid value".to_string(),
                })?;
            }

            variants.push((name, i));
            i += 1;
        }

        ensure!(!tokens.is_empty(), ParseError::EndOfFile);
        tokens.pop();

        Ok(Enum { name, ty, variants })
    }
}

#[derive(Debug)]
pub struct Field {
    name: Identifier,
    ty: DefaultType,
    condition: Option<String>,
}
impl Parse for Field {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure!(tokens.len() >= 2, ParseError::EndOfFile);

        let ty: DefaultType = DefaultType::parse(tokens)?;
        let name: Identifier = Identifier::parse(tokens)?;

        Ok(Field {
            name,
            ty,
            condition: None,
        })
    }
}

// TODO
#[derive(Debug)]
pub struct CustomType {
    name: Identifier,
    fields: Vec<Field>,
}
impl Parse for CustomType {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        ensure!(tokens.len() >= 3, ParseError::EndOfFile);

        ensure_tokens!(tokens, "type");
        let name: Identifier = Identifier::parse(tokens)?;
        ensure_tokens!(tokens, "{");

        let mut fields: Vec<Field> = Vec::new();
        while !tokens.is_empty() && tokens.last().unwrap() != "}" {
            if tokens.last().unwrap() == "if" {
                // TODO
            }
            fields.push(Field::parse(tokens)?);
        }
        ensure_tokens!(tokens, "}");

        Ok(CustomType { name, fields })
    }
}
