fn main() {
    // 测试枚举类型
    let home = IpAddr::V4(String::from("127.0.0.1"));
    println!("home={:#?}", home);

    // 枚举绑定方法
    home.print();

    // Option 实现空安全机制
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // 报错
    println!("y={:#?}", y);

    // match匹配
    let penny = value_in_cents(Coin::Penny);
    let quarter = value_in_cents(Coin::Quarter(101));
    println!("penny: {}, quarter: {}", penny, quarter);

    // if let
    let coin = Coin::Penny;
    if let Coin::Quarter(value) = coin {
        println!("quarter index: {}", value);
    } else {
        println!("not quarter");
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn print(&self) {
        println!("{:#?}", self);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(i8),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Quarter(value) => {
            println!("quarter index: {}", value);
            25
        }
        _ => 10,
    }
}
