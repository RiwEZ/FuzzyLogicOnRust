pub mod rule;
pub mod set;
pub mod shape;
pub mod data;

use shape::trinagular;
use set::LinguisticVar;
use set::arange;
use rule::FuzzyEngine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rsi = LinguisticVar::new(
        vec![
            (trinagular(0f64, 1.0, 30f64), "low"),
            (trinagular(50f64, 1.0, 30f64), "medium"),
            (trinagular(100f64, 1.0, 30f64), "high"),
        ],
        arange(0f64, 100f64, 0.01)
    );
    let bb = LinguisticVar::new(
        vec![
            (trinagular(-120f64, 1.0, 30f64), "short"),
            (trinagular(0f64, 1.0, 100f64), "wait"),
            (trinagular(120f64, 1.0, 30f64), "long"),
        ],
        arange(-120f64, 120f64, 0.01)
    );
    let long = LinguisticVar::new(
        vec![
            (trinagular(0f64, 1.0, 15f64), "weak"),
            (trinagular(30f64, 1.0, 30f64), "strong"),
            (trinagular(100f64, 1.0, 60f64), "verystrong"),
        ],
        arange(0f64, 100f64, 0.01)
    );
    let short = LinguisticVar::new(
        vec![
            (trinagular(0f64, 1.0, 15f64), "weak"),
            (trinagular(30f64, 1.0, 30f64), "strong"),
            (trinagular(100f64, 1.0, 60f64), "verystrong"),
        ],
        arange(0f64, 100f64, 0.01)
    );

    rsi.plot("rsi".into(), "img/rsi.svg".into())?;
    bb.plot("bollinger bands".into(), "img/bb.svg".into())?;
    long.plot("long, short".into(), "img/ls.svg".into())?;
    
    let mut f_engine = FuzzyEngine::new([rsi, bb], [long, short]);

    Ok(())
}
