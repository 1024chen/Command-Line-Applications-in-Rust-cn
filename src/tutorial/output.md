# 输出

## 打印 "Hello World"

```rust
println!("Hello World");
```

当然，这很容易。很好，我们进入下个主题。

## 使用 println

你几乎可以用 `println!` 宏打印所有你喜欢的东西。该宏具有出色的功能， 但也有特殊的语法。它期望编写一个字符串字面量作为第一个参数，其中包括占位符，这些占位符由后续的参数值填充。

例如：

```rust
let x = 42;
println!("My lucky number is {}.", x);
```

会打印

```console
My lucky number is 42.
```

在上面字符串的花括号（`{}`）是这些占位符之一，这是默认的占位符类型，它尝试以人类可读的方式打印给定值。对于数字和字符串这很好用，但并非所有类型都可以这样，这也是为什么还有一个“debug 表示符”的原因，你可以通过填充占位符的大括号，就像这样：`{:?}` 来获得debug 表示符。

例如：

```rust
let xs = vec![1, 2, 3];
println!("The list is: {:?}", xs);
```

会打印

```console
The list is: [1, 2, 3]
```

如果想要自己的数据类型可打印以调试和日志记录，大多数情况下可以在其定义上方添加 `#[derive(Debug)]` 。

<aside>

**补充：**
“用户友好的” 打印是使用 [`Display`] trait 完成的，debug 输出（人类可读但面向开发者）使用 [`Debug`] trait。你可以在 [`std::fmt` 模块的文档][std::fmt]中找到关于 `println!` 使用语法的更多信息。

[`Display`]: https://doc.rust-lang.org/1.39.0/std/fmt/trait.Display.html
[`Debug`]: https://doc.rust-lang.org/1.39.0/std/fmt/trait.Debug.html
[std::fmt]: https://doc.rust-lang.org/1.39.0/std/fmt/index.html

</aside>

## 打印 errors

应该通过 `stderr` 完成错误打印，使得用户和其他工具更容易将其输出传输到文件或更多工具。

<aside>

**补充：**
在大多数操作系统，程序可以写入两个输出流， `stdout` 和 `stderr`。 `stdout` 是用于程序的实际输出，而 `stderr` 允许将错误和其他消息与 `stdout` 分开保存。这样，在向用户显示错误时，输出就可以存储到文件或传输到另一个程序。

</aside>

在 Rust 中这是用 `println!` 和 `eprintln!` 实现的，前者打印到 `stdout`，后者打印到 `stderr` 。

```rust
println!("This is information");
eprintln!("This is an error! :(");
```

<aside>

**当心：**
打印 [转义码][escape codes] 可能会很危险，会将用户的终端处于异常状态。手动打印时请务必当心。

[escape codes]: https://en.wikipedia.org/wiki/ANSI_escape_code

理想状况下，在处理原始转义码的时候，应该使用像 `ansi_term` 之类的 crate ，以使你（和你的用户）生活更轻松（译者注：简化工作）。

</aside>

## 关于打印的性能的注意事项

打印到终端是非常慢的！如果你在循环中调用 `println!` 之类的，它会很容易成为其他快速型程序的瓶颈。为了提升速度，你可以做两件事。

首先，你可能需要减少实际 “刷新” 到终端的写入次数。`println!` 告诉系统每次都刷新终端，因为打印新行是常见的。如果不需要，可以将 `stdout` 句柄包装进 [`BufWriter`] 中， [`BufWriter`] 默认情况下可缓存高达 8kB。（当你想立即打印时，仍然可以在 `BufWriter` 中调用 `.flush()` 函数）

```rust
use std::io::{self, Write};

let stdout = io::stdout(); // 获取全局 stdout 实体
let mut handle = io::BufWriter::new(stdout); // 可选： 将句柄包装进缓冲区中
writeln!(handle, "foo: {}", 42); // 如果你关心此处的 error，添加 `?` 。
```

其次，获取对`stdout` （或 `stderr`）的锁并使用 `writeln!` 直接打印它是很有用的。这样可以防止系统反复锁定和解锁 `stdout` 。

```rust
use std::io::{self, Write};

let stdout = io::stdout(); // 获取全局 stdout 实体
let mut handle = stdout.lock(); // 获取它的锁
writeln!(handle, "foo: {}", 42); // 如果你关心此处的 error，添加 `?` 。
```

你还可以结合这两种实现。

[`BufWriter`]: https://doc.rust-lang.org/1.39.0/std/io/struct.BufWriter.html

## 显示进度条

一些命令行应用程序运行时间不到一分钟，其他的则会花几分钟或几小时。如果要编写后一种类型的程序，则可能需要向用户显示正在发生的事。为此，你应该尝试打印有用的状态更新，最好以一种易于使用的形式打印。

使用 [indicatif] crate，你可以在程序中添加进度条和小框。这儿是个简单的例子：

```rust,ignore
{{#include output-progressbar.rs:1:9}}
```

更多信息请参阅其 [文档][indicatif docs]和 [示例][indicatif examples]

[indicatif]: https://crates.io/crates/indicatif
[indicatif docs]: https://docs.rs/indicatif
[indicatif examples]: https://github.com/mitsuhiko/indicatif/tree/master/examples

## 日志

为了更容易理解程序中发生的事情，我们可能需要添加一些日志语句。在编写应用程序时，这通常很容易。但当半年后再次运行这个程序时，它将变得非常有帮助。在某些方面，日志记录与使用println是相同的，除了可以指定消息的重要性。通常可以使用的级别是 error 、 warn 、 info 、 debug 和 trace ( error 的优先级最高， trace 的优先级最低)。

要将简单的日志记录添加到你的应用程序中，您需要做两件事： [log] crate （其中包含以日志级别命名的宏）和一个适配器（adapter），该适配器实际上将日志输出写入有用的地方。使用日志适配器的能力非常灵活：例如，您可以使用它们将日志不仅写到终端，也写到syslog或中央日志服务器。

[syslog]: https://en.wikipedia.org/wiki/Syslog

由于我们现在只关心编写命令行应用程序，一个易于使用的适配器是 [env_logger] 。之所以称为 “ env” logger，是因为您可以使用环境变量来指定要记录的应用程序部分（以及要记录的级别）。它将在你的的日志消息前加上时间戳和日志消息来源的模块。由于库也可以使用 `log` ，因此您也可以轻松配置其日志输出。

这是一个简单的示例：

[log]: https://crates.io/crates/log
[env_logger]: https://crates.io/crates/env_logger

```rust,ignore
{{#include output-log.rs}}
```

假设有 `src/bin/output-log.rs` 这个文件，在 Linux 和 macOS 上，你可以像这样运行：
```console
$ env RUST_LOG=output_log=info cargo run --bin output-log
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/output-log`
[2018-11-30T20:25:52Z INFO  output_log] starting up
[2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
```

在 Windows PowerShell 上，你可以像这样运行：
```console
$ $env:RUST_LOG="output_log=info"
$ cargo run --bin output-log
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/output-log.exe`
[2018-11-30T20:25:52Z INFO  output_log] starting up
[2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
```

在 Windows CMD 上，你可以像这样运行：
```console
$ set RUST_LOG=output_log=info
$ cargo run --bin output-log
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/output-log.exe`
[2018-11-30T20:25:52Z INFO  output_log] starting up
[2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!
```

`RUST_LOG` 是可用于设置日志设置的环境变量的名称。
`env_logger` 还包含一个构建器（builder），因此你可以以编程方式调整这些设置，而且，例如，默认情况下还显示 info 级别的消息。

有很多其他的日志适配器，以及 `log` 的扩展和替代方法。如果你知道应用程序有很多 `log` ，请确保对其进行 review ，并简化用户的使用。

<aside>

**提示：**
经验表明，即使是非常有用的命令行应用程序也可能会在未来几年被弃用（特别是如果它们仅是临时解决方案）。如果你的应用程序无法正常运行，并且某个人（例如，未来的你）需要找出原因，可以通过 `--verbose` 来获取额外的日志输出，这可能会造成调试数分钟或数小时的差异。 [clap-verbosity-flag] crate 包含使用 `structopt`向项目添加 `--verbose` 的快速方法。

[clap-verbosity-flag]: https://crates.io/crates/clap-verbosity-flag

</aside>
