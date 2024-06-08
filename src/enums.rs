use specmc_base::{
    ensure, ensure_tokens,
    parse::{Identifier, Parse, ParseError},
};
use strtoint::strtoint;

use crate::base::IntegerType;

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
            let _value: String = tokens.pop().ok_or(ParseError::EndOfFile)?;
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

        tokens.pop().ok_or(ParseError::EndOfFile)?;

        Ok(Enum { name, ty, variants })
    }
}
