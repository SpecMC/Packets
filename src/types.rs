use specmc_base::{
    ensure_tokens,
    parse::{Identifier, Parse, ParseError},
};

use crate::base::{BaseType, FieldList};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct CustomType {
    pub name: Identifier,
    pub fields: FieldList,
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use specmc_base::tokenize;

    use crate::{
        base::{Field, IntegerType},
        test_parse,
    };

    use super::*;

    #[test]
    fn test_type() {
        let mut tokens: Vec<String> = tokenize!("bool i32 TestType");

        test_parse!(tokens, Type, Ok(Type::BaseType(BaseType::Bool)));
        test_parse!(
            tokens,
            Type,
            Ok(Type::BaseType(BaseType::Integer(IntegerType::I32)))
        );
        test_parse!(
            tokens,
            Type,
            Ok(Type::CustomType(Identifier("TestType".to_string())))
        );

        assert!(tokens.is_empty());
        test_parse!(tokens, Type, Err(ParseError::EndOfFile));
    }

    #[test]
    fn test_custom_type() {
        let mut tokens: Vec<String> = tokenize!(
            "
            type TestType {
                i32 a
                bool b
                if (b) {
                    i32 c
                }
            }
            "
        );

        test_parse!(
            tokens,
            CustomType,
            Ok(CustomType {
                name: Identifier("TestType".to_string()),
                fields: FieldList(vec![
                    Field {
                        ty: Type::BaseType(BaseType::Integer(IntegerType::I32)),
                        name: Identifier("a".to_string()),
                        value: None,
                        conditions: HashSet::new(),
                    },
                    Field {
                        ty: Type::BaseType(BaseType::Bool),
                        name: Identifier("b".to_string()),
                        value: None,
                        conditions: HashSet::new(),
                    },
                    Field {
                        ty: Type::BaseType(BaseType::Integer(IntegerType::I32)),
                        name: Identifier("c".to_string()),
                        value: None,
                        conditions: HashSet::from_iter(vec!["b".to_string()]),
                    },
                ])
            })
        );

        assert!(tokens.is_empty());
        test_parse!(tokens, CustomType, Err(ParseError::EndOfFile));
    }
}
