use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct Record {
    pub snapped_at: String,
    pub price: f64,
}

/// return (gain, loss)
fn gain_loss(p0: f64, p1: f64) -> (f64, f64) {
    let mut res = (0.0, 0.0);
    if p1 > p0 {
        res.0 = p1 - p0
    } else {
        res.1 = -(p1 - p0)
    }
    res
}

fn rsi(data: &Vec<Record>, n: usize) -> Vec<f64> {
    let mut rsi: Vec<f64> = vec![];

    let mut gains: Vec<f64> = vec![];
    let mut losses: Vec<f64> = vec![];

    let mut avg_g: Vec<f64> = vec![];
    let mut avg_l: Vec<f64> = vec![];

    for (i, item) in data.iter().enumerate() {
        if i <= n + 1 && i > 0 {
            let gl = gain_loss(data[i - 1].price, item.price);
            gains.push(gl.0);
            losses.push(gl.1);
        }

        if i <= n {
            rsi.push(-1.0);
            avg_g.push(0.0);
            avg_l.push(0.0);
        } else if i == n + 1 {
            let avg_gain = gains.iter().sum::<f64>() / (n as f64);
            let avg_loss = losses.iter().sum::<f64>() / (n as f64);
            rsi.push(100f64 - 100f64 / (1.0 + (avg_gain / avg_loss)));
            avg_g.push(avg_gain);
            avg_l.push(avg_loss);
        } else {
            let gl = gain_loss(data[i - 1].price, item.price);
            let avg_gain = (avg_g[i - 1] * (n - 1) as f64 + gl.0) / (n as f64);
            let avg_loss = (avg_l[i - 1] * (n - 1) as f64 + gl.1) / (n as f64);
            rsi.push(100f64 - 100f64 / (1.0 + (avg_gain / avg_loss)));
            avg_g.push(avg_gain);
            avg_l.push(avg_loss);
        }
    }
    rsi
}

fn bb(data: &Vec<Record>, n: usize) -> Vec<(f64, f64)> {
    let mut bb: Vec<(f64, f64)> = vec![];
    for (i, _) in data.iter().enumerate() {
        if i < n {
            bb.push((-1.0, -1.0));
            continue;
        }

        let mut sum = 0.0;
        for j in (i-n)..i {
            sum += data[j].price;   
        }
        let ma = sum/(n as f64);
        let std = ((sum - ma)/n as f64).sqrt(); 
        bb.push((ma + 2.0*std, ma - 2.0*std));
    }
    bb
}

/// read coingecko csv data
fn read_csv(path: &str) -> Vec<Record> {
    let contents = fs::read_to_string(path).unwrap();

    let mut rdr = csv::Reader::from_reader(contents.as_bytes());
    let mut data: Vec<Record> = vec![];
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        data.push(record);
    }
    data
}