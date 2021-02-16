# 一个十五分钟的命令行应用

本教程将指导你用 [Rust] 编写 CLI （命令行界面）应用程序。这将花费你十五分钟以使你拥有一个正在运行的程序（大约到 1.3 章）。在那之后，我们将继续调整我们的程序直到可以发布为止。

[Rust]: https://rust-lang.org/

你将学习关于如何开始的所有要点，以及在哪里能找到更多信息。你可以随意略过现在你不需要知道的部分或者跳到任一位置。

<aside>

**先决条件：**

本教程并不取代编程的通用介绍，希望您先熟悉一些常见概念。你应该熟悉使用命令行/终端。如果你已经了解了一些其他的语言，这可能是你接触 Rust 的良好开端。

**获取帮助：**
如果你对所使用特性（features）的任一点感到不知所措或困惑。请查看 Rust 附带的大量官方文档，首先最重要的是 《Rust 程序设计语言》 这本书，它与大多数 Rust 设施一起提供（`rustup doc`），并可在 [doc.rust-lang.org] 线上获得。

[doc.rust-lang.org]: https://doc.rust-lang.org

也非常欢迎你提出问题—— Rust 社区以友好和乐于助人而闻名。请参阅 [社区主页][community page] 查看大家讨论 Rust 问题的清单。
（译者注：[Rust中文社区主页](https://rustcc.cn/)）

[community page]: https://www.rust-lang.org/community

</aside>

你想要编写哪种项目呢？我们先从简单的事情开始：让我们编写微型的 `grep` 复刻版。这是一个我们可以给出字符串和路径，且它将仅打印给定字符串的行的工具。我们称其为 `grrs` （发音为 “grass” ）。

最后，我们想要能像这样运行我们的工具：

```console
$ cat test.txt
foo: 10
bar: 20
baz: 30
$ grrs foo test.txt
foo: 10
$ grrs --help
[some help text explaining the available options]
```

<aside class="note">

**注意：**

本书是为 [Rust 2018] 所写。代码示例也能用于 Rust 2015 ，但是你可能需要对其进行一些调整；例如，添加 `extern crate foo;` 调用。

确保你运行的 Rust 版本为 Rust 1.31.0 或更往后的版本，并且在 `Cargo.toml` 文件中的 `[package]` 块中有 `edition = "2018"` 设置。

[Rust 2018]: https://doc.rust-lang.org/edition-guide/index.html

</aside>
