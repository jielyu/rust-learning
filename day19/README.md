# 高级特性

## 不安全Rust

[code](rust_unsafe/)

不安全Rust允许四种超能力

### 解引用裸指针

```rust
fn test_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        *r2 += 1;
        println!("r2 is: {}", *r2);
    }
}
```

Output:

```
r1 is: 5
r2 is: 6
```

* `*const` 定义只读指针，`*mut` 定义可写指针

* 使用 `extern` 函数调用外部代码，调用该函数需要在 `unsafe` 区域内

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}
fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

* 使用 `[no_mangle]` 和 `extern` 修饰对其他语言暴露的接口

```rust
#[no_mangle]
  pub extern "C" fn call_from_c() {
      println!("Just called a Rust function from C!");
  }
```

### 调用不安全的函数或者方法

```rust
use std::slice;
fn split_at_mut(s: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = s.len();
    let ptr = s.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}
fn test_unsafe_function() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    println!("a={:?}, b={:?}", a, b);
}
```

Output:

```
a=[1, 2, 3], b=[4, 5, 6]
```

### 访问或修改可变的静态变量

```rust
static mut COUNTER: u32 = 0;
fn test_mut_static() {
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }
}
```

Output:

```
COUNTER: 1
```

### 实现不安全的trait

* 使用 `unsafe` 声明不安全的trait，同时该trait的实现也应该用 `unsafe` 修饰

```rust
unsafe trait Foo {}
unsafe impl Foo for i32 {}
```

##  高级trait

### 在trait中使用关联类型指定占位类型

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Item 就是关联类型， 需要在实现时指定

```rust
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        ...
    }
}
```

占位类型限定只能为某个具体类型实现一次该trait

### 默认范型参数和运算符重载

* 可以实现 `std::ops` 中列举出的那些trait来重载运算符

```rust
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
```

Output:

```
Point { x: 3, y: 3 }
```

* 默认范型参数使用`<T=U>` 实现

```rust
trait Add<RHS=Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}
```

不使用默认范型参数的情况下，可以指定类型

```rust
use std::ops::Add;
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

### 完全限定语法消除歧义

```rust
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
```

Output:

```
This is your captain speaking.
Up!
*waving arms furiously*
```

* 使用 `as` 完全限定关联函数的调用 

```rust
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
```

Output:

```
A baby dog is called a puppy
```

### 在trait中附带另一个trait功能的超trait

```rust
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
```

Output:

```
************
*          *
* (10, 20) *
*          *
************
```

### newtype 模式在外部类型实现外部trait

```rust
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

```

为 `Vec<String>` 类型实现 `Display` 方法， 并且保留已有的所有方法

## 高级类型

### newtype模式实现类型安全和抽象

* newtype 另一个用途时为类型的某些细节提供抽象能力，新类型可以暴露一个与内部私有类型不同的api，从而控制用户访问的功能

* 使用类型别名创建同义类型 

```rust
type Kilometers = i32;
let x: i32 = 5;
let y: Kilometers = 5;
println!("x + y = {}", x + y);
```

* 类型别名最主要的用途时减少代码字符的重复

```rust
type Thunk = Box<dyn Fn() + Send + 'static>;
let f: Thunk = Box::new(|| println!("hi"));
```

### 用不返回的 Never 类型

* 以 `!` 作为返回值类型，即为 空类型
* 返回空类型的几个语句： `continue, panic!, loop`

### 动态大小类型与Sized trait

* `?Sized` 约束表达了与Sized相反的含义，只能用在 `Sized` 上

```rust
fn generic<T: ?Sized>(t: &T){}
```

将t参数的类型由T修改为了&T, 因为 类型可能不是Sized的，所以我们需要将它放置在某种指针的后面

## 高级函数与闭包

### 函数指针

* `fn` 是一个类型而不是trait
* 可以使用函数指针作为参数

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string)
    .collect();
```

## 返回闭包

* 由于闭包使用trait来进行表达，因此无法在函数中直接返回一个闭包
* 使用 `Box` 返回闭包

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

## 宏

### 宏与函数的差别

* 函数在定义签名时必须声明自己参数的个数与类型，而宏则能够 处理可变数量的参数

* 由于编译器会在解释代码前展开宏，所以宏可以被用来 执行某些较为特殊的任务，比如为类型实现trait等

* 宏的定义 要比函数定义复杂得多，因为你需要编写的是用于生成Rust代码的 Rust代码

* 当你在某个文件中调用宏 时，你必须提前 定义宏或将宏引入当前作用域中，而函数则可以在任 意位置定义并在任意位置使用

### 用于通用元编程的 macro_rules! 声明宏

```rust
#[macro_export] 
macro_rules! vec {
    ( $( $x:expr ),* ) => { 
        {
            let mut temp_vec = Vec::new(); 
            $(temp_vec.push($x); )*
            temp_vec 
        }
    }; 
}
```

* 代码中标注的#[macro_export] 意味着这个宏会在它所处的包 被引入作用域后可用
* 使用了macro_rules! 及不带感 叹号的名称来开始定义宏
* $()中的$x:expr可以匹配任意的Rust 表达式，并将其命名为$x
* $()之后的逗号意味着一个可能的字面逗号分隔符会出现在捕获代 码的后面
* 而逗号后的*则意味着这个模式能够匹配零个或多个*之前 的东西
* 它会为模式中匹配到的每一个$()生成$()*中对应的temp_vec.push()代码

详细参考： the little book of rust macros

### 基于属性创建代码的过程宏

过程宏会接收并操作输入的Rust代码，并生成另外一些 Rust代码作为结果

* 当创建过程宏时，宏的定义必须单独放在它们自己的包中，并使 用特殊的包类型

```rust
use proc_macro;
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

* TokenStream类型在proc_macro包(Rust 自带)中定义，表示一段标记序列
* 需要 被宏处理的源代码组成了输入的TokenStream，而宏生成的代码则组成 了输出的TokenStream
* 函数附带的属性决定了我们究竟创建的是哪一 种过程宏
* 同一个包中可以拥有多种不同类型的过程宏

### 编写一个自定义的derive宏

[hello_macro](hello_macro/)
[调用例子](rust_macro/)

### 属性宏

属性宏与自定义派生宏类似，它们允许你创建新的属性，而不是 为derive属性生成代码

```rust
#[route(GET, "/")]
fn index() {}
```

### 函数宏

函数宏可以定义出类似于函数调用的宏，但它们远比普通函数更 为灵活

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {...}
```