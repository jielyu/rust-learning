use crate::List::{Cons, Nil};
use core::cell::RefCell;
fn main() {
    // 创建Box智能指针
    let b = Box::new(5);
    println!("b={}", b);

    // 创建递归结构
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list={:?}", list);

    // Box作为引用
    let x = 5;
    let y = Box::new(x);
    println!("*y={}", *y);

    // MyBox解引用
    let my_y = MyBox::new(x);
    println!("*my_y={}", *my_y);

    // Drop trait
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointer created");
    }

    //  提前调用Drop trait
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        println!("CustomSmartPointer created");
        std::mem::drop(c);
        println!("CustomSmartPointer dropped before the end of main");
    }

    // Rc多重所有权
    let a = Rc::new(RcList::RcCons(
        5,
        Rc::new(RcList::RcCons(10, Rc::new(RcList::RcNil))),
    ));
    let b = RcList::RcCons(3, Rc::clone(&a));
    let c = RcList::RcCons(4, Rc::clone(&a));
    println!("b={:?}", b);
    println!("c={:?}", c);

    // RefCell内部可变借用
    let c = RefCell::new(5);
    *c.borrow_mut() = 10;
    println!("c={:?}", c);
}

// 错误的定义递归结构
// enum List {
//     Cons(i32, List),
//     Nil,
// }

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

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

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use std::rc::Rc;
#[derive(Debug)]
enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}
