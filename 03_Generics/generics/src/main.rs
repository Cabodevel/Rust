fn main() {
    let opt: Option<i32> = Option::Some(32);

    let generic_point:GenericPoint<f64, String> = GenericPoint {
        x: 9.21,
        y: String::from("esto es un string")
    };

    let point:Point = Point { x: 1, y: 2 };


    println!("option is {}", Option::unwrap_or_default(opt));
    println!("Point is: x: {}, y: {}", generic_point.x, generic_point.y);
    point.show_values()
}

struct Point {
    x:i32,
    y:i32
}

impl Point {
    fn show_values(&self){
        println!("Values are x: {}, y: {}", self.x, self.y)
    }
}

struct GenericPoint<T, U> {
    x:T,
    y:U
}
