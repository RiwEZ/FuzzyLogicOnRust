pub mod shape;
use crate::shape::Shape;
use plotters::prelude::*;
use shape::trinagular;
use rand::prelude::*;

#[derive(Debug)]
struct FuzzySet {
    pub name: String,
    pub universe: Vec<f64>, // universe of discourse that own this set
    pub membership: Vec<f64>
}

impl FuzzySet {
    pub fn new<T: Shape>(domain: &Domain, fuzzy_f: T, name: &str) -> FuzzySet {
        let mut membership: Vec<f64> = vec![];
        for i in 0..domain.members.len() {
            membership.push(fuzzy_f.function(domain.members[i]));
        }
        FuzzySet {name: name.to_string(), universe: domain.members.clone(), membership}
    }

    pub fn std_union(&self, set: &FuzzySet, name: &str) -> FuzzySet {
        // check if domain is equal or not?
        if self.universe != set.universe {
            panic!("domain needs to be equal");
        }

        // if equal
        let mut membership: Vec<f64> = vec![]; 
        for i in 0..self.membership.len() {
            membership.push(self.membership[i].max(set.membership[i]));
        }
        FuzzySet { name: name.to_string(), universe: self.universe.clone(), membership }
    }

    pub fn std_intersect(&self, set: &FuzzySet, name: &str) -> FuzzySet {
        // check if domain is equal or not?
        if self.universe != set.universe {
            panic!("domain needs to be equal");
        }

        // if equal
        let mut membership: Vec<f64> = vec![]; 
        for i in 0..self.membership.len() {
            membership.push(self.membership[i].min(set.membership[i]));
        }
        FuzzySet { name: name.to_string(), universe: self.universe.clone(), membership }
    }
}


// GOAL: Fuzzy Inference System to use with security trading
// RULE_1: IF RSI IS LOW AND MA IS LOW THEN BUY
struct Domain {
    pub name: String,
    pub members: Vec<f64>,
    pub fuzzy_sets: Vec<FuzzySet>
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
        let interval = (end - start) * res;
        let mut n = start;
        while n <= end {
            members.push(n);
            n += interval;
        }

        Domain { name, members, fuzzy_sets: Vec::<FuzzySet>::new() }
    }

    pub fn add(&mut self, set: FuzzySet) {
        self.fuzzy_sets.push(set);
    }

    pub fn plot(&self, path: String) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(&path, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(self.name.clone(), ("Hack", 44, FontStyle::Bold).into_font())
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 60)
            .margin(20)
            .build_cartesian_2d(self.members[0]..self.members[self.members.len() - 1], 0f64..1f64)?;
        
        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        let color: Vec<&RGBColor> = vec![&RED, &BLUE, &GREEN];
        for i in 0..self.fuzzy_sets.len() {
            chart.draw_series(
                LineSeries::new(
                    self.members.iter().zip(self.fuzzy_sets[i].membership.iter()).map(|(x, y)| (*x, *y)),
                    color[i % color.len()],
                )
            )?
            .label(self.fuzzy_sets[i].name.clone())
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK)); // there're a problem with styling label
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut d = Domain::new(String::from("test"), 0f64, 10f64, 0.01f64);
    let f1 = trinagular(5f64, 0.8f64, 3f64);
    let f2 = trinagular(2f64, 1.0f64, 4f64);

    let s1 = FuzzySet::new(&d, f1, "f1");
    let s2 = FuzzySet::new(&d, f2, "f2");
    let s = s1.std_intersect(&s2, "f3");
    println!("{:?}", s1.membership);
    println!("{:?}", s2.membership);
    println!("{:?}", s.membership);

    d.add(s1);
    d.add(s2);
    d.add(s);

    d.plot(String::from("img/t.png"))?;
    Ok(())
}
