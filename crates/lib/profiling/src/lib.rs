use dashmap::DashMap;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tracing::{error, Id, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

lazy_static! {
    static ref RESULTS_MAP: DashMap<String, Vec<SingleProfileResult>> = DashMap::new();
    static ref RUNNING_PROFILERS: Arc<RwLock<Vec<u64>>> = Arc::new(RwLock::new(Vec::new()));
}

#[derive(Clone)]
struct SingleProfileResult {
    pub duration: Duration,
    pub keys: Vec<u64>,
}

pub struct FinalProfileResult {
    pub name: String,
    pub total_time: u128,
    pub count: u128,
    pub average_time: u128,
    pub nodes: Vec<FinalProfileResult>,
}

fn generate_final_result(kv: HashMap<String, Vec<SingleProfileResult>>) -> Vec<FinalProfileResult> {
    todo!()
}

pub async fn start_profiler() -> u64 {
    let key = rand::random();
    let mut profilers = RUNNING_PROFILERS.write();
    profilers.push(key);
    key
}

pub async fn stop_profiling(key: u64) {
    let mut raw_results = HashMap::new();
    RESULTS_MAP
        .iter()
        .filter(|x| x.key().contains(&key.to_string()))
        .for_each(|x| {
            raw_results.insert(x.key().clone(), x.value().clone());
        });
    for (name, results) in raw_results {
        let total_time = results.iter().map(|x| x.duration.as_micros()).sum();
        let count = results.len() as u128;
        let average_time = total_time / count;
        let nodes = Vec::new();
        let final_result = FinalProfileResult {
            name: name.clone(),
            total_time,
            count,
            average_time,
            nodes,
        };
    }
}

#[derive(Default)]
pub struct ProfilerTracingLayer;

impl<S> Layer<S> for ProfilerTracingLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        if RUNNING_PROFILERS.read().len() >= 1 {
            match ctx.span(id) {
                None => {
                    error!("No span found")
                }
                Some(span) => {
                    if span.name().starts_with("profiler/") {
                        span.extensions_mut().insert(Instant::now());
                    }
                }
            }
        }
    }
    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
        if RUNNING_PROFILERS.read().len() >= 1 {
            let instant = match ctx.span(id) {
                None => {
                    error!("No span found");
                    None
                }
                Some(span) => {
                    if span.name().starts_with("profiler/") {
                        let start = span.extensions().get::<Instant>().cloned();
                        span.extensions_mut().remove::<Instant>();
                        start
                    } else {
                        None
                    }
                }
            };

            if let Some(start) = instant {
                let elapsed = start.elapsed();
                let name = ctx.span(id).unwrap().name().to_string();
                thread::spawn(move || {
                    let keys = RUNNING_PROFILERS.read().iter().map(|x| *x).collect();
                    let result = SingleProfileResult {
                        duration: elapsed,
                        keys,
                    };
                    RESULTS_MAP
                        .entry(name)
                        .or_insert_with(Vec::new)
                        .push(result);
                });
            }
        }
    }
}
