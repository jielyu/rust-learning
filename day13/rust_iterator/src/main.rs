fn main() {
    test_vec_iter();
    test_map_iter();
    test_closure_iter();
    test_counter_iter();
}

fn test_vec_iter() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    println!("next:{:?}", v1_iter.next());
    println!("next:{:?}", v1_iter.next());
    println!("next:{:?}", v1_iter.next());
    println!("next:{:?}", v1_iter.next());
}

fn test_map_iter() {
    let v1 = vec![1, 2, 3];
    for v in v1.iter().map(|x| x + 1) {
        println!("v={}", v);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn test_closure_iter() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    println!("in_my_size: {:?}", in_my_size);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn test_counter_iter() {
    let mut counter = Counter::new();
    println!("counter.next: {:?}", counter.next());
    println!("counter.next: {:?}", counter.next());
    println!("counter.next: {:?}", counter.next());
    println!("counter.next: {:?}", counter.next());
    println!("counter.next: {:?}", counter.next());
    println!("counter.next: {:?}", counter.next());
}
