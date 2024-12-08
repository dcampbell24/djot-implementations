use std::fs::{self, File};

use anyhow::Ok;
use chrono::Days;
use djot_implementations::Plot;
use full_palette::GREY_A100;
use plotters::prelude::*;

// IBM Design Library
const BLUE_IBM: RGBColor = RGBColor(0x64, 0x8F, 0xFF);
const PURPLE_IBM: RGBColor = RGBColor(0x78, 0x5E, 0xF0);
const RED_IBM: RGBColor = RGBColor(0xDC, 0x26, 0x7F);
const ORANGE_IBM: RGBColor = RGBColor(0xFE, 0x61, 0x00);
const YELLOW_IBM: RGBColor = RGBColor(0xFF, 0xB0, 0x00);

fn main() -> anyhow::Result<()> {
    make_graph(
        "pandoc-manual.dj",
        "ron/pandoc-manual-benchmarks.ron",
        "plotters-graphs/pandoc-manual-benchmarks.png",
    )?;
    make_graph(
        "tartan-wikipedia.dj",
        "ron/tartan-wikipedia-benchmarks.ron",
        "plotters-graphs/tartan-wikipedia-benchmarks.png",
    )?;
    Ok(())
}

fn make_graph(render_file: &str, file_in: &str, file_out: &str) -> anyhow::Result<()> {
    let data_1: String = fs::read_to_string(file_in)?;
    let data_1: Plot = ron::from_str(&data_1)?;

    let file = File::open(format!("djot/{render_file}"))?;
    let metadata = file.metadata()?;
    let (file_size, suffix) = human_readable_bytes(metadata.len());

    let root = BitMapBackend::new(file_out, (1024, 768)).into_drawing_area();
    root.fill(&GREY_A100)?;
    root.titled(
        &format!("Time to Render {render_file} ({file_size:.2}{suffix}) into html (ms)"),
        ("sans-serif", 40.0),
    )?;

    let to_date = data_1.plot_data["Go"]
        .first()
        .unwrap()
        .date
        .checked_add_days(Days::new(1))
        .unwrap();

    let from_date = data_1.plot_data["Go"]
        .last()
        .unwrap()
        .date
        .checked_sub_days(Days::new(1))
        .unwrap();

    let (_upper, lower) = root.split_vertically(50);

    let data = data_1.plot_data.values();

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    let max = data
        .map(|data| {
            data.iter()
                .map(|data| ((data.mean + data.std_dev) * 1_000.0) as i32)
                .max()
                .unwrap()
        })
        .max()
        .unwrap() as f32;

    let mut chart = ChartBuilder::on(&lower)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption(format!("on {}", data_1.cpu), ("sans-serif", 20).into_font())
        .build_cartesian_2d(from_date..to_date, 0f32..max)?;

    chart.configure_mesh().light_line_style(GREY_A100).draw()?;

    let commands = ["Go", "Haskell", "JavaScript", "Lua", "Rust"];
    let colors = [RED_IBM, ORANGE_IBM, YELLOW_IBM, BLUE_IBM, PURPLE_IBM];
    for (command, color) in commands.iter().zip(colors) {
        let data = &data_1.plot_data[*command];

        chart
            .draw_series(data.iter().map(|data| {
                CandleStick::new(
                    data.date,
                    data.mean * 1_000.0,
                    (data.mean + data.std_dev) * 1_000.0,
                    (data.mean - data.std_dev) * 1_000.0,
                    data.mean * 1_000.0,
                    color.filled(),
                    color,
                    15,
                )
            }))?
            .label(*command)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(GREY_A100)
        .draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-graphs' dir exists under current dir");
    println!("Result has been saved to {file_out}");

    Ok(())
}

/// Formats a number of bytes into a human readable SI-prefixed size.
/// Returns a tuple of `(quantity, units)`.
pub fn human_readable_bytes(bytes: u64) -> (f32, &'static str) {
    static UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let bytes = bytes as f32;
    let i = ((bytes.log2() / 10.0) as usize).min(UNITS.len() - 1);
    (bytes / 1024_f32.powi(i as i32), UNITS[i])
}
