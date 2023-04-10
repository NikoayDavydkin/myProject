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

pub struct ScreenSizeParser {
    pub grammar: Grammar,
    pub parser: ProductParser,
}

impl ScreenSizeParser {
    pub fn new() -> Self {
        let grammar = Grammar::Or(
            [
                Grammar::Token(
                    Token::F64(F64 {
                        name: "size".to_owned(),
                        units: set! {"in", "inch", "inches"},
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
                        Grammar::Option(Box::new(Grammar::new("-"))),
                        Grammar::Or(
                            [
                                Grammar::new("\""),
                                Grammar::new("”"),
                                Grammar::new("in"),
                                Grammar::new("inch"),
                                Grammar::new("inches"),
                            ]
                            .to_vec(),
                        ),
                    ]
                    .to_vec(),
                ),
            ]
            .to_vec(),
        );
        let mut parser = ProductParser::new();
        ProductParser::create_parser(attribute_ids::SCREEN_SIZE, &grammar, &mut parser);
        Self { grammar, parser }
    }
}

impl ParserTrait for ScreenSizeParser {
    fn parse(&self, index: usize, tokens: &[String]) -> Option<ParseResult> {
        let result = self.parser.parse(index, &tokens.to_vec())?;
        if let AttributeValue::F64(size) = result.values.get("size")? {
            if *size > 4.0 {
                let mut attributes = Attributes::new();
                attributes.insert(attribute_ids::SCREEN_SIZE, AttributeValue::F64(*size));
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

#[cfg(test)]
mod tests {
    use bonfire_core::tokenize::tokenize;

    use super::*;

    #[test]
    fn test_parse_1() {
        let parser = ScreenSizeParser::new();
        let tokens = tokenize("14.4 in");
        let result = parser.parse(0, &tokens);
        assert_eq!(
            result
                .unwrap()
                .attributes
                .get_f64(&attribute_ids::SCREEN_SIZE),
            Some(14.4)
        );

        let tokens = tokenize("17.3\"");
        let result = parser.parse(0, &tokens);
        assert_eq!(
            result
                .unwrap()
                .attributes
                .get_f64(&attribute_ids::SCREEN_SIZE),
            Some(17.3)
        );

        let tokens = tokenize("16”");
        let result = parser.parse(0, &tokens);
        assert_eq!(
            result
                .unwrap()
                .attributes
                .get_f64(&attribute_ids::SCREEN_SIZE),
            Some(16.0)
        );

        let tokens = tokenize("15 inches");
        let result = parser.parse(0, &tokens);
        assert_eq!(
            result
                .unwrap()
                .attributes
                .get_f64(&attribute_ids::SCREEN_SIZE),
            Some(15.0)
        );

        let tokens = tokenize("15in");
        let result = parser.parse(0, &tokens);
        assert_eq!(
            result
                .unwrap()
                .attributes
                .get_f64(&attribute_ids::SCREEN_SIZE),
            Some(15.0)
        );
    }
}
