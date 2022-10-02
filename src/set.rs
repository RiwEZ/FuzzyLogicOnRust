use crate::shape::*;
use plotters::prelude::*;
use std::error::Error;

pub fn arange(start: f64, stop: f64, interval: f64) -> Vec<f64> {
    if stop < start {
        panic!("end can not be less than start");
    } else if interval <= 0f64 {
        panic!("interval must be > 0");
    }

    let mut members: Vec<f64> = vec![];
    let r = 1.0 / interval;
    let mut n = start;
    while n <= stop {
        members.push(n);
        n += interval;
        if interval < 1.0 {
            n = (n * r).round() / r;
        }
    }
    members
}

pub struct LinguisticVar {
    pub sets: Vec<FuzzySet>,
    pub universe: Vec<f64>,
}

impl LinguisticVar {
    pub fn new<T: Shape>(inputs: Vec<(T, String)>, universe: Vec<f64>) -> LinguisticVar {
        let mut sets: Vec<FuzzySet> = vec![];
        for item in inputs {
            sets.push(FuzzySet::new(&universe, item.0, item.1.clone()));
        }
        LinguisticVar { sets, universe }
    }

    pub fn term(&self, name: &String) -> &FuzzySet {
        match self.sets.iter().find(|x| x.name == *name) {
            Some(x) => x,
            None => panic![
                "there're no fuzzy set name {} in this linguistic variable",
                name
            ],
        }
    }

    pub fn plot(&self, name: String, path: String) -> Result<(), Box<dyn Error>> {
        let root = SVGBackend::new(&path, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(name, ("Hack", 44, FontStyle::Bold).into_font())
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 60)
            .margin(20)
            .build_cartesian_2d(
                self.universe[0]..self.universe[self.universe.len() - 1],
                0f64..1f64,
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        for i in 0..self.sets.len() {
            let color = Palette99::pick(i);
            chart
                .draw_series(LineSeries::new(
                    self.universe
                        .iter()
                        .zip(self.sets[i].membership.iter())
                        .map(|(x, y)| (*x, *y)),
                    color.stroke_width(2),
                ))?
                .label(self.sets[i].name.clone())
                .legend(move |(x, y)| PathElement::new([(x, y), (x + 20, y)], color.filled()));
            // there're a problem with styling legend
        }

        chart
            .configure_series_labels()
            .label_font(("Hack", 14).into_font())
            .background_style(&WHITE)
            .border_style(&BLACK)
            .draw()?;

        root.present()?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FuzzySet {
    pub name: String,
    pub universe: Vec<f64>, // universe of discourse that own this set
    pub membership: Vec<f64>,
}

impl FuzzySet {
    pub fn new<T: Shape>(universe: &Vec<f64>, fuzzy_f: T, name: String) -> FuzzySet {
        let mut membership: Vec<f64> = vec![];
        for i in 0..universe.len() {
            membership.push(fuzzy_f.function(universe[i]));
        }
        FuzzySet {
            name: name.to_string(),
            universe: universe.clone(),
            membership,
        }
    }

    pub fn plot(&self, name: String, path: String) -> Result<(), Box<dyn Error>> {
        let root = SVGBackend::new(&path, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(name, ("Hack", 44, FontStyle::Bold).into_font())
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 60)
            .margin(20)
            .build_cartesian_2d(
                self.universe[0]..self.universe[self.universe.len() - 1],
                0f64..1f64,
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        let color = Palette99::pick(0);
        chart
            .draw_series(LineSeries::new(
                self.universe
                    .iter()
                    .zip(self.membership.iter())
                    .map(|(x, y)| (*x, *y)),
                color.stroke_width(2),
            ))?
            .label(self.name.clone())
            .legend(move |(x, y)| PathElement::new([(x, y), (x + 20, y)], color.filled()));

        chart
            .configure_series_labels()
            .label_font(("Hack", 14).into_font())
            .background_style(&WHITE)
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

        Ok(())
    }

    pub fn degree_of(&self, input: f64) -> f64 {
        let i = self
            .universe
            .iter()
            .position(|x| (*x - input).abs() < 0.0000001)
            .unwrap();
        self.membership[i]
    }

    pub fn min(&self, input: f64, name: String) -> FuzzySet {
        let mut membership: Vec<f64> = vec![];
        for i in 0..self.membership.len() {
            membership.push(self.membership[i].min(input));
        }
        FuzzySet {
            name: name.to_string(),
            universe: self.universe.clone(),
            membership,
        }
    }

    pub fn std_union(&self, set: &FuzzySet, name: String) -> FuzzySet {
        // check if domain is equal or not?
        if self.universe != set.universe {
            panic!("domain needs to be equal");
        }

        // if equal
        let mut membership: Vec<f64> = vec![];
        for i in 0..self.membership.len() {
            membership.push(self.membership[i].max(set.membership[i]));
        }
        FuzzySet {
            name: name.to_string(),
            universe: self.universe.clone(),
            membership,
        }
    }

    pub fn std_intersect(&self, set: &FuzzySet, name: String) -> FuzzySet {
        // check if domain is equal or not?
        if self.universe != set.universe {
            panic!("domain needs to be equal");
        }

        // if equal
        let mut membership: Vec<f64> = vec![];
        for i in 0..self.membership.len() {
            membership.push(self.membership[i].min(set.membership[i]));
        }
        FuzzySet {
            name: name.to_string(),
            universe: self.universe.clone(),
            membership,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arange() {
        assert_eq!(arange(0f64, 5f64, 1f64), vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(
            arange(0f64, 0.5f64, 0.1f64),
            vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5]
        );
    }

    #[test]
    fn test_degree() {
        let s1 = FuzzySet::new(
            &arange(0.0, 10.0, 0.01),
            trinagular(5f64, 0.8f64, 3f64),
            "f1".into(),
        );

        assert_eq!(s1.degree_of(5.0f64), 0.8);
        assert_eq!(s1.degree_of(3.5f64), 0.4);
        assert_eq!(s1.degree_of(0.0f64), 0.0);   
    }

    #[test]
    fn linguistic() {
        let var1 = LinguisticVar::new(
            vec![
                (trinagular(5f64, 0.8, 3f64), "normal".into()),
                (trinagular(3f64, 0.8, 1.5f64), "weak".into()),
            ],
            arange(0f64, 10f64, 0.01),
        );

        assert_eq!(var1.term(&"normal".into()).degree_of(5.0), 0.8);
        assert_eq!(var1.term(&"weak".into()).degree_of(3.0), 0.8);

        var1.plot("var1".into(), "img/t.svg".into()).unwrap();
    }
}
