use specmc_base::{
    ensure_tokens,
    parse::{Identifier, Parse, ParseError},
};
use strtoint::strtoint;

use crate::base::FieldList;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use specmc_base::tokenize;

    use crate::{
        base::{BaseType, Field, IntegerType, Value},
        test_parse,
        types::Type,
    };

    use super::*;

    #[test]
    fn test_direction() {
        let mut tokens: Vec<String> = tokenize!("serverbound clientbound unknown");

        test_parse!(tokens, Direction, Ok(Direction::Serverbound));
        test_parse!(tokens, Direction, Ok(Direction::Clientbound));

        test_parse!(
            tokens,
            Direction,
            Err(ParseError::InvalidToken {
                token: "unknown".to_string(),
                error: "Invalid direction".to_string()
            })
        );
        assert!(tokens.is_empty());
        test_parse!(tokens, Direction, Err(ParseError::EndOfFile));
    }

    #[test]
    fn test_packet() {
        let mut tokens: Vec<String> = tokenize!(
            "
            packet TestPacket(serverbound, Play, 0x42) {
                i32 number
                String message
                bool flag
                if (flag) {
                    i32 other
                }
                VarInt length = len(data)
                List[u8] data
            }
            packet MalformedPacket(serverbound, Play, 0x42) {
                i32 number
                String message
            "
        );

        test_parse!(
            tokens,
            Packet,
            Ok(Packet {
                name: Identifier("TestPacket".to_string()),
                direction: Direction::Serverbound,
                state: Identifier("Play".to_string()),
                id: 66,
                fields: FieldList(vec![
                    Field {
                        ty: Type::BaseType(BaseType::Integer(IntegerType::I32)),
                        name: Identifier("number".to_string()),
                        value: None,
                        condition: None
                    },
                    Field {
                        ty: Type::BaseType(BaseType::String { length: None }),
                        name: Identifier("message".to_string()),
                        value: None,
                        condition: None
                    },
                    Field {
                        ty: Type::BaseType(BaseType::Bool),
                        name: Identifier("flag".to_string()),
                        value: None,
                        condition: None
                    },
                    Field {
                        ty: Type::BaseType(BaseType::Integer(IntegerType::I32)),
                        name: Identifier("other".to_string()),
                        value: None,
                        condition: Some("( flag )".to_string())
                    },
                    Field {
                        ty: Type::BaseType(BaseType::Integer(IntegerType::VarInt)),
                        name: Identifier("length".to_string()),
                        value: Some(Value::Length(Identifier("data".to_string()))),
                        condition: None
                    },
                    Field {
                        ty: Type::BaseType(BaseType::List {
                            ty: Box::new(Type::BaseType(BaseType::Integer(IntegerType::U8))),
                            length: None
                        }),
                        name: Identifier("data".to_string()),
                        value: None,
                        condition: None
                    }
                ])
            })
        );

        test_parse!(tokens, Packet, Err(ParseError::EndOfFile));
        assert!(tokens.is_empty());
    }
}
