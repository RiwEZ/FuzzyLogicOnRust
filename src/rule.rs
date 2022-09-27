use crate::set::*;
use crate::shape::*;

struct FuzzyEngine<const N: usize> {
    rules: Vec<([FuzzySet; N], FuzzySet)>,
}

impl<const N: usize> FuzzyEngine<N> {
    pub fn new() -> FuzzyEngine<N> {
        FuzzyEngine {
            rules: Vec::<([FuzzySet; N], FuzzySet)>::new(),
        }
    }

    pub fn add_rule(&mut self, cond: [FuzzySet; N], res: FuzzySet) {
        self.rules.push((cond, res));
    }

    pub fn calculate(&self, inputs: [f64; N]) -> FuzzySet {
        let mut temp: Vec<FuzzySet> = vec![];
        for j in 0..self.rules.len() {
            let mut aj = f64::MAX;
            for i in 0..self.rules[j].0.len() {
                let v = self.rules[j].0[i].degree_of(inputs[i]);
                aj = aj.min(v);
            }

            let out = self.rules[j].1.min(aj, format!("f{}", j));
            temp.push(out);
        }

        let mut res = temp[0].std_union(&temp[0], "".into());
        for j in 1..temp.len() {
            res = res.std_union(&temp[j], "".into());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut rsi = Domain::new("rsi".into(), 0f64, 100f64, 0.01f64);
        let mut ma = Domain::new("ma".into(), 0f64, 100f64, 0.01f64);
        let mut trend = Domain::new("trend".into(), 0f64, 100f64, 0.01f64);

        let rsi_l = FuzzySet::new(
            &rsi.members,
            trinagular(20f64, 1.0f64, 20f64),
            "low_rsi".into(),
        )
        ;
        let rsi_h = FuzzySet::new(
            &rsi.members,
            trinagular(80f64, 1.0f64, 20f64),
            "high_rsi".into(),
        );

        let ma_l = FuzzySet::new(
            &ma.members,
            trinagular(30f64, 0.8f64, 20f64),
            "low_ma".into(),
        );

        let weak = FuzzySet::new(
            &trend.members,
            trinagular(30f64, 1.0f64, 30f64),
            "weak".into(),
        );

        let mut f_engine = FuzzyEngine::<2>::new();

        f_engine.add_rule([rsi_l.clone(), ma_l.clone()], weak.clone());
        f_engine.add_rule([rsi_h.clone(), ma_l.clone()], weak.clone());

        let res = f_engine.calculate([15f64, 25f64]);

        rsi.add(rsi_l);
        rsi.add(rsi_h);
        rsi.plot("img/rsi.png".into()).unwrap();

        ma.add(ma_l);
        ma.plot("img/ma.png".into()).unwrap();

        trend.add(res);
        trend.add(weak);
        trend.plot("img/t2.png".into()).unwrap();
    }
}
