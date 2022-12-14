pub mod backtest;
pub mod data;
pub mod rule;
pub mod set;
pub mod shape;

use chrono::offset::{Local, TimeZone};
use chrono::Date;
use data::read_csv;
use plotters::prelude::*;
use rule::FuzzyEngine;
use set::{arange, LinguisticVar};
use shape::{trapezoidal, triangular};
use std::error::Error;

fn parse_time(t: &str) -> Date<Local> {
    Local
        .datetime_from_str(t, "%Y-%m-%d %H:%M:%S %Z")
        .unwrap()
        .date()
}

fn max_of_vec(vec: &Vec<f64>) -> f64 {
    vec.iter().fold(f64::NAN, |max, &val| val.max(max))
}

fn min_of_vec(vec: &Vec<f64>) -> f64 {
    vec.iter().fold(f64::NAN, |min, &val| val.min(min))
}

fn plot(date: &Vec<Date<Local>>, data: [&Vec<f64>; 3], path: &str) -> Result<(), Box<dyn Error>> {
    let root = SVGBackend::new(path, (1024, 1024)).into_drawing_area();
    root.fill(&WHITE)?;
    let area = root.split_evenly((3, 1));

    let colors = [&BLUE, &GREEN, &RED];
    let names = ["ETH Price Chart", "Long Signal", "Short Signal"];
    let limits = [
        (min_of_vec(&data[0]), max_of_vec(&data[0])),
        (0.0, 80.0),
        (0.0, 80.0),
    ];
    for (i, draw_area) in area.iter().enumerate() {
        let mut chart = ChartBuilder::on(&draw_area)
            .caption(names[i], ("Hack", 44, FontStyle::Bold).into_font())
            .set_label_area_size(LabelAreaPosition::Left, 70)
            .set_label_area_size(LabelAreaPosition::Bottom, 60)
            .margin_right(20)
            .build_cartesian_2d(date[0]..date[date.len() - 1], limits[i].0..limits[i].1)?;

        chart
            .configure_mesh()
            .y_max_light_lines(0)
            .x_labels(11)
            .x_label_formatter(&|v| format!("{}", v.format("%Y-%m-%d")))
            .x_label_style(("Hack", 16).into_font())
            .y_label_style(("Hack", 16).into_font())
            .draw()?;

        chart.draw_series(LineSeries::new(
            date.iter().zip(data[i].iter()).map(|(d, p)| (*d, *p)),
            colors[i].stroke_width(2),
        ))?;
    }
    root.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rsi_var = LinguisticVar::new(
        vec![
            (&triangular(0f64, 1.0, 30f64), "low"),
            (&triangular(50f64, 1.0, 30f64), "medium"),
            (&triangular(100f64, 1.0, 30f64), "high"),
        ],
        arange(0f64, 100f64, 0.01),
    );
    let bb_var = LinguisticVar::new(
        vec![
            (&triangular(-120f64, 1.0, 30f64), "long"),
            (&trapezoidal(-100f64, -50f64, 50f64, 100f64, 1.0), "wait"),
            (&triangular(120f64, 1.0, 30f64), "short"),
        ],
        arange(-150f64, 150f64, 0.01),
    );
    let long = LinguisticVar::new(
        vec![
            (&triangular(0f64, 1.0, 15f64), "weak"),
            (&triangular(30f64, 1.0, 30f64), "strong"),
            (&triangular(100f64, 1.0, 60f64), "verystrong"),
        ],
        arange(0f64, 100f64, 0.01),
    );
    let short = LinguisticVar::new(
        vec![
            (&triangular(0f64, 1.0, 15f64), "weak"),
            (&triangular(30f64, 1.0, 30f64), "strong"),
            (&triangular(100f64, 1.0, 60f64), "verystrong"),
        ],
        arange(0f64, 100f64, 0.01),
    );

    //rsi_var.plot("rsi".into(), "img/rsi.svg".into())?;
    //bb_var.plot("bollinger bands".into(), "img/bb.svg".into())?;
    //long.plot("long, short".into(), "img/ls.svg".into())?;

    let mut f_engine = FuzzyEngine::new([rsi_var, bb_var], [long, short]);

    f_engine.add_rule(["high", "long"], ["weak", "weak"]);
    f_engine.add_rule(["high", "wait"], ["weak", "strong"]);
    f_engine.add_rule(["high", "short"], ["weak", "verystrong"]);

    f_engine.add_rule(["medium", "long"], ["weak", "strong"]);
    f_engine.add_rule(["medium", "wait"], ["weak", "weak"]);
    f_engine.add_rule(["medium", "short"], ["strong", "weak"]);

    f_engine.add_rule(["low", "long"], ["verystrong", "weak"]);
    f_engine.add_rule(["low", "wait"], ["strong", "weak"]);
    f_engine.add_rule(["low", "short"], ["weak", "weak"]);

    let data = read_csv("eth.csv");
    let rsi = data::rsi(&data, 14)[2256..].to_vec();
    let bb = data::bb(&data, 20)[2256..].to_vec();
    let price: Vec<f64> = data[2256..].iter().map(|x| x.price).collect();
    let bb_inputs: Vec<f64> = price
        .iter()
        .zip(bb.iter())
        .map(|(p, y)| 100.0 * (p - y.0) / (2.0 * y.1))
        .collect();

    let date: Vec<Date<Local>> = data[2256..]
        .iter()
        .map(|x| parse_time(x.snapped_at.as_str()))
        .collect();

    let mut long_singal: Vec<f64> = vec![];
    let mut short_singal: Vec<f64> = vec![];
    for i in 0..price.len() {
        let result = f_engine.calculate([rsi[i], bb_inputs[i]]);
        long_singal.push(result[0].centroid_defuzz());
        short_singal.push(result[1].centroid_defuzz());
    }
    /*
    plot(
        &date,
        [&price, &long_singal, &short_singal],
        "img/chart.svg",
    )?;
    */

    backtest::f_backtest(&price, &long_singal, false);
    backtest::f_backtest(&price, &short_singal, true);
    backtest::c_backtest(&price, &rsi, &bb, false);
    backtest::c_backtest(&price, &rsi, &bb, true);

    Ok(())
}
