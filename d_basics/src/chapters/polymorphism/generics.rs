pub fn deduplication_with_generics() {
    in_functions();
    in_structs();
    in_methods();
}

fn in_functions() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

fn in_structs() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("int struct: {:?}", integer);
    println!("float struct: {:?}", float);

    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!("both_integer struct: {:?}", both_integer);
    println!("both_float struct: {:?}", both_float);
    println!("integer_and_float struct: {:?}", integer_and_float);
}

fn in_methods() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // p.distance_from_origin();

    let p = Point { x: 1.0, y: 4.0 };
    println!("distance_from_origin = {}", p.distance_from_origin());
}
