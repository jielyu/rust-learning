fn main() {
    let point = Point { x: 10, y: 20 };
    println!("point.x()={}", point.x())
}

fn func_name<T>(x: T) -> T {
    x
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
