use std::{collections::HashMap, process::Command};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Plot {
    pub cpu: String,
    pub plot_data: HashMap<String, Vec<PlotData>>,
}

impl From<Results> for Plot {
    fn from(results: Results) -> Self {
        let date = Utc::now();
        let mut plot_data = HashMap::new();
        for result in results.results {
            let plot_data_ = PlotData {
                date,
                mean: result.mean,
                std_dev: result.stddev,
            };

            plot_data
                .entry(result.command)
                .and_modify(|data: &mut Vec<_>| {
                    data.push(plot_data_.clone());
                })
                .or_insert(vec![plot_data_]);
        }

        let output = Command::new("./cpu-name.sh").output().unwrap();
        let mut cpu = String::from_utf8_lossy(&output.stdout).into_owned();
        cpu = cpu.trim().to_string();

        Plot { cpu, plot_data }
    }
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct PlotData {
    pub date: DateTime<Utc>,
    pub mean: f32,
    pub std_dev: f32,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Results {
    results: Vec<Result>,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
struct Result {
    command: String,
    mean: f32,
    stddev: f32,
    median: f32,
    user: f32,
    system: f32,
    min: f32,
    max: f32,
    times: Vec<f32>,
    exit_codes: Vec<i64>,
}
