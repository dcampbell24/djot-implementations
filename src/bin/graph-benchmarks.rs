use std::fs;

use anyhow::Ok;
use chrono::Days;
use djot_implementations::Plot;
use full_palette::{GREY_A100, GREY_A200, PURPLE};
use plotters::prelude::*;

fn main() -> anyhow::Result<()> {
    make_graph(
        "pandoc-manual.dj",
        "ron/benchmarks-1.ron",
        "plotters-graphs/pandoc-manual-benchmarks.png",
    )?;
    make_graph(
        "tartan-wikipedia.dj",
        "ron/benchmarks-2.ron",
        "plotters-graphs/tartan-wikipedia-benchmarks.png",
    )?;
    Ok(())
}

fn make_graph(render_file: &str, file_in: &str, file_out: &str) -> anyhow::Result<()> {
    let data_1: String = fs::read_to_string(file_in)?;
    let data_1: Plot = ron::from_str(&data_1)?;

    let root = BitMapBackend::new(file_out, (1024, 768)).into_drawing_area();
    root.fill(&GREY_A200)?;
    root.titled(
        &format!("Time to Render {render_file} to html (ms)"),
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

    let data = data_1.plot_data.iter().map(|(_command, data)| data);
    let max = data
        .map(|data| {
            data.iter()
                .map(|data| ((data.mean + data.std_dev) * 1_000.0 + 10.0) as i32)
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

    chart.configure_mesh().light_line_style(GREY_A200).draw()?;

    let commands = ["Go", "Haskell", "JavaScript", "Lua", "Rust"];
    let colors = [RED, YELLOW, GREEN, BLUE, PURPLE];
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
