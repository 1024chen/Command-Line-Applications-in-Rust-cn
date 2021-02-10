# 项目设置

如果你还未在你的计算机中安装 Rust，请 [安装 Rust][install Rust] （一般只花费几分钟）。在那之后，打开终端并导航到你想要存放应用程序代码的目录。

[install Rust]: https://www.rust-lang.org/tools/install

首先在你的程序项目所在目录中运行 `cargo new grrs`，你将会看到一个 Rust 项目的典型设置：

- `Cargo.toml` 文件包含了项目的元数据，包括我们使用的依赖/外部库的清单。
- `src/main.rs` 文件是我们(main) 二进制文件的入口点。

如果你在 `grrs` 目录执行 `cargo run` 命令，会得到一个 “hello world” ，说明一切准备就绪。

## 它可能看起来像

```console
$ cargo new grrs
     Created binary (application) `grrs` package
$ cd grrs/
$ cargo run
   Compiling grrs v0.1.0 (/Users/pascal/code/grrs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
     Running `target/debug/grrs`
Hello, world!
```
