# Day 05 结构体

[code](rust_struct/)

## 定义和实例化

* 结构体使用 `struct` 关键字定义

```rust
struct StructName {
    filed_name: TypeName,
    ...
}
```

* 变量名与字段名相同时可以简化

```rust
let email = String::from("someone@example.com");
let username = String::from("someusername");
let user = User {
    email,
    username,
    sign_in_count: 3,
    active: true,
};
```

* 使用其他实例创建新实例，只更新需要修改的字段

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername"),
    ..user
};
```

* 使用struct定义具名元组

```rust
struct Color(i32, i32, i32);
```

* 定义空结构题

```rust
struct EmptyType();
```

* 派生trait增加实用功能

```rust
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

## 方法

在矩形 `Rect` 结构体上绑定方法

```rust
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}
```

### 方法定义格式 

```rust
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### 定义多个参数的方法

```rust
impl Rect {
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

### 定义关联函数

```rust
impl Rect{
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}
```

* 对于同一个结构体，可以使用多个 `impl` 块