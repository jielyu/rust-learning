# 认识Cargo

Cargo 的官方网站是 `crates.io`

## 使用发布配置定制构建

[code](adder/)

* 在 `Cargo.toml` 文件中的 `profile.dev` 区域配置debug设置； `profile.release` 区域配置release设置

设置优化等级的配置如下：

```toml
[profile.dev]
opt-level=1
```

## 将包发布到 crates.io  

* 编写有用的文档注释，使用  `///` 开头；也可以用 `//!`，一般用于 `src/lib.rs` 文件

```
/// 将传入的数字加1
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = adder::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

使用 `cargo doc --open` 编译并打开文档

使用 `cargo test` 运行文档注释中的测试例子

* 使用 `pub use` 将需要公开的条目重新导出到顶层结构中

* 创建 `crates.io` 账户，获取令牌 `token` 之后在本地登录

```
cargo login ${token}
```

* 为包条件元数据

```
[package]
name = "package_name"
version = "0.1.0"
authors = ["Name <xxx@xxx.com>"]
edition = "2018"
description = "None"
license = "MIT"
```

* 发布到 `crates.io`

```
cargo publish
```

* 更改版本号之后，发布已有包的新版本

* 使用 `cargo yank` 移除版本

移除版本

```
cargo yank --vers 1.0.1
```

撤销移除

```
cargo yank --vers 1.0.1 --undo
```

移除操作不会删除代码，只是告知其他项目尽量不依赖该版本

## 工作空间

[code](rust_workspace/)

第一步： 创建工作空间目录， 比如 `rust_workspace`

第二步： 在工作空间目录下创建 `Cargo.toml` 文件，并添加成员信息

```toml
[workspace]

members = [
    "adder",
    "add-one",
]
```

第三步： 创建第一个包

```
cargo new adder
```

第四步： 创建第二个包

```
cargo new add-one --lib
```

第五步： 在 `rust_workspace/adder/Cargo.toml` 中添加依赖

```toml
[dependencies]
add-one = {path = "../add-one"}
```

第六步： 编译运行

```
cargo build
cargo run
```

第七步： 直接在需要添加第三方包的子项目下添加， 比如在 `add-one/Cargo.toml` 文件中添加

```toml
[dependencies]
rand = "0.3.14"
```

第八步： 在工作空间目录下运行测试 

```
cargo test
```

也可以运行指定包的测试

```
cargo test -p add-one
```

## 从 `crates.io` 安装二进制可执行程序

```
cargon install ${tool_name}
```

将程序安装到 `$HOME/.cargo/bin`

## 使用自定义命令扩展Cargo功能

只需将二进制文件命名为 "cargo-something"，并放置在环境变量`PATH`可查找到的路径下即可

使用 `cargo --list` 可以列举出所有的子命令