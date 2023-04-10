use bonfire_core::attribute_histogram::AttributeHistogram;
use bonfire_core::attribute_value::AttributeValue;
use bonfire_core::attributes::Attributes;
use bonfire_core::tokenize::tokenize;
use bonfire_ids::attribute_ids;
use bonfire_ids::category_ids;
use product_parser::ProductParser;
use std::sync::Arc;
use uuid::Uuid;

use crate::parsers::operating_system_parser::OperatingSystemParser;
use crate::parsers::parser_trait::ParserTrait;
use crate::parsers::ram_aspect_parser::RamAspectParser;
use crate::parsers::ram_parser::RamParser;
use crate::parsers::screen_resolution_parser::ScreenResolutionParser;
use crate::parsers::screen_size_parser::ScreenSizeParser;
use crate::parsers::ssd_aspect_parser::SsdAspectParser;
use crate::parsers::storage_capacity_parser::StorageCapacityParser;
use crate::parsers::storage_parser::StorageParser;

pub struct Parser {
    category: Uuid,
    parsers: Vec<Arc<dyn ParserTrait + Send + Sync>>,
    screen_size_parser: ScreenSizeParser,
    operating_system_parser: OperatingSystemParser,
    screen_resolution_parser: ScreenResolutionParser,
    pub cpu_parser: Option<ProductParser>,
    pub gpu_parser: Option<ProductParser>,
    ram_aspect_parser: RamAspectParser,
    ssd_aspect_parser: SsdAspectParser,
}

impl Parser {
    pub fn new(category: &Uuid) -> Self {
        let screen_size_parser = ScreenSizeParser::new();
        let operating_system_parser = OperatingSystemParser::new();
        let screen_resolution_parser = ScreenResolutionParser::new();

        let parsers: Vec<Arc<dyn ParserTrait + Send + Sync>> = match *category {
            category_ids::LAPTOPS => {
                vec![
                    Arc::new(RamParser::new()),
                    Arc::new(StorageParser::new()),
                    Arc::new(ScreenSizeParser::new()),
                    Arc::new(OperatingSystemParser::new()),
                    Arc::new(ScreenResolutionParser::new()),
                ]
            }
            category_ids::INTERNAL_SOLID_STATE_DRIVES => {
                vec![Arc::new(StorageCapacityParser::new())]
            }
            category_ids::INTERNAL_HARD_DRIVES => {
                vec![Arc::new(StorageCapacityParser::new())]
            }
            category_ids::EXTERNAL_SOLID_STATE_DRIVES => {
                vec![Arc::new(StorageCapacityParser::new())]
            }
            category_ids::EXTERNAL_HARD_DRIVES => {
                vec![Arc::new(StorageCapacityParser::new())]
            }
            _ => Vec::new(),
        };
        let cpu_parser = None;
        let gpu_parser = None;

        let ram_aspect_parser = RamAspectParser::new();
        let ssd_aspect_parser = SsdAspectParser::new();

        Self {
            category: *category,
            parsers,
            screen_size_parser,
            operating_system_parser,
            screen_resolution_parser,
            cpu_parser,
            gpu_parser,
            ram_aspect_parser,
            ssd_aspect_parser,
        }
    }

    pub fn parse_title(&self, title: &str, histogram: &mut AttributeHistogram) {
        let tokens = tokenize(title);
        let mut index: usize = 0;
        while index < tokens.len() {
            for parser in &self.parsers {
                if let Some(result) = parser.parse(index, &tokens) {
                    histogram.extend(&result.attributes);
                }
            }
            if let Some(cpu_parser) = &self.cpu_parser {
                if let Some(result) = cpu_parser.parse(index, &tokens) {
                    histogram.insert(&attribute_ids::CPU_ID, &AttributeValue::Uuid(result.id));
                }
            }
            if let Some(gpu_parser) = &self.gpu_parser {
                if let Some(result) = gpu_parser.parse(index, &tokens) {
                    histogram.insert(&attribute_ids::GPU_ID, &AttributeValue::Uuid(result.id));
                }
            }
            index += 1;
        }
    }

    pub fn parse_aspect<T: ParserTrait>(
        &self,
        title: &str,
        parser: &T,
        histogram: &mut AttributeHistogram,
    ) {
        let tokens = tokenize(title);
        let mut index: usize = 0;
        while index < tokens.len() {
            if let Some(result) = parser.parse(index, &tokens) {
                histogram.extend(&result.attributes);
            }
            index += 1;
        }
    }

    pub fn parse_product_parser_aspect(
        &self,
        title: &str,
        attribute_id: Uuid,
        parser: &ProductParser,
        histogram: &mut AttributeHistogram,
    ) {
        let tokens = tokenize(title);
        let mut index: usize = 0;
        while index < tokens.len() {
            if let Some(result) = parser.parse(index, &tokens) {
                histogram.insert(&attribute_id, &AttributeValue::Uuid(result.id));
            }
            index += 1;
        }
    }

    pub fn parse(&self, attributes: &Attributes) -> Attributes {
        let mut histogram = AttributeHistogram::new();
        let mut spam = false;
        if let Some(features) = attributes.get_string_array(&attribute_ids::AMAZON_FEATURES) {
            for feature in features {
                self.parse_title(feature, &mut histogram);
            }
        }
        if let Some(title) = attributes.get_str(&attribute_ids::TITLE) {
            self.parse_title(title, &mut histogram);
            spam = spam || title.contains("Keyboard Cover");
        }
        if let Some(aspects) = attributes.get_string_value_map(&attribute_ids::ASPECTS) {
            for (name, value) in aspects {
                #[allow(clippy::single_match)]
                match value {
                    AttributeValue::String(value) => match name.as_str() {
                        "Max Screen Resolution" | "Maximum Resolution" => {
                            self.parse_aspect(
                                value,
                                &self.screen_resolution_parser,
                                &mut histogram,
                            );
                        }
                        "RAM Size" => {
                            self.parse_aspect(value, &self.ram_aspect_parser, &mut histogram);
                        }
                        "Operating System" => {
                            self.parse_aspect(value, &self.operating_system_parser, &mut histogram);
                        }
                        "Screen Size" => {
                            self.parse_aspect(value, &self.screen_size_parser, &mut histogram);
                        }
                        "Brand" => {
                            let mut new_attributes = Attributes::new();
                            new_attributes.insert(
                                attribute_ids::BRAND,
                                AttributeValue::String(value.clone()),
                            );
                            histogram.extend(&new_attributes);
                        }
                        "Processor" => {
                            if let Some(cpu_parser) = &self.cpu_parser {
                                self.parse_product_parser_aspect(
                                    value,
                                    attribute_ids::CPU_ID,
                                    cpu_parser,
                                    &mut histogram,
                                );
                            }
                        }
                        "GPU" => {
                            if let Some(gpu_parser) = &self.gpu_parser {
                                self.parse_product_parser_aspect(
                                    value,
                                    attribute_ids::GPU_ID,
                                    gpu_parser,
                                    &mut histogram,
                                );
                            }
                        }
                        "SSD Capacity" => {
                            self.parse_aspect(value, &self.ssd_aspect_parser, &mut histogram);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
        let mut return_value = Parser::attributes_from_histogram(&histogram);
        if let Some(screen_size) = return_value.get_f64(&attribute_ids::SCREEN_SIZE) {
            spam = spam || screen_size > 17.3;
        }
        if spam || (category_ids::LAPTOPS == self.category && return_value.attributes.len() < 3) {
            return_value.insert(
                attribute_ids::SPAM,
                AttributeValue::String("Spam".to_owned()),
            );
        } else {
            return_value.insert(
                attribute_ids::SPAM,
                AttributeValue::String("Not Spam".to_owned()),
            );
        }
        return_value
    }

    pub fn attributes_from_histogram(orig_histogram: &AttributeHistogram) -> Attributes {
        let mut histogram = orig_histogram.clone();
        for ssd_size in histogram.u64_sorted_values(&attribute_ids::SSD_SIZE) {
            if let Some(ram_size) = histogram.u64_histograms.get_mut(&attribute_ids::RAM_SIZE) {
                ram_size.remove(&ssd_size);
            }
        }
        for ssd_size in histogram.u64_sorted_values(&attribute_ids::SSD_SIZE) {
            if ssd_size > 16000 {
                if let Some(hist) = histogram.u64_histograms.get_mut(&attribute_ids::SSD_SIZE) {
                    hist.remove(&ssd_size);
                }
            }
        }
        for ram_size in histogram.u64_sorted_values(&attribute_ids::RAM_SIZE) {
            if ram_size > 256 {
                if let Some(hist) = histogram.u64_histograms.get_mut(&attribute_ids::RAM_SIZE) {
                    hist.remove(&ram_size);
                }
            }
        }
        let mut return_value = Attributes::new();
        for (id, hist) in &histogram.u64_histograms {
            if 1 == hist.len() {
                return_value.insert(*id, AttributeValue::U64(*hist.keys().next().unwrap()));
            }
        }
        for (id, hist) in &histogram.string_histograms {
            if 1 == hist.len() {
                return_value.insert(
                    *id,
                    AttributeValue::String(hist.keys().next().unwrap().clone()),
                );
            } else if hist.len() > 1 && attribute_ids::OPERATING_SYSTEM == *id {
                let value = hist.keys().fold(hist.keys().next().unwrap(), |acc, item| {
                    if item.replace(" Home", "").len() > acc.replace(" Home", "").len() {
                        item
                    } else {
                        acc
                    }
                });
                if hist
                    .keys()
                    .all(|os| value.contains(&os.replace(" Home", "")))
                {
                    return_value.insert(*id, AttributeValue::String(value.clone()));
                }
            }
        }
        let screen_size = histogram.screen_size_sorted_values();
        if 1 == screen_size.len() {
            return_value.insert(
                attribute_ids::SCREEN_SIZE,
                AttributeValue::F64(screen_size[0]),
            );
        }
        for (id, hist) in &histogram.screen_resolution_histograms {
            if 1 == hist.len() {
                return_value.insert(
                    *id,
                    AttributeValue::ScreenResolution(hist.keys().next().unwrap().clone()),
                );
            }
        }
        for (id, hist) in &histogram.uuid_histograms {
            if 1 == hist.len() {
                return_value.insert(*id, AttributeValue::Uuid(*hist.keys().next().unwrap()));
            }
        }
        return_value
    }
}

#[cfg(test)]
mod tests {
    use bonfire_core::attribute_value::ScreenResolution;
    use bonfire_core::grammar::Grammar;
    use hashbrown::HashMap;

    use super::*;

    #[test]
    fn test_parse_title_1() {
        let parser = Parser::new(&category_ids::LAPTOPS);
        let mut histogram = AttributeHistogram::new();
        parser.parse_title(
            "4 GB RAM, 64 GB eMMC Storage, 15.6 inch HD Touchscreen, Windows 10 Home in S Mode",
            &mut histogram,
        );
        let attributes = Parser::attributes_from_histogram(&histogram);
        assert_eq!(attributes.get_u64(&attribute_ids::RAM_SIZE), Some(4));
        assert_eq!(attributes.get_u64(&attribute_ids::EMMC_SIZE), Some(64));
        assert_eq!(attributes.get_f64(&attribute_ids::SCREEN_SIZE), Some(15.6));
        assert_eq!(
            attributes.get_str(&attribute_ids::OPERATING_SYSTEM),
            Some("Windows 10 Home")
        );
    }

    #[test]
    fn test_parse_title_2() {
        let parser = Parser::new(&category_ids::LAPTOPS);
        let mut histogram = AttributeHistogram::new();
        parser.parse_title("2560 x 1600•16 inches", &mut histogram);
        let attributes = Parser::attributes_from_histogram(&histogram);
        assert_eq!(
            attributes.get_screen_resolution(&attribute_ids::SCREEN_RESOLUTION),
            Some(&ScreenResolution::new(2560, 1600))
        );
        assert_eq!(attributes.get_f64(&attribute_ids::SCREEN_SIZE), Some(16.0));
    }

    #[test]
    fn test_parse_title_3() {
        let parser = Parser::new(&category_ids::LAPTOPS);
        let mut histogram = AttributeHistogram::new();
        parser.parse_title("Windows 7 Pro Windows 10", &mut histogram);
        let attributes = Parser::attributes_from_histogram(&histogram);
        assert_eq!(attributes.get_str(&attribute_ids::OPERATING_SYSTEM), None);
    }

    /*#[test]
    fn test_parse_title_4() {
        let parser = Parser::new();
        let mut histogram = AttributeHistogram::new();
        parser.parse_title("SSD 256GB RAM 8GB", &mut histogram);
        let attributes = Parser::attributes_from_histogram(&histogram);
        assert_eq!(attributes.get_u64(&attribute_ids::RAM_SIZE), Some(8));
        assert_eq!(attributes.get_u64(&attribute_ids::SSD_SIZE), Some(256));
    }*/

    #[test]
    fn test_parse_title_5() {
        let parser = Parser::new(&category_ids::LAPTOPS);
        let mut histogram = AttributeHistogram::new();
        parser.parse_title("256TB SSD Storage 512GB RAM", &mut histogram);
        let attributes = Parser::attributes_from_histogram(&histogram);
        assert_eq!(attributes.get_u64(&attribute_ids::SSD_SIZE), None);
        assert_eq!(attributes.get_u64(&attribute_ids::RAM_SIZE), None);
    }

    #[test]
    fn test_parse_title_6() {
        let parser = Parser::new(&category_ids::LAPTOPS);
        let mut histogram = AttributeHistogram::new();
        parser.parse_title("17.3&amp;quot;", &mut histogram);
        let attributes = Parser::attributes_from_histogram(&histogram);
        assert_eq!(attributes.get_f64(&attribute_ids::SCREEN_SIZE), Some(17.3));
    }

    #[test]
    fn test_parse_1() {
        let parser = Parser::new(&category_ids::LAPTOPS);
        let mut attributes = Attributes::new();
        attributes.insert(
            attribute_ids::ASPECTS,
            AttributeValue::StringValueMap(
                [
                    (
                        "Max Screen Resolution".to_owned(),
                        AttributeValue::String("1920x1080".to_owned()),
                    ),
                    (
                        "RAM Size".to_owned(),
                        AttributeValue::String("8 GB".to_owned()),
                    ),
                    (
                        "Operating System".to_owned(),
                        AttributeValue::String("Chrome OS".to_owned()),
                    ),
                    (
                        "Screen Size".to_owned(),
                        AttributeValue::String("15.6 Inches".to_owned()),
                    ),
                    (
                        "Brand".to_owned(),
                        AttributeValue::String("Lenovo".to_owned()),
                    ),
                    (
                        "SSD Capacity".to_owned(),
                        AttributeValue::String("2 TB".to_owned()),
                    ),
                ]
                .into(),
            ),
        );
        let attributes = parser.parse(&attributes);
        assert_eq!(
            attributes.get_screen_resolution(&attribute_ids::SCREEN_RESOLUTION),
            Some(&ScreenResolution::new(1920, 1080))
        );
        assert_eq!(attributes.get_u64(&attribute_ids::RAM_SIZE), Some(8));
        assert_eq!(
            attributes.get_str(&attribute_ids::OPERATING_SYSTEM),
            Some("Chrome OS")
        );
        assert_eq!(attributes.get_f64(&attribute_ids::SCREEN_SIZE), Some(15.6));
        assert_eq!(attributes.get_str(&attribute_ids::BRAND), Some("Lenovo"));
        assert_eq!(attributes.get_u64(&attribute_ids::SSD_SIZE), Some(2000));
        assert_eq!(attributes.get_str(&attribute_ids::SPAM), Some("Not Spam"));
    }

    #[test]
    fn test_parse_2() {
        let parser = Parser::new(&category_ids::LAPTOPS);
        let mut attributes = Attributes::new();
        attributes.insert(
            attribute_ids::ASPECTS,
            AttributeValue::StringValueMap(
                [(
                    "Operating System".to_owned(),
                    AttributeValue::String(
                        "Windows 10 Pro Device comes with Windows 10 and a free Windows 1"
                            .to_owned(),
                    ),
                )]
                .into(),
            ),
        );
        let attributes = parser.parse(&attributes);
        assert_eq!(
            attributes.get_str(&attribute_ids::OPERATING_SYSTEM),
            Some("Windows 10 Pro")
        );
    }

    #[test]
    fn test_parse_3() {
        let parser = Parser::new(&category_ids::LAPTOPS);
        let mut attributes = Attributes::new();
        attributes.insert(
            attribute_ids::TITLE,
            AttributeValue::String("8 GB RAM".to_owned()),
        );
        attributes.insert(
            attribute_ids::ASPECTS,
            AttributeValue::StringValueMap(
                [
                    (
                        "RAM Size".to_owned(),
                        AttributeValue::String("512GB".to_owned()),
                    ),
                    (
                        "SSD Capacity".to_owned(),
                        AttributeValue::String("512 GB".to_owned()),
                    ),
                ]
                .into(),
            ),
        );
        let attributes = parser.parse(&attributes);
        assert_eq!(attributes.get_u64(&attribute_ids::SSD_SIZE), Some(512));
        assert_eq!(attributes.get_u64(&attribute_ids::RAM_SIZE), Some(8));
    }

    #[test]
    fn test_parse_4() {
        let mut parser = Parser::new(&category_ids::LAPTOPS);

        let cpu_id = Uuid::parse_str("773ba684-053c-4a2d-980b-03047cbfdcd6").unwrap();
        let gpu_id = Uuid::parse_str("6929ab39-3d68-4d83-8a99-4b8927bb1a0c").unwrap();

        let mut cpu_grammar = HashMap::new();
        cpu_grammar.insert(cpu_id, Grammar::new("AMD Ryzen 9 5900HX Processor"));
        let cpu_parser = ProductParser::init(&cpu_grammar);
        parser.cpu_parser = Some(cpu_parser);

        let mut gpu_grammar = HashMap::new();
        gpu_grammar.insert(gpu_id, Grammar::new("GeForce RTX 3080"));
        let gpu_parser = ProductParser::init(&gpu_grammar);
        parser.gpu_parser = Some(gpu_parser);

        let mut attributes = Attributes::new();
        attributes.insert(
            attribute_ids::TITLE,
            AttributeValue::String("AMD Ryzen 9 5900HX Processor GeForce RTX 3080".to_owned()),
        );
        let attributes = parser.parse(&attributes);
        assert_eq!(attributes.get_uuid(&attribute_ids::CPU_ID), Some(cpu_id));
        assert_eq!(attributes.get_uuid(&attribute_ids::GPU_ID), Some(gpu_id));
    }

    #[test]
    fn test_parse_5() {
        let parser = Parser::new(&category_ids::LAPTOPS);
        let mut attributes = Attributes::new();
        attributes.insert(
            attribute_ids::ASPECTS,
            AttributeValue::StringValueMap(
                [(
                    "Screen Size".to_owned(),
                    AttributeValue::String("27 Inches".to_owned()),
                )]
                .into(),
            ),
        );
        let attributes = parser.parse(&attributes);
        assert_eq!(attributes.get_f64(&attribute_ids::SCREEN_SIZE), Some(27.0));
        assert_eq!(attributes.get_str(&attribute_ids::SPAM), Some("Spam"));
    }

    #[test]
    fn test_parse_6() {
        let parser = Parser::new(&category_ids::LAPTOPS);
        let mut attributes = Attributes::new();
        attributes.insert(
            attribute_ids::TITLE,
            AttributeValue::String("Keyboard Cover for Lenovo".to_owned()),
        );
        let attributes = parser.parse(&attributes);
        assert_eq!(attributes.get_str(&attribute_ids::SPAM), Some("Spam"));
    }

    #[test]
    fn test_parse_7() {
        let parser = Parser::new(&category_ids::INTERNAL_SOLID_STATE_DRIVES);
        let mut attributes = Attributes::new();
        attributes.insert(
            attribute_ids::SOURCE_CATEGORY,
            // Solid State Drives
            AttributeValue::StringSet(vec!["175669".to_owned()].into_iter().collect()),
        );
        attributes.insert(
            attribute_ids::TITLE,
            AttributeValue::String(
                "7.68TB Intel SSD D5-P4420 Series DC NVME U.2 2.5\" SSDPENU076T8 Solid State Drive"
                    .to_owned(),
            ),
        );
        let attributes = parser.parse(&attributes);
        assert_eq!(
            attributes.get_u64(&attribute_ids::STORAGE_CAPACITY),
            Some(7680000000000)
        );
        assert_eq!(attributes.get_str(&attribute_ids::SPAM), Some("Not Spam"));
    }
}
