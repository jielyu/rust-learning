use core::time::Duration;
use std::thread;
fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let x = vec![1, 2, 3];
    let equal_to_x = move |z: Vec<u32>| z == x;
    // println!("x={:?}", x); // 报错，发生可变借用
    let y = vec![1, 2, 3];
    println!("check equal: {}", equal_to_x(y));
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
