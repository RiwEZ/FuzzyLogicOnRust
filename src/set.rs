use crate::shape::*;
use plotters::prelude::*;

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

pub struct Domain {
    pub name: String,
    pub members: Vec<f64>,
    pub fuzzy_sets: Vec<FuzzySet>,
}

impl Domain {
    /// ## Panics
    /// * Panic if `end` >= `start`
    /// * Panic if `res` is not in range (0, 1)
    pub fn new(name: String, start: f64, end: f64, res: f64) -> Domain {
        if end < start {
            panic!("end can not be less than start");
        } else if res <= 0.0 || res >= 1.0 {
            panic!("res should be in range (0, 1)");
        }

        let mut members: Vec<f64> = vec![];
        let r = 1.0 / res;
        let mut n = start;
        while n <= end {
            members.push(n);
            n += res;
            n = (n * r).round() / r;
        }

        Domain {
            name,
            members,
            fuzzy_sets: Vec::<FuzzySet>::new(),
        }
    }

    pub fn find_set(&self, name: String) -> &FuzzySet {
        match self.fuzzy_sets.iter().find(|x| x.name == name) {
            Some(x) => x,
            None => panic!["can't find this set with name {}", name],
        }
    }

    pub fn add(&mut self, set: FuzzySet) -> &FuzzySet {
        if set.universe != self.members {
            panic!["this set is not in this domain"];
        }
        self.fuzzy_sets.push(set);
        &self.fuzzy_sets[self.fuzzy_sets.len() - 1]
    }

    pub fn plot(&self, path: String) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(&path, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(self.name.clone(), ("Hack", 44, FontStyle::Bold).into_font())
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 60)
            .margin(20)
            .build_cartesian_2d(
                self.members[0]..self.members[self.members.len() - 1],
                0f64..1f64,
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        let color: Vec<&RGBColor> = vec![&RED, &BLUE, &GREEN];
        for i in 0..self.fuzzy_sets.len() {
            chart
                .draw_series(LineSeries::new(
                    self.members
                        .iter()
                        .zip(self.fuzzy_sets[i].membership.iter())
                        .map(|(x, y)| (*x, *y)),
                    color[i % color.len()],
                ))?
                .label(self.fuzzy_sets[i].name.clone())
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));
            // there're a problem with styling label
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degree_test() {
        let mut d = Domain::new(String::from("test"), 0f64, 10f64, 0.01f64);
        let f1 = trinagular(5f64, 0.8f64, 3f64);

        let s1 = FuzzySet::new(&d.members, f1, "f1".into());

        println!("{}", s1.degree_of(5.0f64));
    }

    #[test]
    fn basic_test() {
        let mut d = Domain::new(String::from("test"), 0f64, 10f64, 0.01f64);
        let f1 = trinagular(5f64, 0.8f64, 3f64);
        let f2 = trinagular(2f64, 1.0f64, 4f64);

        let s1 = FuzzySet::new(&d.members, f1, "f1".into());
        let s2 = FuzzySet::new(&d.members, f2, "f2".into());
        let s = s1.std_intersect(&s2, "f3".into());

        let s1 = d.add(s1);
        let s2 = d.add(s2);
        let s3 = d.add(s);

        d.plot("img/t.png".into()).unwrap();
    }
}
