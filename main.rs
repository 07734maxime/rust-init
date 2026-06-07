#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32, y: f32
}

impl Point {
    fn dot_product(p1: Point, p2: Point) -> f32 {
        p1.x * p2.x + p1.y * p2.y
    }

    fn dist(p: Point) -> f32 {
        (p.x*p.x + p.y*p.y).sqrt()
    }
    
    fn angle_between(p1: Point, p2: Point) -> f32 {
        (Self::dot_product(p1, p2)/(Self::dist(p1)*Self::dist(p2))).acos()
    }
}

fn main() {
    let p1 = Point { x: 6.0, y: 2.0 };
    let p2 = Point { x: -3.0, y: 2.0 };

    println!("L'angle entre les deux vecteurs est : {}", Point::angle_between(p1,p2))
}
