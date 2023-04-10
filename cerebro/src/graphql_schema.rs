use async_graphql::{Context, Object, SimpleObject};
use bonfire_core::attribute_value::CpuGroup;
use bonfire_core::attributes::Attributes;
use bonfire_core::grammar::Grammar;
use bonfire_core::offers::Offers;
use bonfire_ids::attribute_ids;
use hashbrown::HashMap;
use hashbrown::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct Cpu {
    pub id: Uuid,
    pub attributes: Attributes,
    pub title: String,
    pub num_cores: Option<u64>,
    pub clock_speed: Option<u64>,
    pub clock_speed_max: Option<u64>,
    pub created: Option<u64>,
    pub benchmarks: HashMap<Uuid, f64>,
    pub synth: HashMap<Uuid, f64>,
    pub single_core_constant: f64,
    pub group: Option<CpuGroup>,
    pub cache: Option<u64>,
    pub tdp: Option<f64>,
    pub tdp_up: Option<f64>,
    pub tdp_down: Option<f64>,
    pub base_power: Option<f64>,
    pub max_turbo_power: Option<f64>,
}

impl Cpu {
    pub fn new(product: &Product) -> Self {
        let mut benchmarks = HashMap::new();
        if let Some(passmark) = product.attributes.get_u64(&attribute_ids::CPU_PASSMARK) {
            benchmarks.insert(attribute_ids::CPU_PASSMARK, passmark as f64);
        }
        if let Some(passmark) = product
            .attributes
            .get_u64(&attribute_ids::CPU_PASSMARK_SINGLE_THREAD)
        {
            benchmarks.insert(attribute_ids::CPU_PASSMARK_SINGLE_THREAD, passmark as f64);
        }
        Self {
            id: product.id,
            attributes: product.attributes.clone(),
            title: product
                .attributes
                .get_str(&attribute_ids::TITLE)
                .unwrap()
                .to_owned(),
            num_cores: product.attributes.get_u64(&attribute_ids::CPU_NUM_CORES),
            clock_speed: product.attributes.get_u64(&attribute_ids::CPU_CLOCK_SPEED),
            clock_speed_max: product
                .attributes
                .get_u64(&attribute_ids::CPU_CLOCK_SPEED_MAX),
            created: product
                .attributes
                .get_datetime(&attribute_ids::CREATED)
                .map(|v| v.timestamp() as u64),
            benchmarks,
            synth: HashMap::new(),
            single_core_constant: 0.0,
            group: product.attributes.get_cpu_group(&attribute_ids::CPU_GROUP),
            cache: product.attributes.get_u64(&attribute_ids::CPU_CACHE),
            tdp: product.attributes.get_f64(&attribute_ids::CPU_TDP),
            tdp_up: product.attributes.get_f64(&attribute_ids::CPU_TDP_UP),
            tdp_down: product.attributes.get_f64(&attribute_ids::CPU_TDP_DOWN),
            base_power: product.attributes.get_f64(&attribute_ids::CPU_BASE_POWER),
            max_turbo_power: product
                .attributes
                .get_f64(&attribute_ids::CPU_MAX_TURBO_POWER),
        }
    }

    pub fn max_tdp(&self) -> Option<f64> {
        match self.max_turbo_power {
            Some(max_turbo_power) => Some(max_turbo_power),
            _ => match self.tdp_up {
                Some(tdp_up) => Some(tdp_up),
                _ => self.tdp,
            },
        }
    }
}

#[Object]
impl Cpu {
    async fn id(&self, _ctx: &Context<'_>) -> &Uuid {
        &self.id
    }

    async fn attributes(&self, _ctx: &Context<'_>) -> &Attributes {
        &self.attributes
    }

    async fn title(&self, _ctx: &Context<'_>) -> &str {
        &self.title
    }

    async fn num_cores(&self, _ctx: &Context<'_>) -> &Option<u64> {
        &self.num_cores
    }

    async fn clock_speed(&self, _ctx: &Context<'_>) -> &Option<u64> {
        &self.clock_speed
    }

    async fn clock_speed_max(&self, _ctx: &Context<'_>) -> &Option<u64> {
        &self.clock_speed_max
    }

    async fn tdp(&self, _ctx: &Context<'_>) -> &Option<f64> {
        &self.tdp
    }

    async fn tdp_up(&self, _ctx: &Context<'_>) -> &Option<f64> {
        &self.tdp_up
    }

    async fn tdp_down(&self, _ctx: &Context<'_>) -> &Option<f64> {
        &self.tdp_down
    }

    async fn base_power(&self, _ctx: &Context<'_>) -> &Option<f64> {
        &self.base_power
    }

    async fn max_turbo_power(&self, _ctx: &Context<'_>) -> &Option<f64> {
        &self.max_turbo_power
    }

    async fn created(&self, _ctx: &Context<'_>) -> &Option<u64> {
        &self.created
    }

    async fn passmark(&self, _ctx: &Context<'_>) -> Option<&f64> {
        self.benchmarks.get(&attribute_ids::CPU_PASSMARK)
    }

    async fn passmark_single_thread(&self, _ctx: &Context<'_>) -> Option<&f64> {
        self.benchmarks
            .get(&attribute_ids::CPU_PASSMARK_SINGLE_THREAD)
    }

    async fn synth_passmark_single_thread(&self, _ctx: &Context<'_>) -> Option<&f64> {
        self.synth.get(&attribute_ids::CPU_PASSMARK_SINGLE_THREAD)
    }

    async fn synth_passmark(&self, _ctx: &Context<'_>) -> Option<&f64> {
        self.synth.get(&attribute_ids::CPU_PASSMARK)
    }

    async fn url(&self, _ctx: &Context<'_>) -> Option<&str> {
        self.attributes.get_str(&attribute_ids::SOURCE_URL)
    }
}

#[derive(SimpleObject)]
pub struct Points {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

#[derive(Clone)]
pub struct Product {
    pub id: Uuid,
    pub categories: HashSet<Uuid>,
    pub attributes: Attributes,
    pub grammar: Option<Grammar>,
    pub offers: Offers,
}

impl Product {
    pub fn new(id: Uuid, categories: HashSet<Uuid>, attributes: Attributes) -> Product {
        Self {
            id,
            categories,
            attributes,
            grammar: None,
            offers: Offers::new(),
        }
    }
}

#[Object]
impl Product {
    async fn id(&self, _ctx: &Context<'_>) -> &Uuid {
        &self.id
    }

    async fn category_ids(&self, _ctx: &Context<'_>) -> Vec<Uuid> {
        self.categories.iter().cloned().collect()
    }

    async fn attributes(&self, _ctx: &Context<'_>) -> &Attributes {
        &self.attributes
    }

    async fn grammar(&self, _ctx: &Context<'_>) -> Option<String> {
        self.grammar
            .as_ref()
            .map(|grammar| serde_json::to_string(grammar).unwrap())
    }

    async fn offers(&self, _ctx: &Context<'_>) -> &Offers {
        &self.offers
    }
}

pub struct SynthBenchResult {
    pub cpus: Vec<Cpu>,
}

#[Object]
impl SynthBenchResult {
    async fn cpus(&self, _ctx: &Context<'_>) -> &Vec<Cpu> {
        &self.cpus
    }

    async fn synth_passmark_single_thread_mse(&self, _ctx: &Context<'_>) -> Option<f64> {
        Some(
            self.cpus
                .iter()
                .map(|cpu| {
                    let diff = cpu
                        .benchmarks
                        .get(&attribute_ids::CPU_PASSMARK_SINGLE_THREAD)
                        .unwrap()
                        - cpu
                            .synth
                            .get(&attribute_ids::CPU_PASSMARK_SINGLE_THREAD)
                            .unwrap();
                    (diff * diff) / self.cpus.len() as f64
                })
                .sum(),
        )
    }
}

pub struct SynthBenchData {
    pub products: HashMap<Uuid, Product>,
    pub cpus: HashMap<Uuid, Cpu>,
}

pub type Storage = Arc<RwLock<SynthBenchData>>;

impl SynthBenchData {
    pub async fn new() -> Self {
        Self {
            products: HashMap::new(),
            cpus: HashMap::new(),
        }
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn products(&self, ctx: &Context<'_>) -> Vec<Product> {
        let data = ctx.data_unchecked::<Storage>().read().await;
        data.products
            .values()
            .filter(|cpu| {
                matches!(
                    cpu.attributes.get_cpu_group(&attribute_ids::CPU_GROUP),
                    Some(CpuGroup::IntelCoreIx(_ix))
                )
            })
            .cloned()
            .collect()
    }

    async fn product(&self, ctx: &Context<'_>, id: Uuid) -> Option<Product> {
        let data = ctx.data_unchecked::<Storage>().read().await;
        data.products.get(&id).cloned()
    }

    async fn synth_bench(&self, ctx: &Context<'_>) -> SynthBenchResult {
        let data = ctx.data_unchecked::<Storage>().read().await;
        let cpus: Vec<Cpu> = data
            .cpus
            .values()
            .filter(|cpu| {
                cpu.benchmarks
                    .get(&attribute_ids::CPU_PASSMARK_SINGLE_THREAD)
                    .is_some()
                    && cpu
                        .synth
                        .get(&attribute_ids::CPU_PASSMARK_SINGLE_THREAD)
                        .is_some()
            })
            .cloned()
            .collect();
        SynthBenchResult { cpus }
    }

    async fn points(&self, ctx: &Context<'_>) -> Points {
        let data = ctx.data_unchecked::<Storage>().read().await;
        let cpus: Vec<Cpu> = data
            .cpus
            .values()
            .filter(|cpu| {
                cpu.benchmarks
                    .get(&attribute_ids::CPU_PASSMARK_SINGLE_THREAD)
                    .is_some()
                    && cpu.created.is_some()
            })
            .cloned()
            .collect();
        Points {
            x: cpus.iter().map(|cpu| cpu.created.unwrap() as f64).collect(),
            y: cpus
                .iter()
                .map(|cpu| {
                    *cpu.benchmarks
                        .get(&attribute_ids::CPU_PASSMARK_SINGLE_THREAD)
                        .unwrap()
                })
                .collect(),
        }
    }
}
