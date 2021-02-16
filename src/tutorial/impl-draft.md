# 首次实现 _grrs_

在上一章命令行参数之后，我们有了输入数据，可以编写实际的工具了。我们的 "main" 函数现在只包括以下这行：

```rust,ignore
{{#include impl-draft.rs:17:17}}
```
先从打开我们拿到的文件开始。

```rust,ignore
{{#include impl-draft.rs:18:19}}
```

<aside>  

**补充：**

看到这个 [expect][`.expect`] 方法了么？这是个快捷的退出函数，当无法读取值（该情况下为输入文件）时，使得程序立即退出。它并非完美，在下一章 [更好的错误报告][Nicer error reporting] 中，我们将探讨如何改善。

[`.expect`]: https://doc.rust-lang.org/1.39.0/std/result/enum.Result.html#method.expect
[Nicer error reporting]:./errors.html

</aside>

现在，让我们遍历行并打印包含模式的每一行：

```rust,ignore
{{#include impl-draft.rs:21:25}}
```

尝试一下： `cargo run -- main src/main.rs` 现在应该能正常工作了。

<aside class="exercise">

**读者练习：**

这并非最佳实现：它会将整个文件读入内存——无论文件多大。试找到一种方法优化它！（一种思路是使用 [`BufReader`] 而非  `read_to_string()`。）

[`BufReader`]: https://doc.rust-lang.org/1.39.0/std/io/struct.BufReader.html

</aside>
