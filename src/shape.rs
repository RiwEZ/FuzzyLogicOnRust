pub trait Shape {
    fn function(&self, x: f64) -> f64;
}

pub struct Triangular {
    a: f64,
    b: f64,
    s: f64,
}

impl Shape for Triangular {
    fn function(&self, x: f64) -> f64 {
        if (self.a - self.s) <= x && x <= (self.a + self.s) {
            return self.b * (1.0 - (x - self.a).abs() / self.s);
        }
        0.0
    }
}

pub fn trinagular(a: f64, b: f64, s: f64) -> Triangular {
    Triangular { a, b, s }
}