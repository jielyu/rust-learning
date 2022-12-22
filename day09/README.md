# Day 09 错误处理

[code](rust_error/)

## 不可恢复错误

* 使用 `panic!` 触发不可恢复错误

```rust
fn raise_panic() {
    panic!("crash and burn");
}
```

Output:

```
thread 'main' panicked at 'crash and burn', src/main.rs:6:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

* 在 `Cargo.toml` 文件的 `profile.release` 区域配置 `panic` 行为

```toml
[profile.release]
panic = 'abort'
```

在 release 模式下运行 `cargo run --release` 输出

```
thread 'main' panicked at 'crash and burn', src/main.rs:6:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[1]    17508 abort      cargo run --release
```

* 添加 `RUST_BACKTRACE=1` 打印堆栈 `RUST_BACKTRACE=1 cargo run` 输出

```
thread 'main' panicked at 'crash and burn', src/main.rs:6:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
   2: rust_error::raise_panic
             at ./src/main.rs:6:5
   3: rust_error::main
             at ./src/main.rs:2:5
   4: core::ops::function::FnOnce::call_once
             at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

## 可恢复错误

* 使用 `Result<T, E>` 枚举类型返回可恢复错误

```rust
use std::fs::File;
fn open_error() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open file, error {:?}", error);
        }
    };
}
```

Output:

```
thread 'main' panicked at 'Failed to open file, error Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:16:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

* 匹配不同的错误，尽可能修复已知的问题

```rust
use std::io::ErrorKind;
fn open_error_match() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file, error {:?}", error),
            },
            other_error => panic!("Failed to open file, error {:?}", other_error),
        },
    };
}
```

在打开文件报错后，检查到是 `NotFound` 错误，就尝试创建该文件。

* `unwrap` 和 `expect` 简化失败时触发 `panic` 的处理

```rust
fn test_unwrap() {
    let f = File::open("hello.txt").unwrap();
}
```

Output:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:39:37
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

```rust
fn test_expect() {
    let f = File::open("hello.txt").expect("Failed to open file");
}
```

Output:

```
thread 'main' panicked at 'Failed to open file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:44:37
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

* 使用 `?` 传播错误，只能在返回值为 `Result` 类型的函数中使用

```rust
fn test_broadcast() {
    let f = error_broadcast();
    match f {
        Err(error) => println!("get error, err={}", error),
        Ok(f) => println!("get file successfully"),
    }
}

use std::io;
fn error_broadcast() -> Result<File, io::Error> {
    let f = File::open("hello.txt")?;
    Ok(f)
}
```

Output:

```
get error, err=No such file or directory (os error 2)
```

* `?` 可以链式调用，中途一旦失败就会返回错误

## 要不要使用panic

以下是应当使用 panic 的情形：

* 在示例、原型和测试中
* 当你比编译器拥有更多信息，确认不会发生错误，可以使用 `panic` 或者 `unwrap`
* 错误导致程序出现未知的损坏
* 在随后的代码中无法修复错误导致的损坏
* 没有合适的方法将损坏状态编码到所使用的类型中
* 可以在对象创建时合适地使用 panic 限制实例在预期范围之内
