# 通用集合类型

## 动态数组 vector

[code](rust_vector/)

* 使用 `Vec` 创建动态数组

```rust
let mut v: Vec<i32> = Vec::new();
```

* 使用 `push` 方法添加元素

```rust
v.push(5);
```

* 在离开作用域之后动态数组会自动销毁，其元素也会销毁

* 使用下标读取动态数组的元素

```rust
println!("v[1]={}", v[1]);
```

* 使用 `get` 方法读取动态数组的元素

```rust
match v.get(2) {
    Some(value) => {
        println!("value is {}", value);
    }
    None => println!("no that element"),
}
```

* 使用 `for` 循环遍历动态数组

```rust
let mut sum = 0;
for i in v {
    sum += i;
}
println!("sum={}", sum);
```

* 使用枚举类型在动态数组中存储不同类型的值

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
```

## String

[code](rust_string/)

* 创建新的字符串String

```rust
let mut s = String::new();
let data = "initial contents";
let s = data.to_string();
let s = "initial contents".to_string();
```

* 更新字符串

```rust
let mut s = String::from("hello");
s.push_str(" world"); // 追加字符串
s.push('!'); // 追加字符
println!("s={}", s);
```

* format字符串

需要导入format 

```rust
use std::fmt::format;
```

```rust
let s2 = String::from("hello rust");
let s3 = format!("{}, {}", s, s2);
println!("s3={}", s3);
```

* 字符串不支持索引
* 字符串切片，必须小心切片范围不在字符边界导致崩溃

```rust
let s = &s[0..5];
println!("s={}", s);
```

* 遍历字符串

```rust
let s = String::from("你好");
for c in s.chars() {
    println!("c={}", c);
}
for b in s.bytes() {
    println!("b={}", b);
}
```

Output:

```
c=你
c=好
b=228
b=189
b=160
b=229
b=165
b=189
```

## 哈希映射

[code](rust_hash/)

* 创建HashMap实例

```rust
let mut scores = HashMap::new();
```
    

* 添加元素

```rust
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
println!("scores={:#?}", scores);
```

* 对于实现 `Copy trait` 的类型，他们的值会被复制到哈希映射中，比如 `i32`
* 对于 `String` 这种持有所有权的值，就会被 `move` 到哈希映射中

* 访问HashMap中的值

```rust
println!("scores['Blue']={}", scores["Blue"]);
```

* 遍历HashMap

```rust
for (key, value) in &scores {
    println!("key={}, value={}", key, value);
}
```

* 更新HashMap
    
```rust
scores.insert(String::from("Blue"), 25);
```
