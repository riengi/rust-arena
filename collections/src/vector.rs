// Basic vector with integers
pub fn basic_vector() {
    let v = vec![1, 2, 3];

    print!("Basic vector: ");
    for i in v {
        print!("{},", i);
    }
    println!();
}

// Vector containing various types stored in enum
pub fn various_types_vector() {
    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }
    impl Point {
        fn new(x: u32, y: u32) -> Self {
            Self { x, y }
        }
    }

    #[derive(Debug)]
    enum Type {
        Int(i32),
        UInt(u32),
        Str(String),
        Point(Point),
    }

    print!("Pseudo-mixed vector: ");

    let i = Type::Int(-32);
    let u = Type::UInt(32);
    let s = Type::Str(String::from("I'm String"));
    let p = Type::Point(Point::new(1, 1));

    let v = vec![i, u, s, p];

    for x in v {
        if let Type::Point(value) = x {
            print!("Point({},{})", value.x, value.y);
        } else {
            print!("{:?},", x);
        }
    }
    println!();
}
