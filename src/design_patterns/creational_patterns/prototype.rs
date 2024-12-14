#[derive(Clone)]
pub struct Circle {
    pub x: u32,
    pub y: u32,
    pub r: u32,
}

#[test]
fn test_prototype() {
    let circle1 = Circle {
        x: 10,
        y: 15,
        r: 10,
    };

    let mut circle2 = circle1.clone();
    circle2.r = 77;

    println!("Circle 1: {}, {}, {}", circle1.x, circle1.y, circle1.r);
    println!("Circle 2: {}, {}, {}", circle2.x, circle2.y, circle2.r);
}
