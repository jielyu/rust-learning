# Day 10 范型、trait与生命周期

## 范型数据类型

* 在函数中定义范型 

```rust
fn func_name<T>(x: T) -> T {
    x
}
```

* 在结构体中定义范型

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

如果实例化时给定的 `x, y` 类型不一致，将会报错

* 在枚举类型中定义范型

```rust
enum Result<T, E> {
    Ok(T),
    Err(e),
}
```

* 在方法中定义范型

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

* 范型在编译时使用单态化转换为特定代码，因此与具体类型的代码相比不会有任何速度上的差异

## trait

* 使用 `trait` 关键字定义trait

```rust
trait Summary {
    // 默认实现
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

* 为类型实现trait

```rust
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

* 使用trait作为参数

```rust
use std::fmt::Display;
fn trait_func(item: impl Summary + Display) {}
```

这是一种语法糖的写法，完整的写法如下：

```rust
fn trait_func2<T: Summary + Display>(item: T) {}
```

* 返回实现了trait的类型

```rust
fn return_trait() -> impl Summary {
    Tweet {
        username: String::from("horse_books"),
        content: String::from("of course, ..."),
        reply: false,
        retweet: false,
    }
}
```

* 使用trait实现有条件地实现方法

```rust
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("largest member: {}", self.x);
        } else {
            println!("largest member: {}", self.y);
        }
    }
}
```

## 生命周期

* 使用生命周期避免悬垂引用

* 数据的生命周期必须比对应引用的生命周期长

*  输入多个生命周期，输出值无法确定生命周期时需要进行显式生命周期范型标注

* 生命周期的标注语法使用 `'` 开始

```rust
&'a i32
&'a mut i32
```
* 使用范型在函数签名中标注生命周期

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

* 指定生命周期的方式往往取决于函数的具体功能， 以下代码是合法的，因为返回值的生命周期只与第一个参数有关

```rust
fn longest_x<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

* 当函数返回一个引用时，返回类型的生命周期参数必须要与其中一个参数的生命周期参数匹配

* 需要为结构体定义中的每一个引用添加生命周期

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

编译器自动标注生命周期的规则：

* 每个引用参数都有自己的生命周期
* 当只有一个输入生命周期参数时，这个生命周期会被赋予给所有输出生命周期参数
* 当拥有多个输入生命周期参数，而其中一个是 `&self` 或者 `&mut self` 时，self的生命周期会被赋予给所有输出生命周期参数

* 方法定义中需要标注生命周期

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

* 使用 `'static`  标注静态生命周期， 字符串的字面值生命周期都是 `'static`

```rust
let s: &'static str = "hello world";
```

## 同时使用范型、trait和生命周期

```rust
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```