# 信号处理

诸如命令行应用这样的进程需要对操作系统发送的信号作出反应。最常见的就比如 <kbd>Ctrl</kbd>+<kbd>C</kbd>，该信号通常告诉进程终止。要在 Rust 程序中处理信号，你需要考虑如何接收这些信号以及如何做出反应。

<aside>

**注意：**
如果你的应用不需要优雅地关停，则使用默认处理方式就行（即立即退出，让操作系统清理资源，如打开文件句柄）。该情况下：无需执行本章节告诉你的内容。

然而，对于那些需要自行清理的应用程序，本章节是很重要的！例如，如果你的应用程序需要正确地关闭网络连接（也就是对另一端的进程说 “再见”），删除临时文件，或者重置系统设置，还请继续阅读。

</aside>

## 操作系统间差异

在 Unix 类系统（ 例如 Linux, macOS, 和 FreeBSD ）上，进程可以接收 [信号][signals]。它可以以默认方式（操作系统所提供）对它们做出反应，捕获信号并以程序所定义的方式对它们进行处理，或者完全忽略信号。

[signals]: https://manpages.ubuntu.com/manpages/bionic/en/man7/signal.7.html

Windows 没有信号，你可以用 [Console Handlers][Console Handlers] 来定义在事件发生时执行的回调。还有 [结构化异常处理][structured exception handling]，可以处理各种类型的系统异常，例如被 0 除，无效访问异常，栈溢出，等等之类的。

[Console Handlers]: https://docs.microsoft.com/en-us/windows/console/console-control-handlers
[structured exception handling]: https://docs.microsoft.com/en-us/windows/desktop/debug/structured-exception-handling

## 首先：处理 Ctrl+C

[ctrlc] crate 的用途恰如其名：其允许你以跨平台的方式对用户按的 <kbd>Ctrl</kbd>+<kbd>C</kbd> 做出反应，使用该 crate 的主要方法是：

[ctrlc]: https://crates.io/crates/ctrlc

```rust,ignore
{{#include signals-ctrlc.rs}}
```

当然，这并无帮助：只打印消息，但是不会停止程序（译者注：除非出现错误）。

在实际的程序中，最好是在这个执行信号处理的程序中设置一个变量，然后在程序的各个位置进行检查。例如，可以在信号处理中设置一个 `Arc<AtomicBool>` ，然后在热循环（hot loops）中，或者当等待一个线程时，你定时地检查它，并在当它变为 true 时中断（break）。

## 处理其它类型的信号

[ctrlc][ctrlc] crate 仅处理 <kbd>Ctrl</kbd>+<kbd>C</kbd> ，或者，在 Unix 系统中被称为 `SIGINT` （“中断” 信号）。为了对更多的 Unix 信号做出反应，你应该查看 [signal-hook][signal-hook]。[这篇博文][signal-hook-post]描述了其设计，它时目前社区所支持的最广泛的库。

这有个简单的例子：

```rust,ignore
{{#include signals-hooked.rs}}
```

[signal-hook-post]: https://vorner.github.io/2018/06/28/signal-hook.html

## 使用 channel

不设置变量并使用程序的其他部分检查它，你可以使用 通道（channel） ：创建一个通道，每当接收到信号时，信号处理程序就向该通道发出一个值。在你的应用程序代码中，可以使用这个通道和其他通道作为线程之间的同步点，使用 [crossbeam-channel] 看起来像这样：

[crossbeam-channel]: https://crates.io/crates/crossbeam-channel

```rust,ignore
use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};
use anyhow::Result;

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

fn main() -> Result<()> {
    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("working!");
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}
```

## 使用 future 和 stream

如果你使用 [tokio]，说明你很可能已经在你的应用程序中使用了异步模式和事件驱动设计。相比直接使用 crossbeam 的通道，你可以使用 signal-hook 的 `tokio-support` feature，其允许你在 signal-hook 的类型上调用 [`.into_async()`] 来获得实现了 `futures::Stream` 的新类型。

[signal-hook]: https://crates.io/crates/signal-hook
[tokio]: https://tokio.rs/
[`.into_async()`]: https://docs.rs/signal-hook/0.1.6/signal_hook/iterator/struct.Signals.html#method.into_async

## 当你正在处理第一个 Ctrl+C 的时候接收到了其他的 Ctrl+C 时怎么办

大多数用户会按 <kbd>Ctrl</kbd>+<kbd>C</kbd>，然后给你的程序几秒钟时间退出，或者告诉他们发生了什么。如果这并未发生，他们就会再次按下 <kbd>Ctrl</kbd>+<kbd>C</kbd> 。当然，最典型的行为是使应用程序立即退出。
