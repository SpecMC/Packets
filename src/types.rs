use specmc_base::{
    ensure_tokens,
    parse::{Identifier, Parse, ParseError},
};

use crate::base::{BaseType, FieldList};

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
