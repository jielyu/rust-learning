# Day 01 入门指南

## 开发运行环境

### 安装指令

```bash
curl https://sh.rustup.rs -sSf | sh
```

### 更新指令

```bash
rustup update
```

### 卸载指令

```bash
rustup self uninstall
```

## Hello Rust

进入 `hello_rust` 目录，执行编译指令 `rustc main.rs`，
生成可执行文件之后，运行该文件 `./main`

## Hello Cargo

### 查看cargo版本

```bash
cargo --version
```

### 创建hello_cargo项目

```bash
cargo new hello_cargo
```

生成项目的目录结构如下：
```
.
├── Cargo.toml
└── src
    └── main.rs
```

### 编译

```bash
cargo build
```

release模式的编译需添加参数

```bash
cargo build --release
```

会生成 `target` 目录存放编译产出， 目录结构如下：

```
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    ├── debug
    │   ├── build
    │   ├── deps
    │   │   ├── hello_cargo-1690ef88d930c8b3
    │   │   ├── hello_cargo-1690ef88d930c8b3.2jk93w8cbjq7std1.rcgu.o
    │   │   ├── hello_cargo-1690ef88d930c8b3.3faieb32u4023tsv.rcgu.o
    │   │   ├── hello_cargo-1690ef88d930c8b3.406304og7o5e866.rcgu.o
    │   │   ├── hello_cargo-1690ef88d930c8b3.d
    │   │   ├── hello_cargo-1690ef88d930c8b3.eozlhzjxg9esk48.rcgu.o
    │   │   ├── hello_cargo-1690ef88d930c8b3.oh9lu26p5somjf4.rcgu.o
    │   │   └── hello_cargo-1690ef88d930c8b3.qpeb6hyav9d6bfa.rcgu.o
    │   ├── examples
    │   ├── hello_cargo
    │   ├── hello_cargo.d
    │   └── incremental
    │       └── hello_cargo-d4g6xr68omkp
    │           ├── s-ggenkli7po-thambk-21gxcv8gplesl
    │           │   ├── 2jk93w8cbjq7std1.o
    │           │   ├── 3faieb32u4023tsv.o
    │           │   ├── 406304og7o5e866.o
    │           │   ├── dep-graph.bin
    │           │   ├── eozlhzjxg9esk48.o
    │           │   ├── oh9lu26p5somjf4.o
    │           │   ├── qpeb6hyav9d6bfa.o
    │           │   ├── query-cache.bin
    │           │   └── work-products.bin
    │           └── s-ggenkli7po-thambk.lock
    └── release
        ├── build
        ├── deps
        │   ├── hello_cargo-7882b278faa24516
        │   └── hello_cargo-7882b278faa24516.d
        ├── examples
        ├── hello_cargo
        ├── hello_cargo.d
        └── incremental
```

### 运行

```bash
cargo run
```

Output:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_cargo`
Hello, Cargo!
```

运行 release 版本需要添加参数

```bash
cargo run --release
```

Output:

```
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/hello_cargo`
Hello, Cargo!
```