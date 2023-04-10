use bonfire_core::attribute_value::AttributeValue;
use bonfire_core::attribute_value::ScreenResolution;
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

pub struct ScreenResolutionParser {
    pub grammar: Grammar,
    pub parser: ProductParser,
}

impl ScreenResolutionParser {
    pub fn new_screen_resolution(width: u64, height: u64) -> Grammar {
        let symbol = "x";
        let text = std::format!("{}{}{}", width, symbol, height);
        Grammar::Token(
            Token::String(text),
            [
                ("width".to_owned(), AttributeValue::U64(width)),
                ("height".to_owned(), AttributeValue::U64(height)),
            ]
            .into(),
        )
    }

    pub fn new() -> Self {
        let grammar = Grammar::Or(
            [
                ScreenResolutionParser::new_screen_resolution(1024, 600),
                ScreenResolutionParser::new_screen_resolution(1280, 720),
                ScreenResolutionParser::new_screen_resolution(1280, 800),
                ScreenResolutionParser::new_screen_resolution(1366, 768),
                ScreenResolutionParser::new_screen_resolution(1600, 768),
                ScreenResolutionParser::new_screen_resolution(1600, 900),
                ScreenResolutionParser::new_screen_resolution(1536, 1024),
                ScreenResolutionParser::new_screen_resolution(1920, 1080),
                ScreenResolutionParser::new_screen_resolution(1920, 1200),
                ScreenResolutionParser::new_screen_resolution(1920, 1280),
                ScreenResolutionParser::new_screen_resolution(2160, 1440),
                ScreenResolutionParser::new_screen_resolution(2256, 1504),
                ScreenResolutionParser::new_screen_resolution(2560, 1440),
                ScreenResolutionParser::new_screen_resolution(2560, 1600),
                ScreenResolutionParser::new_screen_resolution(2496, 1664),
                ScreenResolutionParser::new_screen_resolution(2880, 1800),
                ScreenResolutionParser::new_screen_resolution(2880, 1920),
                ScreenResolutionParser::new_screen_resolution(3072, 1920),
                ScreenResolutionParser::new_screen_resolution(3000, 2000),
                ScreenResolutionParser::new_screen_resolution(3240, 2160),
                ScreenResolutionParser::new_screen_resolution(3300, 2200),
                ScreenResolutionParser::new_screen_resolution(3840, 2400),
                ScreenResolutionParser::new_screen_resolution(3840, 2560),
                Grammar::Sequence(
                    [
                        Grammar::Token(
                            Token::U64(U64 {
                                name: "width".to_owned(),
                                units: set! {},
                            }),
                            HashMap::new(),
                        ),
                        Grammar::new("x"),
                        Grammar::Token(
                            Token::U64(U64 {
                                name: "height".to_owned(),
                                units: set! {},
                            }),
                            HashMap::new(),
                        ),
                    ]
                    .to_vec(),
                ),
            ]
            .to_vec(),
        );
        let mut parser = ProductParser::new();
        ProductParser::create_parser(attribute_ids::SCREEN_RESOLUTION, &grammar, &mut parser);
        Self { grammar, parser }
    }
}

impl ParserTrait for ScreenResolutionParser {
    fn parse(&self, index: usize, tokens: &[String]) -> Option<ParseResult> {
        let result = self.parser.parse(index, &tokens.to_vec())?;
        if let AttributeValue::U64(width) = result.values.get("width")? {
            if let AttributeValue::U64(height) = result.values.get("height")? {
                if *width > 400 && *height > 400 {
                    let mut attributes = Attributes::new();
                    attributes.insert(
                        attribute_ids::SCREEN_RESOLUTION,
                        AttributeValue::ScreenResolution(ScreenResolution {
                            width: *width,
                            height: *height,
                        }),
                    );
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
