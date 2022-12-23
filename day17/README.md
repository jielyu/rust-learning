# 面向对象

## 面向对象语言的特性

* 使用 `pub` 控制暴露的结构
* 使用 `struct` `enum` 组织数据结构
* 使用 `impl` 绑定方法到数据结构上
* 使用 `trait` 定义接口实现集成，配合范型实现多态

## 使用trait对象存储不同类型的值

[code](rust_oop/)

* trait 对象必须保证对象安全，需要满足两条规则

> 方法的返回类型不能是 `Self`
> 方法中不包含任何范型参数

## 状态模式

* 状态模式可以使用 trait+范型 实现

[code](rust_state_pattern/)

* rust还可以使用不同类型实现不同状态之间的转换

[code](rust_type_pattern/)
