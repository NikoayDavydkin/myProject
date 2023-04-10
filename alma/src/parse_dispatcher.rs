use bonfire_core::attributes::Attributes;
use bonfire_ids::attribute_ids;
use bonfire_ids::category_ids;
use hashbrown::HashSet;
use product_parser::ProductParser;
use uuid::Uuid;

use crate::parse::Parser;

pub struct ParseDispatcher {
    pub laptop_parser: Parser,
    pub storage_parser: Parser,
}

impl ParseDispatcher {
    pub fn new() -> Self {
        Self {
            laptop_parser: Parser::new(&category_ids::LAPTOPS),
            storage_parser: Parser::new(&category_ids::INTERNAL_SOLID_STATE_DRIVES),
        }
    }

    pub fn set_cpu_parser(&mut self, parser: ProductParser) {
        self.laptop_parser.cpu_parser = Some(parser);
    }

    pub fn set_gpu_parser(&mut self, parser: ProductParser) {
        self.laptop_parser.gpu_parser = Some(parser);
    }

    pub fn parse(
        &self,
        category_ids: &HashSet<Uuid>,
        attributes: &Attributes,
    ) -> (HashSet<Uuid>, Attributes) {
        let new_categories = ParseDispatcher::parse_categories(attributes);
        let category_ids = if !new_categories.is_empty() {
            new_categories
        } else {
            category_ids.clone()
        };
        let mut attributes = attributes.clone();
        for category in &category_ids {
            let new_attributes = match *category {
                category_ids::LAPTOPS => self.laptop_parser.parse(&attributes),
                category_ids::INTERNAL_SOLID_STATE_DRIVES => self.storage_parser.parse(&attributes),
                category_ids::INTERNAL_HARD_DRIVES => self.storage_parser.parse(&attributes),
                category_ids::EXTERNAL_SOLID_STATE_DRIVES => self.storage_parser.parse(&attributes),
                category_ids::EXTERNAL_HARD_DRIVES => self.storage_parser.parse(&attributes),
                _ => Attributes::new(),
            };
            attributes.attributes.extend(new_attributes.attributes);
        }
        (category_ids, attributes)
    }

    pub fn parse_categories(attributes: &Attributes) -> HashSet<Uuid> {
        let mut return_value = HashSet::new();
        if let Some(categories) = attributes.get_string_set(&attribute_ids::SOURCE_CATEGORY) {
            for category_id in categories {
                match category_id.as_str() {
                    "111422" => {
                        // Apple Laptops
                        return_value.insert(category_ids::LAPTOPS);
                    }
                    "177" => {
                        // PC Laptops and Netbooks
                        return_value.insert(category_ids::LAPTOPS);
                    }
                    "56083" => {
                        // Internal Hard Disk Drives
                        return_value.insert(category_ids::INTERNAL_HARD_DRIVES);
                    }
                    "131553" => {
                        // External Hard Disk Drives
                        return_value.insert(category_ids::EXTERNAL_HARD_DRIVES);
                    }
                    "175669" => {
                        // Solid State Drives
                        return_value.insert(category_ids::INTERNAL_SOLID_STATE_DRIVES);
                    }
                    _ => {
                        println!("Error: category {} not found", category_id);
                    }
                }
            }
        }

        return_value
    }
}

#[cfg(test)]
mod tests {
    use bonfire_core::attribute_value::AttributeValue;
    use bonfire_core::attribute_value::ScreenResolution;

    use super::*;

    #[test]
    fn test_parse_category_1() {
        let mut attributes = Attributes::new();
        attributes.insert(
            attribute_ids::SOURCE_CATEGORY,
            AttributeValue::StringSet(vec!["177".to_owned()].into_iter().collect()),
        );
        let categories = ParseDispatcher::parse_categories(&attributes);
        assert_eq!(
            categories,
            vec![category_ids::LAPTOPS].into_iter().collect()
        );
    }

    #[test]
    fn test_parse_1() {
        let parser = ParseDispatcher::new();
        let categories = vec![category_ids::LAPTOPS].into_iter().collect();
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
        let (_category_ids, attributes) = parser.parse(&categories, &attributes);
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
        let parser = ParseDispatcher::new();
        let category_ids = HashSet::new();
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
        let (category_ids, attributes) = parser.parse(&category_ids, &attributes);
        assert_eq!(
            category_ids,
            vec![category_ids::INTERNAL_SOLID_STATE_DRIVES]
                .into_iter()
                .collect()
        );
        assert_eq!(
            attributes.get_u64(&attribute_ids::STORAGE_CAPACITY),
            Some(7680000000000)
        );
    }
}
