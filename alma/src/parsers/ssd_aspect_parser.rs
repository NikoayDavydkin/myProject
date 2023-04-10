use bonfire_core::attribute_value::AttributeValue;
use bonfire_core::attributes::Attributes;
use bonfire_core::grammar::Grammar;
use bonfire_ids::attribute_ids;
use product_parser::ProductParser;

use crate::parsers::parse_result::ParseResult;
use crate::parsers::parser_trait::ParserTrait;
use crate::parsers::ram_parser::RamParser;

pub struct SsdAspectParser {
    pub grammar: Grammar,
    pub parser: ProductParser,
}

impl SsdAspectParser {
    pub fn new() -> Self {
        let grammar = Grammar::Sequence(
            [
                RamParser::size_grammar(),
                Grammar::Option(Box::new(Grammar::Or(
                    [
                        Grammar::new_with_str_value("ssd", "type", "ssd"),
                        Grammar::new_with_str_value("pcie ssd", "type", "ssd"),
                        Grammar::new_with_str_value("nvme ssd", "type", "ssd"),
                        Grammar::new_with_str_value("nvme pcie ssd", "type", "ssd"),
                        Grammar::new_with_str_value("pcie nvme ssd", "type", "ssd"),
                    ]
                    .to_vec(),
                ))),
            ]
            .to_vec(),
        );
        let mut parser = ProductParser::new();
        ProductParser::create_parser(attribute_ids::SSD_SIZE, &grammar, &mut parser);
        Self { grammar, parser }
    }
}

impl ParserTrait for SsdAspectParser {
    fn parse(&self, index: usize, tokens: &[String]) -> Option<ParseResult> {
        let result = self.parser.parse(index, &tokens.to_vec())?;
        if let AttributeValue::U64(size) = result.values.get("size")? {
            if let AttributeValue::String(units) = result.values.get("units")? {
                let mut attributes = Attributes::new();
                match units.as_str() {
                    "gb" => {
                        attributes.insert(attribute_ids::SSD_SIZE, AttributeValue::U64(*size));
                    }
                    "tb" => {
                        attributes
                            .insert(attribute_ids::SSD_SIZE, AttributeValue::U64(size * 1000));
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

#[cfg(test)]
mod tests {
    use bonfire_core::tokenize::tokenize;

    use super::*;

    #[test]
    fn test_parse_1() {
        let parser = SsdAspectParser::new();
        let tokens = tokenize("8TB SSD");
        let result = parser.parse(0, &tokens);
        assert_eq!(
            result.unwrap().attributes.get_u64(&attribute_ids::SSD_SIZE),
            Some(8000)
        );

        let tokens = tokenize("512GB");
        let result = parser.parse(0, &tokens);
        assert_eq!(
            result.unwrap().attributes.get_u64(&attribute_ids::SSD_SIZE),
            Some(512)
        );

        let tokens = tokenize("16 GB");
        let result = parser.parse(0, &tokens);
        assert_eq!(
            result.unwrap().attributes.get_u64(&attribute_ids::SSD_SIZE),
            Some(16)
        );

        let tokens = tokenize("2TB");
        let result = parser.parse(0, &tokens);
        assert_eq!(
            result.unwrap().attributes.get_u64(&attribute_ids::SSD_SIZE),
            Some(2000)
        );
    }
}
