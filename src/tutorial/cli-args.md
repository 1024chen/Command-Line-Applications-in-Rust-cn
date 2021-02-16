# 解析命令行参数

我们的 CLI 工具的一个典型调用像这样：

```console
$ grrs foobar test.txt
```

我们期望我们的程序查看 `test.txt` 并且打印出含有 `foobar` 的行。但是我们如何得到这两个值呢？

在程序名称之后的文本通常叫做 “命令行参数” 或 “命令行标签” （特别是当它们看起来像 `--this` 时）。在内部，操作系统通常将它们表示为 字符串列表——通俗地说，它们用空格分隔。

有许多方法考虑这些参数，以及如何将其解析为更易于使用的参数。您还需要告诉程序的用户他们需要给出哪些参数以及期望的格式。

## 获取参数

标准库包含了 [`std::env::args()`] 函数，它为你提供给定参数的 迭代器[iterator] 。第一个输入（在索引 `0` 处）将是你的程序所叫名称 （例如 `grrs`）。其后是用户随之编写的内容。

[`std::env::args()`]: https://doc.rust-lang.org/1.39.0/std/env/fn.args.html
[iterator]: https://doc.rust-lang.org/1.39.0/std/iter/index.html

用这种方法获取原始参数非常容易（在文件 `src/main.rs` 的 `fn main() {` 之后）：

```rust,ignore
{{#include cli-args-struct.rs:10:11}}
```

## CLI 参数作为数据类型

将 CLI 参数视为输入程序的自定义数据类型而不是视为一串文本，通常会很有意义。

来看 `grrs foobar test.txt`：有两个参数，第一个是 `模式`（要查找的字符串），然后是 `路径`（要查找的文件）。

对两者更多的描述？好吧，首先两者都是必需的。我们还未讨论任何默认值，因此我们希望用户总是提供两个值。除此之外，我们可以说一下它们的类型：模式应该是字符串，而第二个参数应该是文件路径。

在 Rust 中，通常围绕数据处理来构造程序，因此查看 CLI 参数 的方法很合适（译者注：即将参数作为数据类型），让我们这样开始（在文件 `src/main.rs` 中的 `fn main() {` 之前）：

```rust,ignore
{{#include cli-args-struct.rs:3:7}}
```

这定义了一个拥有两个用于储存数据的字段 `pattern` 和 `path` 的新结构（一个[结构体][`struct`]）。

[`struct`]: https://doc.rust-lang.org/1.39.0/book/ch05-00-structs.html

<aside>

**顺便一说：**

[`PathBuf`] 就像一个[字符串][`String`] 但用于跨平台工作的文件系统路径。

[`PathBuf`]: https://doc.rust-lang.org/1.39.0/std/path/struct.PathBuf.html
[`String`]: https://doc.rust-lang.org/1.39.0/std/string/struct.String.html

</aside>

现在，我们依然需要将我们程序获取的实际参数转换成这种形式。一种选择是手动解析从操作系统获得的字符串列表，然后自己构建结构。 看起来像这样：

```rust,ignore
{{#include cli-args-struct.rs:10:15}}
```

代码可以工作，但这样非常不方便。你将如何处理支持 `--pattern="foo"` 或 `--pattern "foo"` 的需求？你如何实现 `--help`？

## 用 StructOpt 传递命令行参数

一个更好的办法是使用众多可用库中的一个。解析命令行参数最常用的库叫做 [`clap`] 。其具有你所期望的所有功能，包括支持子命令，shell 实现和良好的帮助信息。


[`structopt`] 库基于 `clap` 构建，并提供 “derive” 宏来为 `struct` 定义生成 `clap` 代码。非常不错，我们要做的就是注解一个结构体且它会生成将参数解析为字段的代码。

[`clap`]: https://clap.rs/
[`structopt`]: https://docs.rs/structopt

让我们通过在 `Cargo.toml` 文件的 `[dependencies]` 块中添加 `structopt = "0.3.13"` 来首次引入 `structopt` 。

现在，我们可以在代码中写入 `use structopt::StructOpt;` ，并在 `struct Cli` 上方添加 `#[derive(StructOpt)]`。我们顺便也写一些文档注释。

它看起来像这样（在 `src/main.rs` 中的 `fn main() {` 之前）：

```rust,ignore
{{#include cli-args-structopt.rs:3:14}}
```

<aside class="node">

**注意:**

可以向字段添加许多自定义属性。例如，我们添加了一个属性来告诉 structopt 如何解析 `PathBuf` 类型。要想在 `-o` 或 `--output` 之后使用该字段作为参数，可以添加 `#[structopt(short = "o", long = "output")]`。更多信息请参阅[structopt 文档][`structopt`]。

</aside>

在 `Cli` 结构体下方，我们的模板包含 `main` 函数。程序启动时，将调用此函数。第一行是：

```rust,ignore
{{#include cli-args-structopt.rs:15:18}}
```

这将尝试将参数解析到我们的 `Cli` 结构体中。

但是如果失败了怎么办呢？这时候这种办法的优点就显现出来了：Clap知道期望哪些字段以及期望的格式。它会自动生成一个较好的 `--help` 消息，同时当你写入 `--putput` 时，会发出一些错误提示来建议你更改为 `--output` 。

<aside class="note">

**注意：**   

`from_agrs` 方法应该用在 `main`函数中。当其失败时，将会打印错误或帮助信息，并立即退出程序。不要将它用在其他地方。

</aside>

## 这看起来像

运行的时候不使用任何参数：

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 10.16s
     Running `target/debug/grrs`
error: The following required arguments were not provided:
    <pattern>
    <path>

USAGE:
    grrs <pattern> <path>

For more information try --help
```

可以直接在使用 `cargo run` 时通过在其后写 `--` 传递参数：

```console
$ cargo run -- some-pattern some-file
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/grrs some-pattern some-file`
```

如你所见，没有输出。这很棒：表示没有错误，程序（运行完）结束了。

<aside class="exercise">

**读者练习：**

让程序输出其参数！
</aside>

