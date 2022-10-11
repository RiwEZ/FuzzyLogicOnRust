use crate::set::*;
use crate::shape::*;
use std::error::Error;

pub struct FuzzyEngine<const N: usize, const M: usize> {
    inputs_var: [LinguisticVar; N],
    outputs_var: [LinguisticVar; M],
    rules: Vec<(Vec<String>, Vec<String>)>, // list of ([input1_term, input2_term, ...] -> output_term)
}

impl<const N: usize, const M: usize> FuzzyEngine<N, M> {
    pub fn new(inputs_var: [LinguisticVar; N], output_var: [LinguisticVar; M]) -> FuzzyEngine<N, M> {
        FuzzyEngine {
            inputs_var,
            outputs_var: output_var,
            rules: Vec::<(Vec<String>, Vec<String>)>::new(),
        }
    }

    pub fn add_rule(&mut self, cond: [&str; N], res: [&str; M]) {
        for i in 0..self.inputs_var.len() {
            self.inputs_var[i].term(&cond[i]); // check if term "cond[i]" exist
        }
        for i in 0..self.outputs_var.len() {
            self.outputs_var[i].term(&res[i]); // term() check if term "res" is exist
        }
        
        let conditions: Vec<String> = cond.iter().map(|x| x.to_string()).collect();
        let results: Vec<String> = res.iter().map(|x| x.to_string()).collect();
        self.rules.push((conditions, results));
    }

    pub fn calculate(&self, inputs: [f64; N]) -> Vec<FuzzySet> {
        let mut temp: Vec<Vec<FuzzySet>> = vec![];
        for j in 0..self.rules.len() {
            let mut aj = f64::MAX;
            for i in 0..self.rules[j].0.len() {
                let fuzzy_set = self.inputs_var[i].term(&self.rules[j].0[i]);
                let v = fuzzy_set.degree_of(inputs[i]);
                aj = aj.min(v);
            }

            let mut t: Vec<FuzzySet> = vec![];
            for i in 0..self.rules[j].1.len() {
                t.push(self
                    .outputs_var[i]
                    .term(&self.rules[j].1[i])
                    .min(aj, format!("f{}", j)));
            }
            
            temp.push(t);
        }
        let mut res: Vec<FuzzySet> = vec![];
        for i in 0..M {
            res.push(temp[0][i].std_union(&temp[0][i], "".into()));
            
        }
        for j in 1..temp.len() {
            for i in 0..M {
                res[i] = res[i].std_union(&temp[j][i], "".into());
            }
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
                (&trinagular(20f64, 1.0, 20f64), "low"),
                (&trinagular(80f64, 1.0, 20f64), "high"),
            ],
            arange(0f64, 100f64, 0.01),
        );

        let mut f_engine = FuzzyEngine::new([rsi.clone()], [rsi]);
        f_engine.add_rule(["medium".into()], ["low".into()]);
    }

    #[test]
    fn basic_test() -> Result<(), Box<dyn Error>> {
        let rsi = LinguisticVar::new(
            vec![
                (&trinagular(20f64, 1.0, 20f64), "low"),
                (&trinagular(80f64, 1.0, 20f64), "high"),
            ],
            arange(0f64, 100f64, 0.01),
        );

        let ma = LinguisticVar::new(
            vec![(&trinagular(30f64, 0.8, 20f64), "low")],
            arange(0f64, 100f64, 0.01),
        );

        let trend = LinguisticVar::new(
            vec![(&trinagular(30f64, 1f64, 30f64), "weak")],
            arange(0f64, 100f64, 0.01),
        );

        rsi.plot("rsi".into(), "img/test/rsi.svg".into())?;
        ma.plot("ma".into(), "img/test/ma.svg".into()).unwrap();
        ma.plot("ma".into(), "img/test/ma.svg".into()).unwrap();
        trend.plot("trend".into(), "img/test/trend.svg".into()).unwrap();

        let mut f_engine = FuzzyEngine::new([rsi, ma], [trend]);

        f_engine.add_rule(["low", "low"], ["weak"]);
        f_engine.add_rule(["high", "low"], ["weak"]);

        let res = f_engine.calculate([15f64, 25f64]);
        res[0].plot("t2".into(), "img/test/t2.svg".into())?;

        Ok(())
    }
}
