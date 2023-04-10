use bonfire_core::attribute_value::AttributeValue;
use bonfire_core::attributes::Attributes;
use bonfire_core::grammar::Grammar;
use bonfire_core::grammar::Token;
use bonfire_core::grammar::U64;
use bonfire_ids::attribute_ids;
use hashbrown::HashMap;
use hashbrown::HashSet;
use literal::{set, SetLiteral};
use product_parser::ProductParser;

use crate::parsers::parse_result::ParseResult;
use crate::parsers::parser_trait::ParserTrait;

pub struct OperatingSystemParser {
    pub grammar: Grammar,
    pub parser: ProductParser,
}

impl OperatingSystemParser {
    pub fn new() -> Self {
        let grammar = Grammar::Or(
            [
                Grammar::Sequence(
                    [
                        Grammar::Token(
                            Token::String("windows".to_owned()),
                            [(
                                "system".to_owned(),
                                AttributeValue::String("Windows".to_owned()),
                            )]
                            .into(),
                        ),
                        Grammar::Option(Box::new(Grammar::Or(
                            [
                                Grammar::Token(
                                    Token::U64(U64 {
                                        name: "version".to_owned(),
                                        units: set! {},
                                    }),
                                    HashMap::new(),
                                ),
                                Grammar::Token(
                                    Token::String("XP".to_owned()),
                                    [(
                                        "version".to_owned(),
                                        AttributeValue::String("XP".to_owned()),
                                    )]
                                    .into(),
                                ),
                            ]
                            .to_vec(),
                        ))),
                        Grammar::Option(Box::new(Grammar::Or(
                            [
                                Grammar::Token(
                                    Token::String("pro".to_owned()),
                                    [(
                                        "edition".to_owned(),
                                        AttributeValue::String("Pro".to_owned()),
                                    )]
                                    .into(),
                                ),
                                Grammar::Token(
                                    Token::String("professional".to_owned()),
                                    [(
                                        "edition".to_owned(),
                                        AttributeValue::String("Pro".to_owned()),
                                    )]
                                    .into(),
                                ),
                                Grammar::Token(
                                    Token::String("home".to_owned()),
                                    [(
                                        "edition".to_owned(),
                                        AttributeValue::String("Home".to_owned()),
                                    )]
                                    .into(),
                                ),
                            ]
                            .to_vec(),
                        ))),
                    ]
                    .to_vec(),
                ),
                Grammar::Sequence(
                    [
                        Grammar::Token(Token::String("chrome".to_owned()), HashMap::new()),
                        Grammar::Token(
                            Token::String("os".to_owned()),
                            [(
                                "system".to_owned(),
                                AttributeValue::String("Chrome OS".to_owned()),
                            )]
                            .into(),
                        ),
                    ]
                    .to_vec(),
                ),
            ]
            .to_vec(),
        );
        let mut parser = ProductParser::new();
        ProductParser::create_parser(attribute_ids::OPERATING_SYSTEM, &grammar, &mut parser);
        Self { grammar, parser }
    }
}

impl ParserTrait for OperatingSystemParser {
    fn parse(&self, index: usize, tokens: &[String]) -> Option<ParseResult> {
        let result = self.parser.parse(index, &tokens.to_vec())?;
        if let AttributeValue::String(system) = result.values.get("system")? {
            let mut os = system.to_owned();

            if let Some(version) = result.values.get("version") {
                let version = match version {
                    AttributeValue::U64(version) => {
                        os += " ";
                        os += &version.to_string();
                        Some(version.to_string())
                    }
                    AttributeValue::String(version) => {
                        os += " ";
                        os += version;
                        Some(version.to_string())
                    }
                    _ => None,
                };

                if let Some(version) = version {
                    match result.values.get("edition") {
                        Some(edition) => {
                            if let AttributeValue::String(edition) = edition {
                                os += " ";
                                os += edition;
                            }
                        }
                        _ => {
                            let set: HashSet<&str> =
                                vec!["XP", "7", "8", "10", "11"].into_iter().collect();
                            if system == "Windows" && set.contains(version.as_str()) {
                                os += " ";
                                os += "Home";
                            } else {
                                println!("Invalid version: {}", version);
                            }
                        }
                    }
                }
            }

            let mut attributes = Attributes::new();
            attributes.insert(attribute_ids::OPERATING_SYSTEM, AttributeValue::String(os));
            return Some(ParseResult {
                start: index,
                end: 0,
                attributes,
            });
        }
        None
    }
}
