# 更好的错误报告

错误总会发生这个事实我们只能选择接受。与其他语言相比，在使用 Rust 时很难不注意和面对这个现实：因为它没有异常，所有可能的错误状态通常都编码在函数的返回类型中。

## Results

像 [`read_to_string`] 这样的函数并不返回一个字符串，而是返回一个包含 `String` 或某种类型的错误（这种情况下为 [`std::io::Error`]）的 [`Result`] 。

[`read_to_string`]: https://doc.rust-lang.org/1.39.0/std/fs/fn.read_to_string.html
[`Result`]: https://doc.rust-lang.org/1.39.0/std/result/index.html
[`std::io::Error`]: https://doc.rust-lang.org/1.39.0/std/io/type.Result.html

怎么能知道它是什么呢？因为 `Result` 是个`枚举`类型，所以可以使用 `match` 来检查它是哪种变量：

```rust,no_run
let result = std::fs::read_to_string("test.txt");
match result {
    Ok(content) => { println!("File content: {}", content); }
    Err(error) => { println!("Oh noes: {}", error); }
}
```

<aside>

**补充：**
不清楚枚举是什么或它们如何在 Rust 中工作？[查看 Rust book 的这章](https://doc.rust-lang.org/1.39.0/book/ch06-00-enums.html)以掌握最新信息。

</aside>

## Unwrapping

现在，我们可以访问文件中的内容，但是在 `match` 块之后，我们实际上就不能做任何事了（译者注：因为上面代码中 match 到的类型并未在 match 块后记录下来）。为此，我们需要以某种方式处理错误情况。挑战在于 `match` 块的所有分支都需要返回相同类型的的值。有一个巧妙的技巧可以解决这个问题：

```rust,no_run
let result = std::fs::read_to_string("test.txt");
let content = match result {
    Ok(content) => { content },
    Err(error) => { panic!("Can't deal with {}, just exit here", error); }
};
println!("file content: {}", content);
```

我们可以在 match 块后使用 `content` 字符串。如果 `result` 为 error，这个 `content` 字符串就不存在。但因为程序会在使用 `content` 之前退出，所以这个方法没有任何问题。

这看起来很生猛，但非常方便。如果你的程序需要读取文件，且如果文件不存在则不能执行任何操作，退出是一种有效的策略。`Result` 上甚至还有一个快捷方法，称为`解包`（`unwrap`） :

```rust,no_run
let content = std::fs::read_to_string("test.txt").unwrap();
```

## 不需要 panic

当然，终止程序不是唯一的处理错误的办法。除了 `panic!`，我们还可以轻松编写 `return`：

```rust,no_run
# fn main() -> Result<(), Box<std::error::Error>> {
let result = std::fs::read_to_string("test.txt");
let _content = match result {
    Ok(content) => { content },
    Err(error) => { return Err(error.into()); }
};
# Ok(())
# }
```
（译者注：以上这种不在自身内部进行错误处理，而将其包装为 Result 往函数调用者处传递，让函数调用者进行正确值或错误处理的写法被称为**传播错误**，当然，如果你学过 Java 这类带有异常的语言，就会觉得有点像 throw Exception ，然而两者还是有本质上的区别的。）
然而这改变了我们函数所需的返回类型。实际上，在我们的示例中一直隐藏着一些事物：代码所在函数的签名。上一个示例中的 `return` 十分重要。这儿是一个完整的例子：

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    };
    println!("file content: {}", content);
    Ok(())
}
```

我们的返回类型是一个 `Result`！这就是为什么我们可以在第二个匹配分支写 `return Err(error);` 的原因。底部为何有 `Ok(())` 呢？这是函数的默认返回值，意味着 “结果没问题，且没有内容”。

<aside>

**补充：**
为什么不写成 `return Ok(());` 呢？这是很容易也是完全有效的。Rust 中 任何块（译者注：此处指的块是以一对花括号包裹的代码块。）的最后一个表达式是其返回值，习惯上会省略不必要的返回值。（译者注：这里需要分清表达式和语句。Rust是一门面向表达式的语言，Rust 中你可以简单认为语句是以 “；” 结尾的表达式，且语句无返回值或返回值为 () 单元类型。）

</aside>

## 问号

就像调用 `.unwrap()` 是 `match` 的 error 分支 `panic!` 的简便写法一样。我们有另一个 `match` 中 error 分支 `return` 的简写：？。

是的，就是一个问号。你可以将该操作符附加到 `Result` 类型的值上，Rust 会将其扩展为与我们刚才编写的 `match` 非常相似的内容。

尝试一下：

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("test.txt")?;
    println!("file content: {}", content);
    Ok(())
}
```

可以说是非常简洁了！

<aside>

**补充：**
这里有一些尚未要求理解如何工作的事情。例如，在 `main` 函数的返回错误类型是 `Box<dyn std::error::Error>` ，但是我们刚才在上面看到 `read_to_string` 返回 [`std::io::Error`]。这行得通是因为 ？展开为 converts error 类型的代码。

`Box<dyn std::error::Error>` 也是一种有趣的类型。它是一个可以包含任意实现了标准 [`Error`][`std::error::Error`] trait 的类型 的 `Box` 智能指针。这意味着基本上所有 error 都能放进该 Box ，所以我们可以将 ？用于返回 `Result` 的所有常用函数。 

[`std::error::Error`]: https://doc.rust-lang.org/1.39.0/std/error/trait.Error.html

</aside>

## 提供 context

当在 `main` 函数中使用 `?` 获取 error 是没问题的，但是这还不够好。例如：当你运行  `std::fs::read_to_string("test.txt")?` ，但是 `test.txt` 并不存在，你会得到这个输出：

```text
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

如果代码中没有字面上包含文件名（译者注：即读取文件函数的参数是一个变量而非字面量），那么很难确定是哪个文件 `NotFound`。这里有多种解决办法。

例如，我们可以创建自己的 error 类型，然后用它来构建自定义错误消息：

```rust,ignore
{{#include errors-custom.rs}}
```

现在，运行会得到我们的自定义错误消息：

```text
Error: CustomError("Error reading `test.txt`: No such file or directory (os error 2)")
```

不是很完美，但是稍后我们可以很容易地根据我们的类型调整 debug 输出。

这种模式实际上很常见。不过它有个问题：我们并不储存原始 error ，仅储存其字符串表示。常用的 [`anyhow`] 库有个巧妙的解决方案：与我们的 `CustomError` 类型类似，它的 [`Context`] trait 可以用来添加描述。除此之外，它还保留了 原始 error ，因此我们会得到指向错误根源的 error “链”。

[`anyhow`]: https://docs.rs/anyhow
[`Context`]: https://docs.rs/anyhow/1.0/anyhow/trait.Context.html

让我们通过在 `Cargo.toml` 文件的 `[dependencies]` 块中添加 `anyhow = "1.0"` 来首次引入 `anyhow` crate 。

完整的示例看起来像这样：

```rust,ignore
{{#include errors-exit.rs}}
```

这会打印出错误：

```text
Error: could not read file `test.txt`

Caused by:
    No such file or directory (os error 2)
```
