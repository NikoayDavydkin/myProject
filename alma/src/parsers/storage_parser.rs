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

pub struct StorageParser {
    pub grammar: Grammar,
    pub parser: ProductParser,
}

impl StorageParser {
    pub fn new() -> Self {
        let grammar = Grammar::Sequence(
            [
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
                ),
                Grammar::Or(
                    [
                        Grammar::new_with_str_value("ssd", "type", "ssd"),
                        Grammar::new_with_str_value("pcie ssd", "type", "ssd"),
                        Grammar::new_with_str_value("nvme ssd", "type", "ssd"),
                        Grammar::new_with_str_value("nvme pcie ssd", "type", "ssd"),
                        Grammar::new_with_str_value("pcie nvme ssd", "type", "ssd"),
                        Grammar::new_with_str_value("emmc", "type", "emmc"),
                        Grammar::new_with_str_value("hard drive", "type", "hard drive"),
                    ]
                    .to_vec(),
                ),
            ]
            .to_vec(),
        );
        let mut parser = ProductParser::new();
        ProductParser::create_parser(attribute_ids::SSD_SIZE, &grammar, &mut parser);
        Self { grammar, parser }
    }
}

impl ParserTrait for StorageParser {
    fn parse(&self, index: usize, tokens: &[String]) -> Option<ParseResult> {
        let result = self.parser.parse(index, &tokens.to_vec())?;
        if let AttributeValue::U64(size) = result.values.get("size")? {
            if let AttributeValue::String(units) = result.values.get("units")? {
                if let AttributeValue::String(drive_type) = result.values.get("type")? {
                    let size_gb = match units.as_str() {
                        "gb" => Some(*size),
                        "tb" => Some(size * 1000),
                        _ => None,
                    }?;
                    let mut attributes = Attributes::new();
                    match drive_type.as_str() {
                        "ssd" => {
                            if size_gb >= 128 {
                                attributes
                                    .insert(attribute_ids::SSD_SIZE, AttributeValue::U64(size_gb));
                            }
                        }
                        "emmc" => {
                            attributes
                                .insert(attribute_ids::EMMC_SIZE, AttributeValue::U64(size_gb));
                        }
                        "hard drive" => {
                            attributes
                                .insert(attribute_ids::HDD_SIZE, AttributeValue::U64(size_gb));
                        }
                        _ => {}
                    }
                    if !attributes.attributes.is_empty() {
                        return Some(ParseResult {
                            start: index,
                            end: 0,
                            attributes,
                        });
                    }
                }
            }
        }
        None
    }
}
