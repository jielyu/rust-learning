# 智能指针

## Box<T>用于在堆上分配内存

* 使用 `Box::new()` 创建Box智能指针，并获得所指向值的所有权
* 原始递归结构由于无法确定大小导致编译报错，可以使用智能指针解决

报错的定义：

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```

使用智能指针解决编译问题后如下：

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

实例化递归结构的示例：

```rust
let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list={:?}", list);
```

Output:

```
list=Cons(1, Cons(2, Cons(3, Nil)))
```

## Deref trait

实现 `Deref` trait 使我们可以自定义解引用运算符 `*` 的行为

*  将 `Box<T>` 当作引用操作

```rust
let x = 5;
let y = Box::new(x);
println!("*y={}", *y);
```

* 定义可以解引用的自己的智能指针

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

```

* 当类型 T 实现 `Deref` trait 时，解引用转换能将 T 的引用转换为 T 经过 Deref 操作后生成的引用
* 当我们将引用作为参数传递给函数或者方法时，如果传入的类型与参数类型不一致，解引用转换就会自动发生。
* 当 `T: Deref<Target=U>` 时， 允许 `&T` 转换为 `&U`
* 当 `T: DerefMut<Target=U>` 时，允许 `&mut T` 转换为 `&mut U`
* 当 `T: Deref<Target=U>` 时，允许 `&mut T` 转换为 `& U`

## Drop trait

Drop trait 允许我们在变量离开作用域时执行某些自定义的操作，比如释放文件、网络连接等资源

实现 Drop trait 的例子如下：

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```

调用例子

```rust
{
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created");
}
```

Output:

```
CustomSmartPointer created
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

* drop 函数在变量离开作用域时自动调用，调用顺序为先进后出的栈弹出顺序

* 使用 std::mem::drop 提前调用

```rust
{
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer created");
    std::mem::drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}
```

Output:

```
CustomSmartPointer created
Dropping CustomSmartPointer with data `my stuff`!
CustomSmartPointer dropped before the end of main
```

## Rc<T> 智能指针

Rc<T>是一种基于引用计数的智能指针，从而支持多重所有权

*  `Rc<T>` 只能单线程使用

* 使用  `Rc<T>` 定义多个头部的链表

```rust
use std::rc::Rc;
#[derive(Debug)]
enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}
```

一个共用部分元素的两头部链表示例如下：

```rust
let a = Rc::new(RcList::RcCons(
    5,
    Rc::new(RcList::RcCons(10, Rc::new(RcList::RcNil))),
));
let b = RcList::RcCons(3, Rc::clone(&a));
let c = RcList::RcCons(4, Rc::clone(&a));
println!("b={:?}", b);
println!("c={:?}", c);
```

Output:

```
b=RcCons(3, RcCons(5, RcCons(10, RcNil)))
c=RcCons(4, RcCons(5, RcCons(10, RcNil)))
```
* `Rc::clone` 会增加引用计数，而不是深度拷贝值

## RefCell<T>

RefCell<T> 是使用了内部可变模式的类型，允许在持有不可变引用的前提下对数据进行修改

* `RefCell<T>` 代表了其持有数据的唯一所有权
* `RefCell<T>` 只会在运行时检查借用规则，如果违反，则触发 panic 终止程序

```rust
let c = RefCell::new(5);
*c.borrow_mut() = 10;
println!("c={:?}", c);
```

Output:

```
let c = RefCell::new(5);
*c.borrow_mut() = 10;
println!("c={:?}", c);
```

* 将 `Rc<T>` 与 `RefCell<T>` 结合使用可以实现一个拥有多重所有权的内部可变数据

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil, 
}
```

## 循环引用

* `Rc<T>` 与 `RefCell<T>` 一起使用时，可能会形成循环引用，从而造成内存泄漏

* 使用 `Weak<T>` 代替 `Rc<T>` 可以避免循环引用

```rust
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn test_weak_ref() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

* `Rc::downgrade` 只增加弱引用计数，该计数不为零不影响释放
* `Weak<T>` 在使用时需要调用 `upgrade` 操作，该操作返回 `Option` 枚举类型保证空安全
