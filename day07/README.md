# 包管理

## 模块和作用域

[code](restaurant/)

* 使用关键字 `mod` 定义模块

```rust
mod front_of_house {}
```

* 使用关键字 `pub` 对外暴露接口

```rust
pub mod hosting {}
```

* 使用关键字 `super` 跳转至父模块

```rust
fn tack_order() {
    super::hosting::add_to_waitlist();
}
```

* 结构体的字段默认是不公开的，公开需要加 `pub` 

```rust
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}
```

* 枚举类型的字段默认是公开的，只要枚举类型本身加 `pub` 就可以对外暴露

```rust
pub enum Appetizer {
    Soup,
    Salad,
}
```

* 使用 `use` 关键字导入命名空间

```rust
use crate::front_of_house::hosting;
fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

* 使用 `as` 关键字重命名导入的命名空间或者类型

```rust
use std::io::Result as IoResult;
fn function1() -> IoResult<()> {
    Ok(())
}
```

* 使用 `pub use` 重新导出名称

```rust
pub use back_of_house::Breakfast;
```

* 使用外部包需要在 `Cargo.toml` 文件的 `dependencies` 区域添加项目名字

```toml
[dependencies]
rand = "0.5.5"
```

在代码中使用时，用 `use` 导入需要用到的名称

```rust
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}
```

* 使用 `use` 可以进行路径嵌套

```rust
use std::{cmp::Ordering, io};
```

* 导入时可以使用通配符导入命名空间中所有的名称

```rust
use std::collections::*;
```

## 模块树与文件树

[code](restaurant_file/)

*  模块树与文件树可以进行一一对应

```
.
├── back_of_house.rs
├── front_of_house
│   ├── hosting.rs
│   └── serving.rs
├── front_of_house.rs
└── lib.rs
```