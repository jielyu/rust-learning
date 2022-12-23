# 模式匹配

## 所有可以使用模式的场合

* match分支

```rust
match result {
    Ok{v} => v,
    Err(error)=>println!("error: {}", error),
}
```

* if let 条件表达式

```rust
if let Some(color) = favorite_color {
    println!("Using your favorite color, {}, as the background", color);
}
```

* while let 条件循环

```rust
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

* for 循环

```rust
let v = vec!['a', 'b', 'c'];
for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

* let 语句

```rust
let x = 5;
let (x, y, z) = (1, 2, 3);
```

* 函数参数

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
```

## 可失败性

* 不可失败的模式可以匹配任何传入的值， 比如 `let x = 5;`

* 函数参数、let语句和for循环只接受不可失败的模式

* if let和while let表达式只接收可失败的模式

## 模式语法

* 可以直接使用模式匹配字面量

```rust
let x = 1;
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

* 命名变量是一种可以匹配任何值的不可失败模式

```rust
fn main() {
let x = Some(5); 
let y = 10;
match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y), 
    _ => println!("Default case, x = {:?}", x),
}
println!("at the end: x = {:?}, y = {:?}", x, y); }
```

* 使用 `|` 表示“或”的意思，用来一次性匹配多种模式

```rust
let x = 1;
match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

* 使用 `...` 来匹配区间值

```rust
let x = 5;
match x {
    1 ... 5 => println!("one through five"),
    _ => println!("something else"),
}
```

* 使用结构模式来分解结构体、枚举、元组或者引用

```rust
struct Point {
    x: i32,
y: i32, }
fn main() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

* 用于解构枚举类型的模式必须要对应枚举定义中存储数据的方式

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}
```

* 使用 `_` 忽略模式中的值

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}
fn main() {
    foo(3, 4);
}
```

* 以下划线开始的变量可以避免触发变量未使用的警告
* `_x` 语法仍然将值绑定到变量上，而`_` 则完全不会绑定
* `..` 语法会自动展开并填充任意多个所需的值

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
} }
```

* 使用匹配守卫(分支模式后的if条件语句)可以表达更复杂的意图

```rust
let num = Some(4);
match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

* `@` 运算符允许在测试一个值是否匹配模式的同时创建存储该值的变量

```rust
enum Message {
    Hello { id: i32 },
}
let msg = Message::Hello { id: 5 };
match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```