# 枚举

[code](rust_enum/)

## 定义枚举

```
enum EnumName {
    FieldName,
}
```

### 枚举关联数据

```rust
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
```

### 枚举绑定函数

```rust
impl IpAddr {
    fn print(&self) {
        println!("{:#?}", self);
    }
}
```

### Option空安全

```rust
enum Option<T> {
    Some<T>,
    None,
}
```

## match运算符

* match 可以匹配的模式可由字面量、变量名、通配符和许多其他的东西组成

### 绑定值模式

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(i8),
}
```

匹配时可以获取绑定值

```rust
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
```

* 匹配必须穷举所有可能
* 使用 `_` 通配符对其他进行匹配
* 匹配 `Option<T>` 可以对空进行处理

 

## if let 控制流

* 可以认为 `if let` 是对 `match` 在简单模式时的语法糖

```rust
let coin = Coin::Penny;
    if let Coin::Quarter(value) = coin {
        println!("quarter index: {}", value);
    } else {
        println!("not quarter");
    }
```
