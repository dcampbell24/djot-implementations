use std::fs;

use anyhow::Ok;
use chrono::{DateTime, Days, ParseError, Utc};
use djot_implementations::Plot;
use full_palette::{GREY_A100, GREY_A200, PURPLE};
use plotters::prelude::*;

fn parse_time(t: &str) -> Result<DateTime<Utc>, ParseError> {
    DateTime::parse_from_str(t, "%F %H:%M:%S %:z").map(|dt| dt.to_utc())
}

const OUT_FILE_NAME: &str = "plotters-graphs/pandoc-manual-benchmarks.png";

fn main() -> anyhow::Result<()> {
    let data_1: String = fs::read_to_string("benchmarks-1.ron")?;
    let data_1: Plot = ron::from_str(&data_1)?;

    let go_data = get_data_go();
    let haskell_data = get_data_haskell();
    let javascript_data = get_data_javascript();
    let lua_data = get_data_lua();
    let rust_data = get_data_rust();

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&GREY_A200)?;
    root.titled(
        "Time to Render pandoc-manual.dj to html (ms)",
        ("sans-serif", 40.0),
    )?;

    let to_date = parse_time(go_data.first().unwrap().0)?
        .checked_add_days(Days::new(1))
        .unwrap();
    let from_date = parse_time(go_data.last().unwrap().0)?
        .checked_sub_days(Days::new(1))
        .unwrap();

    let (_upper, lower) = root.split_vertically(50);

    let mut chart = ChartBuilder::on(&lower)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption(
            "on AMD EPYC 7763 64-Core Processor",
            ("sans-serif", 20).into_font(),
        )
        .build_cartesian_2d(from_date..to_date, 0f32..155f32)?;

    chart.configure_mesh().light_line_style(GREY_A200).draw()?;

    chart
        .draw_series(go_data.iter().map(|x| {
            CandleStick::new(
                parse_time(x.0).unwrap(),
                x.1,
                x.2,
                x.3,
                x.4,
                RED.filled(),
                RED,
                15,
            )
        }))?
        .label("Go")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .draw_series(haskell_data.iter().map(|x| {
            CandleStick::new(
                parse_time(x.0).unwrap(),
                x.1,
                x.2,
                x.3,
                x.4,
                YELLOW.filled(),
                YELLOW,
                15,
            )
        }))?
        .label("Haskell")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], YELLOW));

    chart
        .draw_series(javascript_data.iter().map(|x| {
            CandleStick::new(
                parse_time(x.0).unwrap(),
                x.1,
                x.2,
                x.3,
                x.4,
                GREEN.filled(),
                GREEN,
                15,
            )
        }))?
        .label("JavaScript")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));

    chart
        .draw_series(lua_data.iter().map(|x| {
            CandleStick::new(
                parse_time(x.0).unwrap(),
                x.1,
                x.2,
                x.3,
                x.4,
                BLUE.filled(),
                BLUE,
                15,
            )
        }))?
        .label("Lua")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .draw_series(rust_data.iter().map(|x| {
            CandleStick::new(
                parse_time(x.0).unwrap(),
                x.1,
                x.2,
                x.3,
                x.4,
                PURPLE.filled(),
                PURPLE,
                15,
            )
        }))?
        .label("Rust")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], PURPLE));

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(GREY_A100)
        .draw()?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-graphs' dir exists under current dir");
    println!("Result has been saved to {OUT_FILE_NAME}");

    Ok(())
}

fn get_data_go() -> Vec<(&'static str, f32, f32, f32, f32)> {
    vec![("2024-11-29 16:54:45-05:00", 1.1, 1.0, 2.2, 1.1)]
}

fn get_data_haskell() -> Vec<(&'static str, f32, f32, f32, f32)> {
    vec![("2024-11-29 16:54:45-05:00", 42.3, 41.9, 42.7, 42.3)]
}

fn get_data_javascript() -> Vec<(&'static str, f32, f32, f32, f32)> {
    vec![("2024-11-29 16:54:45-05:00", 127.8, 119.8, 136.8, 127.8)]
}

fn get_data_lua() -> Vec<(&'static str, f32, f32, f32, f32)> {
    vec![("2024-11-29 16:54:45-05:00", 149.1, 147.0, 152.9, 149.1)]
}

fn get_data_rust() -> Vec<(&'static str, f32, f32, f32, f32)> {
    vec![("2024-11-29 16:54:45-05:00", 6.0, 5.9, 8.2, 6.0)]
}
