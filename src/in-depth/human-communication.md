# 与人交互

请确保你已经先阅读了 [CLI 输出的章节][output] 。它涵盖了如何将输出写入到终端，而本章将讨论关于输出什么。

[output]: ../tutorial/output.html

## 当一切正常

即使一切正常，报告应用程序的进展也是很有用的。尽量提供丰富·简洁的消息，不要在日志中使用过多的术语。记住：应用程序没有崩溃，所以用户没有理由去（日志之类的输出）查找错误。

最重要的是，保持风格一致的沟通。使用相同的前缀和句子结构会使得日志易于浏览。

尝试让你的应用程序的输出告诉你它所做的事以及对用户的影响。这可能会包括显示所涉及步骤的时间线，甚至是进度条和长期运行操作的指示器。用户理应无法感觉到程序正在做一些他们无法理解的隐秘的事情。（译者注：也就是对用户隐藏无关的部分。）

## 当很难说清发生了什么

在传递不可名状的状态时，保持一致是很重要的。不遵循严格日志级别的记录日志的应用与不记录日志的应用相比，能提供的信息量相同甚至更少。

因此，定义与之相关的事件和消息的严重级别非常重要；然后为它们使用一致的日志级别。以此方式，用户可以通过 `--verbose` 标签或环境变量（例如  `RUST_LOG`）选择他们日志的数量。

常用的 `log` crate [定义][log-levels] 了以下级别（按照严重级别递增排序）：

- trace
- debug
- info
- warning
- error

一个很好的做法是将 _info_ 视为默认的日志级别。可以将它用于信息输出。（一些输出风格更倾向于安静的应用程序可能默认情况下只显示警告和错误。）

此外，通常一个很好的做法是在日志消息中使用相似的前缀和句子结构，可以轻松地使用如 `grep` 这类工具来过滤它们。消息本身应提供足够的上下文，以便在过滤后的日志中使用，同时又不太冗长。

[log-levels]: https://docs.rs/log/0.4.4/log/enum.Level.html

### 日志语句示例

```console
error: could not find `Cargo.toml` in `/home/you/project/`
```

```console
=> Downloading repository index
=> Downloading packages...
```

以下日志输出来自于 [wasm-pack]:

```console
 [1/7] Adding WASM target...
 [2/7] Compiling to WASM...
 [3/7] Creating a pkg directory...
 [4/7] Writing a package.json...
 > [WARN]: Field `description` is missing from Cargo.toml. It is not necessary, but recommended
 > [WARN]: Field `repository` is missing from Cargo.toml. It is not necessary, but recommended
 > [WARN]: Field `license` is missing from Cargo.toml. It is not necessary, but recommended
 [5/7] Copying over your README...
 > [WARN]: origin crate has no README
 [6/7] Installing WASM-bindgen...
 > [INFO]: wasm-bindgen already installed
 [7/7] Running WASM-bindgen...
 Done in 1 second
```

## panic 的时候

时常被遗忘的一个方面是程序在崩溃时也会输出某些内容。在 Rust 中，“崩溃” 通常也是 “panic” （也即是“受控制的崩溃”，而不是 “操作系统 kill 掉了进程”）。默认情况下，panic 发生的时候，"panic 处理程序"会将一些信息打印到控制台。

例如，如果你用 `cargo new --bin foo` 创建了一个新的二进制项目，并且将`fn main` 里面的内容替换成了 `panic!("Hello World")`，当你运行程序的时候你会得到以下输出：

```console
thread 'main' panicked at 'Hello, world!', src/main.rs:2:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

这对你——开发者来说，是很有用的信息。(Surprise: 程序因你的 `main.rs` 文件的第二行而崩溃了)。但是对于那些甚至没有访问源代码的用户来说，这并非很有价值。实际上，它很可能只能令人困惑。这就是为什么添加一个自定义的 panic 处理程序是一个很好的想法，它提供了更多的以终端用户为中心的输出。

有个做到了这点（译者注：自定义的 panic 处理程序）的库叫做 [human-panic] 。要将其添加到 CLI 项目中，你可以导入它并在 `main` 函数的开头调用 `setup_panic!()` 宏：

```rust,ignore
use human_panic::setup_panic;

fn main() {
   setup_panic!();

   panic!("Hello world")
}
```

这将显示一条非常友好的消息，并告诉用户他们能做什么：

```console
Well, this is embarrassing.

foo had a problem and crashed. To help us diagnose the problem you can send us a crash report.

We have generated a report file at "/var/folders/n3/dkk459k908lcmkzwcmq0tcv00000gn/T/report-738e1bec-5585-47a4-8158-f1f7227f0168.toml". Submit an issue or email with the subject of "foo Crash Report" and include the report as an attachment.

- Authors: Your Name <your.name@example.com>

We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.

Thank you kindly!
```

[human-panic]: https://crates.io/crates/human-panic
[wasm-pack]: https://crates.io/crates/wasm-pack
