pub mod shape;
use crate::shape::Shape;
use plotters::prelude::*;
use shape::trinagular;
use rand::prelude::*;

// GOAL: Fuzzy Inference System to use with security trading
// RULE_1: IF RSI IS LOW AND MA IS LOW THEN BUY
struct Domain {
    name: String,
    x: Vec<f64>,
    fuzzy_sets: Vec<(String, Vec<f64>)>
}

impl Domain {
    /// ## Panics
    /// * Panic if `end` >= `start`
    /// * Panic if `res` is not in range (0, 1)
    pub fn new(name: String, start: f64, end: f64, res: f64) -> Domain {
        if end < start {
            panic!("end can not be less than start");
        } else if res <= 0.0 && res >= 1.0 {
            panic!("res should be in range (0, 1)");
        }

        let mut x: Vec<f64> = vec![];
        let interval = (end - start) * res;
        let mut n = start;
        while n <= end {
            x.push(n);
            n += interval;
        }

        Domain { name, x, fuzzy_sets: Vec::<(String, Vec<f64>)>::new() }
    }

    pub fn add<T: Shape>(&mut self, fuzzy_f: T, name: String) {
        let mut membership: Vec<f64> = vec![];
        for i in 0..self.x.len() {
            membership.push(fuzzy_f.function(self.x[i]));
        }
        self.fuzzy_sets.push((name, membership));
    }

    pub fn plot(&self, path: String) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(&path, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(self.name.clone(), ("Hack", 44, FontStyle::Bold).into_font())
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 60)
            .margin(20)
            .build_cartesian_2d(self.x[0]..self.x[self.x.len() - 1], 0f64..1f64)?;
        
        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        let color: Vec<&RGBColor> = vec![&RED, &BLUE, &GREEN];
        for i in 0..self.fuzzy_sets.len() {
            chart.draw_series(
                LineSeries::new(
                    self.x.iter().zip(self.fuzzy_sets[i].1.iter()).map(|(x, y)| (*x, *y)),
                    color[i % color.len()],
                )
            )?
            .label(self.fuzzy_sets[i].0.clone())
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
    let mut d = Domain::new(String::from("test"), 0f64, 100f64, 0.01);
    let f1 = trinagular(50f64, 0.8f64, 40f64);
    let f2 = trinagular(25f64, 1.0f64, 25f64);
    
    d.add(f1, String::from("f1"));
    d.add(f2, String::from("f2"));

    d.plot(String::from("img/t.png"))?;
    Ok(())
}
