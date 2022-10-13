// get long, short signal as an input
// iterate over all data with inital capital 1000$
// entry when signal is >= 40
// entry size is 20% of capital
// stop-loss when price go down 10%
// take profit when price go up 20%

struct Position {
    at_price: f64,
    at_time: usize,
    amount: f64,
    realize: bool,
}

impl Position {
    pub fn new(price: f64, money: f64, time: usize) -> Position {
        Position {
            at_price: price,
            at_time: time,
            amount: money / price,
            realize: false,
        }
    }
}

fn realizing_pos(capital: f64, price: &Vec<f64>, pos_list: &mut Vec<Position>, pos_type: bool) {
    let mut profit: Vec<f64> = vec![];
    let mut losses: Vec<f64> = vec![];
    for (i, p) in price.iter().enumerate() {
        for j in 0..pos_list.len() {
            if i <= pos_list[j].at_time || pos_list[j].realize == true {
                continue;
            }
            let diff = (p - pos_list[j].at_price) / pos_list[j].at_price * 100.0;
            if !pos_type {
                if diff > 20.0 {
                    profit.push((p - pos_list[j].at_price) * pos_list[j].amount);
                    pos_list[j].realize = true;
                } else if diff < -10.0 {
                    losses.push((p - pos_list[j].at_price) * pos_list[j].amount);
                    pos_list[j].realize = true;
                }
            } else {
                if diff < -20.0 {
                    profit.push(-1.0 * (p - pos_list[j].at_price) * pos_list[j].amount);
                    pos_list[j].realize = true;
                } else if diff > 10.0 {
                    losses.push(-1.0 * (p - pos_list[j].at_price) * pos_list[j].amount);
                    pos_list[j].realize = true;
                }
            }
        }
    }

    let total_profit = profit.iter().fold(0.0, |s, x| s + x);
    let total_losses = losses.iter().fold(0.0, |s, x| s + x);
    println!("total trade: {:.3}", pos_list.len());
    println!("net profit: {:.3}", total_profit + total_losses);
    println!("count: {}, profits: {:.3}", profit.len(), total_profit);
    println!("count: {}, losses: {:.3}", losses.len(), total_losses);
    println!("---------------");
    /*
    println!(
        "total capital(+net_profit)): {}",
        capital + (1000.0 - capital) + total_profit + total_losses
    );
    */
}

/// Fuzzy BackTest
/// pos_type - false for long, true for short
pub fn f_backtest(price: &Vec<f64>, signal: &Vec<f64>, pos_type: bool) {
    let mut capital = 1000.0;
    let mut pos_list: Vec<Position> = vec![];

    for (i, p) in price.iter().enumerate() {
        if signal[i] >= 40.0 {
            if capital > 0.0 {
                let pos = Position::new(*p, 100.0, i);
                capital -= 100.0;
                pos_list.push(pos);
            }
        }
    }

    realizing_pos(capital, price, &mut pos_list, pos_type)
}

pub fn c_backtest(price: &Vec<f64>, rsi: &Vec<f64>, bb: &Vec<(f64, f64)>, pos_type: bool) {
    let mut capital = 1000.0;
    let mut pos_list: Vec<Position> = vec![];

    for (i, p) in price.iter().enumerate() {
        let beta = (p - bb[i].0) / (2.0 * bb[i].1);
        if !pos_type {
            if rsi[i] < 30.0 && beta < -0.9 {
                if capital > 0.0 {
                    let pos = Position::new(*p, 100.0, i);
                    capital -= 100.0;
                    pos_list.push(pos);
                } else if rsi[i] < 30.0 && beta >= -0.9 && beta < 0.0 {
                    let pos = Position::new(*p, 100.0, i);
                    capital -= 100.0;
                    pos_list.push(pos);
                }
            }
        } else {
            if rsi[i] > 70.0 && beta < 0.9 {
                if capital > 0.0 {
                    let pos = Position::new(*p, 100.0, i);
                    capital -= 100.0;
                    pos_list.push(pos);
                } else if rsi[i] > 70.0 && beta <= 0.9 && beta > 0.0 {
                    let pos = Position::new(*p, 100.0, i);
                    capital -= 100.0;
                    pos_list.push(pos);
                }
            }
        }
    }

    realizing_pos(capital, price, &mut pos_list, pos_type)
}
