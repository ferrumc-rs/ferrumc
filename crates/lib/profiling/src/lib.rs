use dashmap::DashMap;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use serde_derive::Serialize;
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

#[derive(Clone, Serialize)]
struct SingleProfileResult {
    pub duration: Duration,
    pub keys: Vec<u64>,
}

#[derive(Serialize)]
pub struct FinalProfileResult {
    pub name: String,
    pub total_time: u128,
    pub count: u128,
    pub average_time: u128,
    pub nodes: Vec<FinalProfileResult>,
}

fn generate_final_result(kv: HashMap<String, Vec<SingleProfileResult>>) -> Vec<FinalProfileResult> {
    // Collect all root nodes
    let mut root_nodes = kv
        .keys()
        .collect::<Vec<&String>>()
        .iter()
        .filter(|x| x.contains("/"))
        .map(|x| x.split("/").collect::<Vec<&str>>()[0])
        .collect::<Vec<&str>>();
    root_nodes.dedup();
    // All the nodes that don't have children
    let end_nodes = kv
        .keys()
        .collect::<Vec<&String>>()
        .iter()
        .filter(|x| !x.contains("/"))
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();
    let mut final_results = end_nodes
        .iter()
        .map(|x| {
            let total_time = kv
                .get(*x)
                .unwrap()
                .iter()
                .map(|x| x.duration.as_micros())
                .sum();
            let count = kv.get(*x).unwrap().len() as u128;
            let average_time = total_time / count;
            let nodes = Vec::new();
            FinalProfileResult {
                name: x.to_string(),
                total_time,
                count,
                average_time,
                nodes,
            }
        })
        .collect::<Vec<FinalProfileResult>>();
    // Using recursion to build the tree from all the nodes that have children
    // Basically we just cut off the first part of the string and use it as the parent node
    root_nodes.iter().for_each(|x| {
        let mut map = HashMap::new();

        kv.iter().for_each(|(k, v)| {
            if k.starts_with(x) {
                let new_key = k.split_once("/").unwrap().1.to_string();
                map.insert(new_key, v.clone());
            }
        });
        let children = generate_final_result(map);

        let mut total_time = children.iter().map(|x| x.total_time).sum();
        if kv.contains_key(&x.to_string()) {
            total_time += kv
                .get(&x.to_string())
                .unwrap()
                .iter()
                .map(|x| x.duration.as_micros())
                .sum::<u128>();
        }
        let mut count = children.iter().map(|x| x.count).sum();
        if kv.contains_key(&x.to_string()) {
            count += kv.get(&x.to_string()).unwrap().len() as u128;
        }
        let average_time = total_time / count;
        final_results.push(FinalProfileResult {
            name: x.to_string(),
            total_time,
            count,
            average_time,
            nodes: children,
        });
    });

    final_results
}

pub async fn start_profiler() -> u64 {
    let key = rand::random();
    let mut profilers = RUNNING_PROFILERS.write();
    profilers.push(key);
    key
}

pub async fn stop_profiling(key: u64) -> Vec<FinalProfileResult> {
    let mut raw_results = HashMap::new();
    RESULTS_MAP
        .iter()
        .filter(|x| x.key().contains(&key.to_string()))
        .for_each(|x| {
            raw_results.insert(x.key().clone(), x.value().clone());
        });
    let final_results = tokio::task::spawn_blocking(|| generate_final_result(raw_results))
        .await
        .unwrap();
    final_results
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

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    async fn dummy_func1() {
        tokio::time::sleep(Duration::from_millis(1500)).await;
    }

    #[tokio::test]
    async fn test_profiler() {
        let _profiler = start_profiler().await;
        info!("test");
        let results = stop_profiling(0).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "test");
    }
}
