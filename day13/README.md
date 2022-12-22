# 迭代器与闭包

## 闭包

能捕获环境的匿名函数

* 闭包使用 `||` 包裹参数，定义方式如下

```rust
let closure = |x| {x+1};
```

*  闭包不需要类型标注，编译器会自动推导，也可以显式标注

```rust
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly ...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

* 多次使用闭包传入不同类型的参数会报错

* 闭包捕获环境的三种类型：

> FnOnce 从环境转移值的所有权到闭包内部，只能调用一次，使用关键字 `move` 修饰

```rust
let x = vec![1, 2, 3];
let equal_to_x = move |z: Vec<u32>| z == x;
// println!("x={:?}", x); // 报错，发生可变借用
let y = vec![1, 2, 3];
println!("check equal: {}", equal_to_x(y));
```

> FnMut 从环境可变得借用值
> Fn 从环境不可变得借用值

## 迭代器

迭代器模式允许依次为序列中的每一个元素执行某些任务

* 迭代器都实现了 `Iterator` trait 和 `next` 方法

* 调用 `map` 迭代器适配器创建新的迭代器

```rust
fn test_map_iter() {
    let v1 = vec![1, 2, 3];
    for v in v1.iter().map(|x| x + 1) {
        println!("v={}", v);
    }
}
```

Output:

```
v=2
v=3
v=4
```

* 使用闭包捕获环境配合迭代器

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
```

* 自定义迭代器的实现

```rust
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
```

* 迭代器并不比循环的性能差，可以放心使用