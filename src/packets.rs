use specmc_base::{
    ensure_tokens,
    parse::{Identifier, Parse, ParseError},
};
use strtoint::strtoint;

use crate::base::FieldList;

#[derive(Debug)]
pub enum Direction {
    Serverbound,
    Clientbound,
}
impl Parse for Direction {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, ParseError> {
        use Direction::*;
        match tokens.pop().ok_or(ParseError::EndOfFile)?.as_str() {
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
pub struct Packet {
    pub name: Identifier,
    pub direction: Direction,
    pub state: Identifier,
    pub id: u32,
    pub fields: FieldList,
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
        let id: String = tokens.pop().ok_or(ParseError::EndOfFile)?;
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
