fn main() {
    // 测试结构体的实例化
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 3,
    };
    println!("user={:#?}", user);

    // 简化初始化方法
    let email = String::from("someone@example.com");
    let username = String::from("someusername");
    let user = User {
        email,
        username,
        sign_in_count: 3,
        active: true,
    };
    println!("user={:#?}", user);

    // 使用其他实例创建新实例
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        ..user
    };
    println!("user2={:#?}", user2);

    // 具名元组
    let black = Color(0, 0, 0);
    println!("black={:#?}", black);

    // 空结构体
    let empty = EmptyType();
    println!("empty={:#?}", empty);

    // 定义方法
    let rect = Rect {
        width: 10,
        height: 5,
    };
    println!("rect.area = {}", rect.area());

    // 关联方法
    let square = Rect::square(3);

    // 多参数方法
    if rect.can_hold(&square) {
        println!("rect can hold square");
    } else {
        println!("rect can not hold square");
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct EmptyType();

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    // 定义方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 一个impl块可以实现多个方法
    fn length(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// 多个impl块
impl Rect {
    // 多参数函数
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}
