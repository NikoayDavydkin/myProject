use gloo::timers::callback::Interval;
use gloo_console::error;
//use gloo_console::info;
use hashbrown::HashMap;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use yew::html::Scope;
use yew::prelude::*;

use crate::components::graph::Graph;
use crate::cpu::Cpu;
use crate::ids::attribute_ids;
use crate::serde::synth_bench::SynthBench;
use crate::synth_bench_client;

pub enum Msg {
    UpdateSynthBench(SynthBench),
    Tick,
    CpuSelected(Option<Uuid>),
    CpuHover(Option<Uuid>),
    XAxisChanged(GraphAxis),
    YAxisChanged(GraphAxis),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GraphAxis {
    PassMarkSingleThread,
    SynthPassMarkSingleThread,
    SynthPassMarkDiff,
    SynthPassMarkRatio,
    MaxTdp,
}

impl std::str::FromStr for GraphAxis {
    type Err = ();

    fn from_str(s: &str) -> Result<GraphAxis, ()> {
        match s {
            "PassMarkSingleThread" => Ok(GraphAxis::PassMarkSingleThread),
            "SynthPassMarkSingleThread" => Ok(GraphAxis::SynthPassMarkSingleThread),
            "SynthPassMarkDiff" => Ok(GraphAxis::SynthPassMarkDiff),
            "SynthPassMarkRatio" => Ok(GraphAxis::SynthPassMarkRatio),
            "MaxTdp" => Ok(GraphAxis::MaxTdp),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for GraphAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn axis_value(axis: &GraphAxis, cpu: &Cpu) -> Option<f64> {
    match axis {
        GraphAxis::PassMarkSingleThread => Some(cpu.passmark_single_thread),
        GraphAxis::SynthPassMarkSingleThread => Some(cpu.synth_passmark_single_thread),
        GraphAxis::SynthPassMarkDiff => {
            Some(cpu.synth_passmark_single_thread - cpu.passmark_single_thread)
        }
        GraphAxis::SynthPassMarkRatio => {
            Some((cpu.synth_passmark_single_thread / cpu.passmark_single_thread).log2())
        }
        GraphAxis::MaxTdp => match cpu.attributes.get_f64(&attribute_ids::CPU_TDP_UP) {
            Some(tdp) => Some(tdp),
            _ => cpu.attributes.get_f64(&attribute_ids::CPU_TDP),
        },
    }
}

pub fn cpu_graph_minmax(axis: &GraphAxis, cpus: &HashMap<Uuid, Cpu>) -> (f64, f64) {
    let mut min = cpus
        .values()
        .filter_map(|cpu| axis_value(axis, cpu))
        .fold(f64::INFINITY, f64::min);
    let mut max = cpus
        .values()
        .filter_map(|cpu| axis_value(axis, cpu))
        .fold(f64::NEG_INFINITY, f64::max);
    match axis {
        GraphAxis::SynthPassMarkDiff | GraphAxis::SynthPassMarkRatio => {
            min = f64::min(min, -max);
            max = f64::max(max, -min);
        }
        _ => {}
    }
    (min, max)
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct Page {
    pub cpus: HashMap<Uuid, Cpu>,
    pub mse: f64,
    pub interval: Option<Interval>,
    pub cpu_select: Option<Uuid>,
    pub cpu_hover: Option<Uuid>,
    pub x_axis: GraphAxis,
    pub y_axis: GraphAxis,
}

impl Page {
    async fn fetch_worker(link: Scope<Page>) {
        match synth_bench_client::get_cpus().await {
            Ok(synth) => {
                link.send_message(Msg::UpdateSynthBench(synth));
            }
            Err(error) => {
                error!(std::format!("Could not fetch: {}", error));
                link.send_message(Msg::UpdateSynthBench(SynthBench {
                    cpus: Vec::new(),
                    synth_passmark_single_thread_mse: 0.0,
                }));
            }
        }
    }
}

impl Component for Page {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let interval_handle = {
            let link = ctx.link().clone();
            Some(Interval::new(10, move || link.send_message(Msg::Tick)))
        };
        Self {
            cpus: HashMap::new(),
            mse: 0.0,
            interval: interval_handle,
            cpu_select: None,
            cpu_hover: None,
            x_axis: GraphAxis::SynthPassMarkSingleThread,
            y_axis: GraphAxis::PassMarkSingleThread,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateSynthBench(synth) => {
                self.cpus = synth
                    .cpus
                    .into_iter()
                    .map(|cpu| {
                        (
                            cpu.id,
                            Cpu {
                                id: cpu.id,
                                title: cpu.title,
                                passmark_single_thread: cpu.passmark_single_thread,
                                synth_passmark_single_thread: cpu.synth_passmark_single_thread,
                                attributes: Some(cpu.attributes.attributes).into(),
                            },
                        )
                    })
                    .collect();
                self.mse = synth.synth_passmark_single_thread_mse;
                true
            }
            Msg::Tick => {
                let link = ctx.link().clone();
                spawn_local(Page::fetch_worker(link));
                false
            }
            Msg::CpuSelected(id) => {
                self.cpu_select = id;
                true
            }
            Msg::CpuHover(id) => {
                self.cpu_hover = id;
                true
            }
            Msg::XAxisChanged(axis) => {
                self.x_axis = axis;
                true
            }
            Msg::YAxisChanged(axis) => {
                self.y_axis = axis;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut points: Vec<(f64, f64, Uuid)> = self
            .cpus
            .values()
            .filter_map(|cpu| {
                Some((
                    axis_value(&self.x_axis, cpu)?,
                    axis_value(&self.y_axis, cpu)?,
                    cpu.id,
                ))
            })
            .collect();
        points.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let (mut min_x, mut max_x) = cpu_graph_minmax(&self.x_axis, &self.cpus);
        let (mut min_y, mut max_y) = cpu_graph_minmax(&self.y_axis, &self.cpus);
        let square = match self.x_axis {
            GraphAxis::PassMarkSingleThread | GraphAxis::SynthPassMarkSingleThread => {
                matches!(
                    self.y_axis,
                    GraphAxis::PassMarkSingleThread | GraphAxis::SynthPassMarkSingleThread
                )
            }
            _ => false,
        };
        if square {
            min_x = f64::min(min_x, min_y);
            max_x = f64::max(max_x, max_y);
            min_y = f64::min(min_x, min_y);
            max_y = f64::max(max_x, max_y);
        }
        let selected = match self.cpu_hover {
            Some(id) => Some(id),
            _ => self.cpu_select,
        };
        let graph_axis_options = vec![
            GraphAxis::PassMarkSingleThread,
            GraphAxis::SynthPassMarkSingleThread,
            GraphAxis::SynthPassMarkDiff,
            GraphAxis::SynthPassMarkRatio,
            GraphAxis::MaxTdp,
        ];
        html! {
            <div class={classes!("page")}>
                <div class="left">
                    <label>{std::format!("Num Cpus: {}", self.cpus.len())}</label>
                    <div class="graph-container">
                        <Graph
                            points={points}
                            selected={selected}
                            min_x={min_x}
                            max_x={max_x}
                            min_y={min_y}
                            max_y={max_y}
                            on_changed={ctx.link().callback(Msg::CpuSelected)}
                            on_hover={ctx.link().callback(Msg::CpuHover)}
                        />
                    </div>
                    <div class="select-container">
                        <label>{std::format!("MSE: {}", self.mse)}</label>
                        <select
                            onchange={ctx.link().callback(|event: Event| {
                                let value = event.target().unwrap().dyn_into::<web_sys::HtmlSelectElement>().unwrap().value();
                                Msg::XAxisChanged(value.parse::<GraphAxis>().unwrap())
                            })}
                        >
                            {
                                graph_axis_options.iter().map(|option| html! {
                                    <option selected={self.x_axis == *option}>{option.to_string()}</option>
                                }).collect::<Vec<Html>>()
                            }
                        </select>
                        <select
                            onchange={ctx.link().callback(|event: Event| {
                                let value = event.target().unwrap().dyn_into::<web_sys::HtmlSelectElement>().unwrap().value();
                                Msg::YAxisChanged(value.parse::<GraphAxis>().unwrap())
                            })}
                        >
                            {
                                graph_axis_options.iter().map(|option| html! {
                                    <option selected={self.y_axis == *option}>{option.to_string()}</option>
                                }).collect::<Vec<Html>>()
                            }
                        </select>
                    </div>
                </div>
                <div class="right">
                    if let Some(selected) = selected {
                        if let Some(cpu) = self.cpus.get(&selected) {
                            <div class="cpu-specs">
                                <label>{cpu.id}</label>
                                <label>{cpu.title.clone()}</label>
                                if let Some(passmark_url) = cpu.attributes.get_str(&attribute_ids::PASSMARK_URL) {
                                    <label><a href={passmark_url.to_owned()} target="_blank">{std::format!("Passmark: {}", cpu.passmark_single_thread)}</a></label>
                                }
                                <label>{std::format!("Synth: {}", cpu.synth_passmark_single_thread)}</label>
                                if let Some(speed) = cpu.attributes.get_u64(&attribute_ids::CPU_CLOCK_SPEED_MAX) {
                                    <label>{std::format!("Clock Clock Speed Max: {} GHz", speed as f64 / 1000.0)}</label>
                                }
                                if let Some(cache) = cpu.attributes.get_u64(&attribute_ids::CPU_CACHE) {
                                    <label>{std::format!("Cache: {}", cache)}</label>
                                }
                                if let Some(num_cores) = cpu.attributes.get_u64(&attribute_ids::CPU_NUM_CORES) {
                                    <label>{std::format!("Num Cores: {}", num_cores)}</label>
                                }
                                if let Some(tdp) = cpu.attributes.get_f64(&attribute_ids::CPU_TDP) {
                                    <label>{std::format!("TDP: {}", tdp)}</label>
                                }
                                if let Some(tdp) = cpu.attributes.get_f64(&attribute_ids::CPU_TDP_UP) {
                                    <label>{std::format!("TDP-up: {}", tdp)}</label>
                                }
                                if let Some(tdp) = cpu.attributes.get_f64(&attribute_ids::CPU_TDP_DOWN) {
                                    <label>{std::format!("TDP-down: {}", tdp)}</label>
                                }
                                <label>
                                    {std::format!("SynthPassMarkRatio: {}",
                                        cpu.synth_passmark_single_thread / cpu.passmark_single_thread)
                                    }
                                </label>
                            </div>
                        }
                    }
                </div>
            </div>
        }
    }
}
