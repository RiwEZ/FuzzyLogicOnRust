use crate::set::*;
use crate::shape::*;
use std::error::Error;

struct FuzzyEngine<const N: usize> {
    inputs_var: [LinguisticVar; N],
    output_var: LinguisticVar,
    rules: Vec<([String; N], String)>, // list of ([input1_term, input2_term, ...] -> output_term)
}

impl<const N: usize> FuzzyEngine<N> {
    pub fn new(inputs_var: [LinguisticVar; N], output_var: LinguisticVar) -> FuzzyEngine<N> {
        FuzzyEngine {
            inputs_var,
            output_var,
            rules: Vec::<([String; N], String)>::new(),
        }
    }

    pub fn add_rule(&mut self, cond: [String; N], res: String) {
        for i in 0..self.inputs_var.len() {
            self.inputs_var[i].term(&cond[i]);
        }
        self.output_var.term(&res);
        self.rules.push((cond, res));
    }

    pub fn calculate(&self, inputs: [f64; N]) -> FuzzySet {
        let mut temp: Vec<FuzzySet> = vec![];
        for j in 0..self.rules.len() {
            let mut aj = f64::MAX;
            for i in 0..self.rules[j].0.len() {
                let fuzzy_set = self.inputs_var[i].term(&self.rules[j].0[i]);

                let v = fuzzy_set.degree_of(inputs[i]);
                aj = aj.min(v);
            }
            let out = self
                .output_var
                .term(&self.rules[j].1)
                .min(aj, format!("f{}", j));
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
    #[should_panic]
    fn test_adding_rule() {
        let rsi = LinguisticVar::new(
            vec![
                (trinagular(20f64, 1.0, 20f64), "low".into()),
                (trinagular(80f64, 1.0, 20f64), "high".into()),
            ],
            arange(0f64, 100f64, 0.01),
        );
    }

    #[test]
    fn basic_test() -> Result<(), Box<dyn Error>> {
        let rsi = LinguisticVar::new(
            vec![
                (trinagular(20f64, 1.0, 20f64), "low".into()),
                (trinagular(80f64, 1.0, 20f64), "high".into()),
            ],
            arange(0f64, 100f64, 0.01),
        );

        let ma = LinguisticVar::new(
            vec![(trinagular(30f64, 0.8, 20f64), "low".into())],
            arange(0f64, 100f64, 0.01),
        );

        let trend = LinguisticVar::new(
            vec![(trinagular(30f64, 1f64, 30f64), "weak".into())],
            arange(0f64, 100f64, 0.01),
        );

        rsi.plot("rsi".into(), "img/rsi.svg".into())?;
        ma.plot("ma".into(), "img/ma.svg".into()).unwrap();
        trend.plot("trend".into(), "img/trend.svg".into()).unwrap();

        let mut f_engine = FuzzyEngine::<2>::new([rsi, ma], trend);

        f_engine.add_rule(["low".into(), "low".into()], "weak".into());
        f_engine.add_rule(["high".into(), "low".into()], "weak".into());

        let res = f_engine.calculate([15f64, 25f64]);
        res.plot("t2".into(), "img/t2.svg".into())?;

        Ok(())
    }
}
