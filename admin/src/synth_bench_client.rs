use anyhow::{anyhow, Result};

use reqwest::StatusCode;
use serde_json::json;

use crate::serde::synth_bench::SynthBench;
use crate::serde::synth_bench::SynthBenchResult;

pub async fn get_cpus() -> Result<SynthBench> {
    let query = "{
    synthBench {
      cpus {
        id
        attributes {
          attributes {
            key
            value
          }
        }
        title
        passmarkSingleThread
        synthPassmarkSingleThread
      }
      synthPassmarkSingleThreadMse
    }
}";
    let request_json = json!({ "query": query });

    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:19000/graphql")
        .header("Content-Type", "application/json")
        .body(request_json.to_string())
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let result: SynthBenchResult = serde_json::from_str(&response.text().await?)?;
            match result.data {
                Some(data) => Ok(data.synth_bench),
                None => {
                    return Err(anyhow!(std::format!(
                        "Call to Synth Bench failed: {:?}",
                        result
                            .errors
                            .ok_or_else(|| anyhow!("data and errors both null")),
                    )))
                }
            }
        }
        _ => Err(anyhow!(std::format!(
            "Getting grammars failed: {:?}",
            response
        ))),
    }
}
