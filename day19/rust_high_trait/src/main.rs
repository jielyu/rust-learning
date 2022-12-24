fn main() {
    test_override_operator();
    test_limit();
    test_association_func();
    test_super_trait();
    test_newtype();
}

// 实现trait冲在运算符
use std::ops::Add;
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn test_override_operator() {
    println!("{:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });
}

// 指定范型参数，而不是使用默认参数
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// 完全限定语法
trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn test_limit() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

// 限定关联函数
trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn test_association_func() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// trait附带trait
use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
struct SuperPoint {
    x: i32,
    y: i32,
}

impl OutlinePrint for SuperPoint {}

impl fmt::Display for SuperPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn test_super_trait() {
    let point = SuperPoint { x: 10, y: 20 };
    point.outline_print();
}

// newtype 模式
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

use std::ops::Deref;
impl Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        println!("deref");
        &self.0
    }
}

fn test_newtype() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}, len = {}", w, w.len());
}
