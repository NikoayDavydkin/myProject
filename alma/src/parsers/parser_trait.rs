use crate::parsers::parse_result::ParseResult;

pub trait ParserTrait {
    fn parse(&self, index: usize, tokens: &[String]) -> Option<ParseResult>;
}
