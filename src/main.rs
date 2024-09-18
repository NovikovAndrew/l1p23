fn main() {
    let p1 = geometry::Point::new(1.0, 10.0);
    let p2 = geometry::Point::new(-5.0, 20.0);

    println!("distance p1 and p2 is {}", p1.distance(&p2))
}

// mod чтобы скрыть поля структуры
mod geometry {
    pub struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // конструктор
        pub fn new(x: f64, y: f64) -> Self {
            Point {x, y}
        }

        // метод для нахождения расстояния между двумя точками
        pub fn distance(&self, p2: &Point) -> f64 {
            let dx = self.x - p2.x;
            let dy = self.y - p2.y;
            ((dx * dx) + (dy * dy)).sqrt()
        }
    }
}