use std::fs::{self, File};

use anyhow::{Context, Ok};
use chrono::Days;
use djot_implementations::Plot;
use full_palette::GREY_A100;
use plotters::prelude::*;

// Paul Tol's Notes
// https://personal.sron.nl/~pault/
const BLUE_BRIGHT: RGBColor = RGBColor(0x44, 0x77, 0xAA);
const CYAN_BRIGHT: RGBColor = RGBColor(0x66, 0xCC, 0xEE);
const GREEN_BRIGHT: RGBColor = RGBColor(0x22, 0x88, 0x33);
const YELLOW_BRIGHT: RGBColor = RGBColor(0xCC, 0xBB, 0x44);
const RED_BRIGHT: RGBColor = RGBColor(0xEE, 0x66, 0x77);
const _PURPLE_BRIGHT: RGBColor = RGBColor(0xAA, 0x33, 0x77);
const _GREY_BRIGHT: RGBColor = RGBColor(0xBB, 0xBB, 0xBB);

fn main() -> anyhow::Result<()> {
    make_graph(
        "pandoc-manual.dj",
        "tmp/pandoc-manual-benchmarks.ron",
        "tmp/pandoc-manual-benchmarks.png",
    )?;
    make_graph(
        "tartan-wikipedia.dj",
        "tmp/tartan-wikipedia-benchmarks.ron",
        "tmp/tartan-wikipedia-benchmarks.png",
    )?;
    Ok(())
}

fn make_graph(render_file: &str, file_in: &str, file_out: &str) -> anyhow::Result<()> {
    let data_1: String = fs::read_to_string(file_in).context(format!("Open {file_in}"))?;
    let data_1: Plot = ron::from_str(&data_1)?;

    let file = File::open(format!("benchmark-files/{render_file}"))
        .context(format!("Open {render_file}"))?;
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
    let colors = [
        RED_BRIGHT,
        YELLOW_BRIGHT,
        GREEN_BRIGHT,
        CYAN_BRIGHT,
        BLUE_BRIGHT,
    ];
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
    root.present().expect(
        "Unable to write result to file, please make sure 'tmp' dir exists under current dir",
    );
    println!("Result has been saved to {file_out}");

    Ok(())
}

/// Formats a number of bytes into a human readable SI-prefixed size.
/// Returns a tuple of `(quantity, units)`.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
#[must_use]
pub fn human_readable_bytes(bytes: u64) -> (f32, &'static str) {
    static UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
    let bytes = bytes as f32;
    let i = ((bytes.log2() / 10.0) as usize).min(UNITS.len() - 1);
    (bytes / 1024_f32.powi(i as i32), UNITS[i])
}
