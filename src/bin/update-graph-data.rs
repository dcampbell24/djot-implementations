use std::fs;

use djot_implementations::{Plot, Results};

fn main() -> anyhow::Result<()> {
    // read the old graph data.
    // read the new graph data and append it to the old.
    let data: String = fs::read_to_string("json/benchmarks-1.json")?;
    let results: Results = serde_json::from_str(&data)?;
    let plot = Plot::from(results);
    let plot_ron = ron::ser::to_string_pretty(&plot, ron::ser::PrettyConfig::default())?;
    fs::write("ron/benchmarks-1.ron", plot_ron.as_bytes())?;
    Ok(())
}
