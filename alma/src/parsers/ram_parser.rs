use bonfire_core::attribute_value::AttributeValue;
use bonfire_core::attributes::Attributes;
use bonfire_core::grammar::Grammar;
use bonfire_core::grammar::Token;
use bonfire_core::grammar::U64;
use bonfire_ids::attribute_ids;
use hashbrown::HashMap;
use literal::{set, SetLiteral};
use product_parser::ProductParser;

use crate::parsers::parse_result::ParseResult;
use crate::parsers::parser_trait::ParserTrait;

pub struct RamParser {
    pub grammar: Grammar,
    pub parser: ProductParser,
}

impl RamParser {
    pub fn size_grammar() -> Grammar {
        Grammar::Or(
            [
                Grammar::Token(
                    Token::U64(U64 {
                        name: "size".to_owned(),
                        units: set! {"gb", "tb"},
                    }),
                    HashMap::new(),
                ),
                Grammar::Sequence(
                    [
                        Grammar::Token(
                            Token::U64(U64 {
                                name: "size".to_owned(),
                                units: set! {},
                            }),
                            HashMap::new(),
                        ),
                        Grammar::Or(
                            [
                                Grammar::Token(
                                    Token::String("gb".to_owned()),
                                    [
                                        (
                                            "units".to_owned(),
                                            AttributeValue::String("gb".to_owned()),
                                        ),
                                    ]
                                    .into(),
                                ),
                                Grammar::Token(
                                    Token::String("tb".to_owned()),
                                    [
                                        (
                                            "units".to_owned(),
                                            AttributeValue::String("tb".to_owned()),
                                        ),
                                    ]
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
        )
    }
    pub fn memory_type_grammar() -> Grammar {
        Grammar::Or(
            [
                Grammar::new("ddr3"),
                Grammar::new("ddr3l"),
                Grammar::new("lpddr3"),
                Grammar::new("ddr4"),
                Grammar::new("ddr4l"),
                Grammar::new("ddr4u"),
                Grammar::new("lpddr4"),
                Grammar::new("lpddr4x"),
            ]
            .to_vec(),
        )
    }
    pub fn new() -> Self {
        let grammar = Grammar::Sequence(
            [
                RamParser::size_grammar(),
                Grammar::Or(
                    [
                        Grammar::Sequence(
                            [
                                Grammar::Option(Box::new(RamParser::memory_type_grammar())),
                                Grammar::new("ram"),
                            ]
                            .to_vec(),
                        ),
                        RamParser::memory_type_grammar(),
                    ]
                    .to_vec(),
                ),
            ]
            .to_vec(),
        );
        let mut parser = ProductParser::new();
        ProductParser::create_parser(attribute_ids::RAM_SIZE, &grammar, &mut parser);
        Self { grammar, parser }
    }
}

impl ParserTrait for RamParser {
    fn parse(&self, index: usize, tokens: &[String]) -> Option<ParseResult> {
        let result = self.parser.parse(index, &tokens.to_vec())?;
        if let AttributeValue::U64(size) = result.values.get("size")? {
            if let AttributeValue::String(units) = result.values.get("units")? {
                let mut attributes = Attributes::new();
                match units.as_str() {
                    "gb" => {
                        attributes.insert(attribute_ids::RAM_SIZE, AttributeValue::U64(*size));
                    }
                    "tb" => {
                        attributes
                            .insert(attribute_ids::RAM_SIZE, AttributeValue::U64(size * 1000));
                    }
                    _ => return None,
                }
                return Some(ParseResult {
                    start: index,
                    end: 0,
                    attributes,
                });
            }
        }
        None
    }
}
