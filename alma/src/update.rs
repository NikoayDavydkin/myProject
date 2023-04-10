use anyhow::Result;
use bonfire_core::grammar::Grammar;
use bonfire_ids::category_ids;
use hashbrown::HashMap;
use product_parser::ProductParser;
use tokio::time::{sleep, Duration};

use crate::bonfire_client::get_grammars;
use crate::graphql_schema::Storage;

pub fn parse_grammar(grammar_string: &str) -> Result<Grammar> {
    Ok(serde_json::from_str(grammar_string)?)
}

pub async fn update(data: Storage) -> ! {
    loop {
        match get_grammars(category_ids::LAPTOP_CPUS).await {
            Ok(products) => {
                let mut grammars = HashMap::new();
                for product in products {
                    if let Some(grammar_string) = product.grammar {
                        match parse_grammar(&grammar_string) {
                            Ok(grammar) => {
                                grammars.insert(product.id, grammar);
                            }
                            Err(error) => {
                                println!("Could not parse grammar: {}", error);
                            }
                        }
                    }
                }
                let cpu_parser = ProductParser::init(&grammars);
                data.write().await.parser.set_cpu_parser(cpu_parser);
            }
            Err(error) => {
                println!("Could not get grammars: {}", error);
            }
        }
        match get_grammars(category_ids::LAPTOP_GPUS).await {
            Ok(products) => {
                let mut grammars = HashMap::new();
                for product in products {
                    if let Some(grammar_string) = product.grammar {
                        match parse_grammar(&grammar_string) {
                            Ok(grammar) => {
                                grammars.insert(product.id, grammar);
                            }
                            Err(error) => {
                                println!("Could not parse grammar: {}", error);
                            }
                        }
                    }
                }
                let gpu_parser = ProductParser::init(&grammars);
                data.write().await.parser.set_gpu_parser(gpu_parser);
            }
            Err(error) => {
                println!("Could not get grammars: {}", error);
            }
        }
        sleep(Duration::from_secs(3600)).await;
    }
}
