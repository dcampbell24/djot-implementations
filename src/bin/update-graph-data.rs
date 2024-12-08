use std::fs;

use djot_implementations::{Plot, Results};

fn main() -> anyhow::Result<()> {
    // read the old graph data.
    // read the new graph data and append it to the old.
    for (json, ron) in [
        (
            "tmp/pandoc-manual-benchmarks.json",
            "tmp/pandoc-manual-benchmarks.ron",
        ),
        (
            "tmp/tartan-wikipedia-benchmarks.json",
            "tmp/tartan-wikipedia-benchmarks.ron",
        ),
    ] {
        let data: String = fs::read_to_string(json)?;
        let results: Results = serde_json::from_str(&data)?;
        let plot = Plot::from(results);
        let plot_ron = ron::ser::to_string_pretty(&plot, ron::ser::PrettyConfig::default())?;
        fs::write(ron, plot_ron.as_bytes())?;
    }

    Ok(())
}
