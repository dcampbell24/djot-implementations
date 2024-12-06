use std::process::Command;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Plot {
    document: String,
    cpu: String,
    plot_data: Vec<PlotData>,
}

impl From<Results> for Plot {
    fn from(results: Results) -> Self {
        let date = Utc::now();
        let mut plot_data = Vec::new();
        for result in results.results {
            plot_data.push(PlotData {
                command: result.command,
                date,
                mean: result.mean,
                min: result.min,
                max: result.max,
            });
        }

        let output = Command::new("./cpu-name.sh").output().unwrap();
        let mut cpu = String::from_utf8_lossy(&output.stdout).into_owned();
        cpu = cpu.trim().to_string();

        Plot {
            document: String::new(),
            cpu,
            plot_data,
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
struct PlotData {
    command: String,
    date: DateTime<Utc>,
    mean: f32,
    min: f32,
    max: f32,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Results {
    results: Vec<Result>,
}

#[derive(Serialize, Debug, Deserialize)]
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
