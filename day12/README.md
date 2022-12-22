# IO项目

[code](minigrep/)

* 接收命令行参数

```rust
let args: Vec<String> = env::args().collect();
```

* 终止程序

```rust
process::exit(1);
```

* 标准错误输出

```rust
eprintln!("failed to grep: {}", e);
```

* 获取环境变量

```rust
let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
```

* 大小写敏感示例

```
cargo run to poem.txt
```

Output:

```
Are you nobody, too?
How dreary to be somebody!
```

* 大小写不敏感示例

```
CASE_INSENSITIVE=1 cargo run to poem.txt
```

Output:

```
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```