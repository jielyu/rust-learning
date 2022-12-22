# 测试

## 如何编写测试

* 测试模块由 `#[cfg(test)]` 修饰， 测试函数由 `#[test]` 修饰
* `assert!`, `assert_eq!`, `assert_ne!` 用于断言

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_ne!(result, 3);
        // 测试失败的情况
        assert!(result != 4, "failed to calculate {}", result);
    }
}
```

* `#[should_panic]` 断言发生错误

```rust
#[test]
#[should_panic]
fn test_panic() {
    panic!("test panic");
}
```

* `should_panic` 添加参数用于测试错误信息是否包含指定内容

```rust
#[test]
#[should_panic(expected = "test")]
fn test_panic2() {
    panic!("test panic");
}
```

* 使用 `Result` 编写测试

```rust
#[test]
fn test_result() -> Result<(), String> {
    if 2 + 3 == 4 {
        Ok(())
    } else {
        Err(String::from("check calculation failed"))
    }
}
```

## 控制测试运行方式

* 默认使用多线程运行多个测试，可以使用参数指定为单线程

```
cargo test -- --test-threads=1
```

* 默认测试函数的打印被拦截，可以设置参数禁用输出截获功能

```
cargo test -- --nocapture
```

* 可以根据指定名称匹配运行部分测试

只运行一个测试就写出函数的全名

```
cargo test it_works
```

运行一系列测试就写出这个系列共同的前缀

```
cargo test test_
```

* 可以使用 `#[ignore]` 忽略一些测试

```rust
#[test]
#[ignore]
fn test_ignore() {
    assert!(2 + 3 == 5)
}
```

* 也可以使用命令参数只运行这些被忽略的测试

```
cargo test -- --ignored
```

## 测试的组织结构

* 每个源代码文件中都新建一个 `tests` 模块来存放测试函数，并使用 `#[cfg(test)]` 进行标注

标注的目的是指在 `cargo test` 命令时编译运行该模块，在 `cargo build` 时不参与编译

* rust允许测试私有函数

* 集成测试都放在 `tests` 目录下

`tests/integration_test.rs`

```rust
use adder;

#[test]
fn check_add() {
    assert!(4 == adder::add(2, 3));
}
```

执行以下指令运行

```
cargo test --test integration_test
```

在单元测试存在失败的情况下， `cargo test` 不会运行集成测试，就可以使用这种指定运行的方式

* 如果需要使用一些公共的测试模块，可以放在 `tests/common` 目录下，然后在集成测试中导入

