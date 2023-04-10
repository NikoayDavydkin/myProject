use bonfire_core::attribute_value::AttributeValue;
use bonfire_ids::attribute_ids;
use bonfire_ids::category_ids;
use hashbrown::HashMap;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

use crate::bonfire_client::search;
use crate::graphql_schema::Product;
use crate::graphql_schema::Storage;
use crate::synth::calculate_synthbench;

pub async fn update(data: Storage) -> ! {
    loop {
        match search(category_ids::CPUS).await {
            Ok(products) => {
                let cpus = calculate_synthbench(&products);
                let mut products: HashMap<Uuid, Product> =
                    products.into_iter().map(|p| (p.id, p)).collect();
                match cpus {
                    Ok(cpus) => {
                        for cpu in &cpus {
                            products.get_mut(&cpu.id).unwrap().attributes.insert(
                                attribute_ids::SYNTH_BENCH,
                                AttributeValue::UuidF64Map(cpu.synth.clone()),
                            );
                        }
                        data.write().await.cpus = cpus.into_iter().map(|c| (c.id, c)).collect();
                    }
                    Err(error) => {
                        println!("Calculate SynthBench failed: {}", error);
                    }
                }
                data.write().await.products = products;
                println!("Products loaded");
            }
            Err(error) => {
                println!("Error: Could not get Cpus: {}", error);
            }
        }
        sleep(Duration::from_secs(10)).await;
    }
}
