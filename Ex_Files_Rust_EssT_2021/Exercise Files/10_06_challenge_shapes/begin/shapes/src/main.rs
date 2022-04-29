struct Rectangle {
    w: f64,
    h: f64,
}

trait Shape {
    fn new(a: f64, b: f64) -> Rectangle;
    fn get_area(&self) -> f64;
    fn scale(&mut self, a: f64);
}

impl Shape for Rectangle {
    fn new(a: f64, b: f64) -> Rectangle {
        return Rectangle { w: a, h: b };
    }

    fn scale(&mut self, a: f64) {
        self.w *= a;
        self.h *= a;
    }

    fn get_area(&self) -> f64 {
        self.h * self.w
    }
}

fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}
