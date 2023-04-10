use bonfire_core::attribute_value::AttributeValue;
use bonfire_core::attributes::Attributes;
use bonfire_core::grammar::Grammar;
use bonfire_core::grammar::Token;
use bonfire_core::grammar::F64;
use bonfire_ids::attribute_ids;
use hashbrown::HashMap;
use literal::{set, SetLiteral};
use product_parser::ProductParser;

use crate::parsers::parse_result::ParseResult;
use crate::parsers::parser_trait::ParserTrait;

pub struct StorageCapacityParser {
    pub grammar: Grammar,
    pub parser: ProductParser,
}

impl StorageCapacityParser {
    pub fn new() -> Self {
        let grammar = Grammar::Sequence(
            [Grammar::Or(
                [
                    Grammar::Token(
                        Token::F64(F64 {
                            name: "size".to_owned(),
                            units: set! {"gb", "tb"},
                        }),
                        HashMap::new(),
                    ),
                    Grammar::Sequence(
                        [
                            Grammar::Token(
                                Token::F64(F64 {
                                    name: "size".to_owned(),
                                    units: set! {},
                                }),
                                HashMap::new(),
                            ),
                            Grammar::Or(
                                [
                                    Grammar::Token(
                                        Token::String("gb".to_owned()),
                                        [(
                                            "units".to_owned(),
                                            AttributeValue::String("gb".to_owned()),
                                        )]
                                        .into(),
                                    ),
                                    Grammar::Token(
                                        Token::String("tb".to_owned()),
                                        [(
                                            "units".to_owned(),
                                            AttributeValue::String("tb".to_owned()),
                                        )]
                                        .into(),
                                    ),
                                ]
                                .to_vec(),
                            ),
                        ]
                        .to_vec(),
                    ),
                ]
                .to_vec(),
            )]
            .to_vec(),
        );
        let mut parser = ProductParser::new();
        ProductParser::create_parser(attribute_ids::SSD_SIZE, &grammar, &mut parser);
        Self { grammar, parser }
    }
}

impl ParserTrait for StorageCapacityParser {
    fn parse(&self, index: usize, tokens: &[String]) -> Option<ParseResult> {
        let result = self.parser.parse(index, &tokens.to_vec())?;
        if let AttributeValue::F64(size) = result.values.get("size")? {
            if let AttributeValue::String(units) = result.values.get("units")? {
                let size_bytes = match units.as_str() {
                    "gb" => Some((size * 1000.0) as u64 * 1000 * 1000),
                    "tb" => Some((size * 1000.0) as u64 * 1000 * 1000 * 1000),
                    _ => None,
                }?;
                let mut attributes = Attributes::new();
                attributes.insert(
                    attribute_ids::STORAGE_CAPACITY,
                    AttributeValue::U64(size_bytes),
                );
                if !attributes.attributes.is_empty() {
                    return Some(ParseResult {
                        start: index,
                        end: 0,
                        attributes,
                    });
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use bonfire_core::tokenize::tokenize;

    use super::*;

    #[test]
    fn test_parse_1() {
        let parser = StorageCapacityParser::new();
        let tokens = tokenize("8TB");
        let result = parser.parse(0, &tokens);
        assert_eq!(
            result
                .unwrap()
                .attributes
                .get_u64(&attribute_ids::STORAGE_CAPACITY),
            Some(8000000000000)
        );

        let tokens = tokenize("512GB");
        let result = parser.parse(0, &tokens);
        assert_eq!(
            result
                .unwrap()
                .attributes
                .get_u64(&attribute_ids::STORAGE_CAPACITY),
            Some(512000000000)
        );

        let tokens = tokenize("7.68TB");
        let result = parser.parse(0, &tokens);
        assert_eq!(
            result
                .unwrap()
                .attributes
                .get_u64(&attribute_ids::STORAGE_CAPACITY),
            Some(7680000000000)
        );
    }
}
