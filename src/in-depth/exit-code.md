# 退出码

一个程序并不能总是成功的（也就是毫无 bug），当错误发生时，你应该确保正确地发出必要的信息。除了[告诉用户有关错误](human-communication.html)之外，在大多数系统上，当进程退出时，它还会发出一个退出码（大多数平台所兼容的是一个 0 到 255 的整数）。你应该尝试为程序的状态发出正确的代码。例如，在理想情况下，当程序成功时，它应该以 `0` 退出。

但是，当错误发生的时候，情况也会变得更复杂一点。在实际情况中，当一个常见的故障发生时，许多工具以 `1` 退出。目前， Rust 设置了一个当进程 panic 时的退出码 `101`。除此之外，人们还在自己的程序中做了许多（关于退出码的）事。

那么，该怎么办呢？BSD 系操作系统为它们的退出码设置了一个通用的定义集合
(你可以在[这里][`sysexits.h`]找到它们)。 Rust 的  [`exitcode`]  库提供了这些相同的代码，可以被用在你的应用程序中。可能会使用到的值请参阅其 ADP 文档。

在将 `exitcode` 依赖添加到 `Cargo.toml` 之后，你可以像这样使用它：

```rust,ignore
fn main() {
    // ...actual work...
    match result {
        Ok(_) => {
            println!("Done!");
            std::process::exit(exitcode::OK);
        }
        Err(CustomError::CantReadConfig(e)) => {
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::CONFIG);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::DATAERR);
        }
    }
}
```


[`exitcode`]: https://crates.io/crates/exitcode
[`sysexits.h`]: https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+11.2-stable&arch=default&format=html
