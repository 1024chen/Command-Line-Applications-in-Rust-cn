# 测试

在数十年的软件开发中，人们发现了一个真理：未经测试的软件很少能工作。（许多人甚至会说：“其实大多数经过测试的软件也不能正常工作”，但我们都是乐观主义者，不是吗？）因此，要确保你的程序能实现你期望的功能，明智的办法是测试一下。

一种简单的方是编写一个 `README` 文件描述程序应该做什么。当你准备发布新版本时，浏览 `README` 文件并确保其行为仍然符合预期。你还可以写下程序对错误输入的反应，以使操作更加严格。

这是另一个奇特的想法：在你写代码之前先编写 `README` 文件。

<aside>

**补充：**
如果你没有听说过测试驱动开发（TDD），请参阅其[wiki] (TDD)。

[test-driven development]: https://en.wikipedia.org/wiki/Test-driven_development


</aside>

## 自动化测试

现在，一切看起来都还不错，但是我们需要手动进行测试么？这会浪费很多时间。同时，许多人更喜欢让计算机来做这些。让我们来谈谈如何将测试自动化。

Rust 有内置的测试框架，所以让我们从编写第一个测试开始：

```rust,ignore
#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}
```

你可以将这段代码放入任何文件(这里说的是 .rs 文件)中， `cargo test` 会找到并运行它。 `#[test]` 属性是关键，它允许构建系统发现此函数并将其作为测试运行，以验证它们不会 panic 。

<aside class="exercise">

**读者练习：**
使得此测试有效。

你应该最终得到以下输出：
```text
running 1 test
test check_answer_validity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

</aside>

现在我们已经知道了如何编写测试。我们还应该搞明白怎样去测试。如你所见，为函数编写断言是相当容易的，但是命令行应用程序通常不止一个函数！更糟的是，它经常处理用户输入，读取文件和输出。

## 让你的代码可测试

有两种测试函数的互补实现：测试构建的完整应用程序中的小型单元，这被称为 “单元测试”。还有一种“从外部”测试最终的应用程序，这被称作“黑盒测试”或“集成测试”。让我们从第一种开始。

要弄清我们应该测试什么，得知道我们的程序功能是什么。`grrs` 主要的是 应该打印出与给定模式匹配的行。因此，让我们为此编写单元测试：我们应该确保最重要的逻辑部分正常工作，并且我们希望以不依赖周围设置代码（例如，处理命令行参数的代码）的方式进行测试。

回到我们的 `grrs`的 [首次实现](impl-draft.md) ，我们在 `main`函数中增加了这个代码块：

```rust,ignore
// ...
for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line);
    }
}
```

不幸的是，这很不容易测试。首先，它在 mian 函数中，所以不能轻易地调用它。这个问题可以通过将此段代码移到函数中来轻松修复：

```rust,no_run
fn find_matches(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}
​```现在我们可以在测试中调用此函数，并查看其输出：

​```rust,ignore
#[test]
fn find_a_match() {
    find_matches("lorem ipsum\ndolor sit amet", "lorem");
    assert_eq!( // uhhhh
```

或者，我们可以？现在， `find_matches` 直接打印到 `stdout`，也就是终端。我们无法在测试中轻松捕捉到这点！当实现之后编写测试时，经常会出现一个问题：编写一个函数，该函数与使用它的上下文紧密地整合在一起。

<aside class="note">

**注意：**
在编写小型命令行程序的时候这完全没问题。没有必要让所有（代码）都是可测试的！然而，考虑你应该给哪部分代码编写测试很重要，虽然我们可以很容易地将函数变为可测试的，但这种情况是不常见的。

</aside>

好了，那我们如何使其可测试呢？我们需要以某种方式捕获输出。 Rust 标准库在处理 I/O （输入/输出）时有一些简洁的抽象，我们将使用一种叫做 [`std::io::Write`][`std::io::Write`] 的 [trait][trpl-traits] ，该 trait 对我们可以写入的事物进行抽象，其中不仅包括字符串，也包括 `stdout` 。

[trpl-traits]: https://doc.rust-lang.org/book/ch10-02-traits.html
[`std::io::Write`]: https://doc.rust-lang.org/1.39.0/std/io/trait.Write.html

如果这是你第一次在 Rust 中听到 "trait" 这个术语，那么你一定会对它感到满意的。 trait 是 Rust
最强大的特性之一，你可以将它看作 Java 中的接口（interface），或者是 Haskell 中的 类型类（type class） （不管你对它们有多熟悉）。它们允许抽象出不同类型之间共享的行为。使用 trait 的代码可以以非常通用和灵活的方式进行表达（译者注：也就是 trait 的定义和实现方式）。不过，这也意味着它很难阅读。不要让它吓到你：因为即使是使用了多年 Rust 的人也不能总是立即知道通用（generic）代码的行为（译者注：也就是说，有些 trait 太难读懂，连老手都要理解半天，别怕，因为对于大家来说都是难点，嘿嘿）。在这种情况下，考虑其具体用途是非常有用的。例如，在我们的例子中，我们抽象的行为是 “写入” 。实现（“impl”）它的类型的示例包括：终端标准输出，文件，内存中的缓冲区，或者 TCP 网络连接。（向下滚动 [`std::io::Write` 的文档][`std::io::Write`] 查看 "Implementors" 清单。）

有了这些知识，让我们将函数改为接受第三个参数。它应该是任何实现了 `Write` 的类型。这样的话，我们就可以在测试中提供一个简单的字符串并对其进行断言。这是我们为此编写的 `find_matches` 版本。

```rust,ignore
{{#include testing/src/main.rs:24:30}}
```

新参数是 `mut writer`，即叫做 ”writer“ 的可变变量，它的类型是 `impl std::io::Write` ，你可以将其（这个参数）理解为 "实现了 `Write` trait 的任何类型的占位符"。还要注意到我们使用 `writeln!(writer, …)` 替换了先前使用的 `println!(…)` ，`println!` 与 `writeln!` 的工作原理相同，但 `println!` 始终被用于标准输出。

现在我们可以测试输出了：

```rust,ignore
{{#include testing/src/main.rs:32:37}}
```

现在要在我们的应用程序代码中使用它，必须在 `main` 对 `find_matches` 的调用中通过添加 [`&mut std::io::stdout()`][stdout] 作为第三个参数。以下是一个 main 函数示例，其基于我们在前几章中看到的内容，并使用我们所提取的 `find_matches` 函数：

```rust,ignore
{{#include testing/src/main.rs:14:22}}
```

[stdout]: https://doc.rust-lang.org/1.39.0/std/io/fn.stdout.html

<aside class="note">

**注意：**
由于 `stdout` 期望的是字节（而非字符串），所以我们使用 `std::io::Write` 代替 `std::fmt::Write`。因此，在我们的测试中给出了一个空 vector 作为 “writer” （它的类型会被推断为 `Vec<u8>`），在 `assert_eq!` 中，我们使用 `b"foo"` ( `b` 前缀让其成为字节串字面量，所以它的类型将会是 `&[u8]` 而非 `&str` )。

</aside>

<aside class="note">

**注意：**
我们也可以让此函数返回 `String`，但这会改变其行为。并非直接写入终端，而是将所有内容收集到一个字符串中，并在最后一次转储所有结果。

</aside>

<aside class="exercise">

**读者练习：**
[`writeln!`] 返回 [`io::Result`] ，因为可能会写入失败，例如当缓冲区已满且无法扩展时，向 `find_matches` 中添加错误处理。

[`writeln!`]: https://doc.rust-lang.org/1.39.0/std/macro.writeln.html
[`io::Result`]: https://doc.rust-lang.org/1.39.0/std/io/type.Result.html

</aside>

我们刚才说了如何使该段代码易于测试。我们需要

1. 明确应用程序的一个核心部分，
2. 将它变成它自己的函数，
3. 使他更灵活。

即使我们的目标是使其可测试，但我们最终得到的结果实际上是一段很地道且可重用的 Rust 代码。这太棒了！


## 将你的代码分割进库和二进制目标

我们到这儿可以再做一件事。到目前为止我们已经将我们所写的一切都写进了 `src/main.rs` 文件。这意味着我们当前的项目产生单个二进制文件。但是我们也可以将我们的代码作为库，就像这样：

1. 将 `find_matches` 函数放入 `src/lib.rs` 这个新文件。
2. 在`fn` 前添加 `pub` (所以它现在是 `pub fn find_matches`) ，使它可以供库用户访问。
3. 从 `src/main.rs` 中移除 `find_matches` 。
4. 在 `fn main`，将 `grrs::` 放在 `find_matches` 之前来调用函数，所以它现在是 `grrs::find_matches(…)`。这意味着将使用的是我们编写的库中的函数！

Rust 处理项目的方式非常灵活，并且考虑提前将哪些放入你的 crate 的库部分是个好主意。例如，你可以考虑先为应用程序的特定逻辑编写库，之后在 CLI 中像使用其他库一样使用它。或者，如果你的项目有多个二进制文件，你可以将公共的功能放进 crate 的库部分。

<aside class="note">

**注意：:**
说说将所有内容放入 `src/main.rs` :如果继续这么做，它将会变得很难阅读。[模块系统][module system] 可以帮助你组织和结构化代码。

[module system]: https://doc.rust-lang.org/1.39.0/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

</aside>


## 通过运行众多测试来测试 CLI 应用程序

到现在，我们已经尽力来测试应用程序的业务逻辑了，即 `find_matches` 函数。这是非常有价值的，并且是迈向经过良好测试的代码库的第一步。（通常，这类测试被称为“单元测试”。）

然而，有很多代码我们没有测试：所有编写来与外界打交道的代码！想象你编写了 main 函数，但是不小心遗留了硬编码字符串，而不是用户提供的路径参数。我们同样应该为此编写测试！（这种级别的测试通常称为 “集成测试”或“系统测试”）

从本质上说，我们仍旧在编写函数并且用 `#[test]` 进行注解，这只是在这些函数内所做的事。例如，我们想要使用项目的主二进制文件，并像运行常规程序一样运行它。我们还将把这些测试放进新目录的新文件中： `tests/cli.rs` 。

<aside>

**补充：**
按照惯例， `cargo` 将在 `tests/` 目录中寻找集成测试。同样，它会在 `benches/` 目录中寻找基准测试，在 `examples`/ 目录中查找示例。这些约定还扩展约定了你的主要源代码：库有一个 `src/lib.rs` 文件，主二进制文件是 `src/main.rs`，或者，如果有多个二进制文件，Cargo 期望它们在 `src/bin/<name>.rs`。遵循这些约定将使得习惯于阅读 Rust 代码的人更容易发现你的代码库。（译者注：Rust 2018 与 Rust 2015 的约定目录组织方式会略有差别）
</aside>

回顾一下， `grrs` 是在文件中搜索字符串的小工具。我们前面已经测试了可以找到匹配项。让我们考虑一下我们可以测试的其他功能。

这儿是我所想出的几个。

- 如果文件不存在会发生什么？
- 没有匹配时输出是什么？
- 当我们忘记一个（或两个）参数时，程序是否以错误消息的形式退出？

这些都是有效的测试用例。我们还应该为 ”快乐 path“（"happy path"）添加测试用例，也即是我们至少找到一个匹配项并进行打印。

为了简化这些测试，我们将使用 [`assert_cmd`] crate ，它有一堆简洁的帮手，允许我们运行我们的主二进制文件并且查看其行为。此外，我们还将添加 [`predicates`] crate ，该 crate 帮助我们编写 `assert_cmd` 可以测试的断言（并且有很好的错误消息）。我们添加这些依赖不是在主清单中（译者注：也就是不在 `Cargo.toml`的 [dependencies] 块中添加），而是在 `Cargo.toml` 中的 "dev dependencies" 块。它们只在开发 crate 时需要，而使用时则不需要。


```toml
{{#include testing/Cargo.toml:11:13}}
```

[`assert_cmd`]: https://docs.rs/assert_cmd
[`predicates`]: https://docs.rs/predicates

这听起来像有很多设置。不过让我们深入研究并创建 `tests/cli.rs` 文件：

```rust,ignore
{{#include testing/tests/cli.rs:1:15}}
```

你可以用 `cargo test` 运行上面我们编写的测试。第一次可能会花费较长时间，因为 `Command::cargo_bin("grrs")` 需要编译主二进制文件。 

## 生成测试文件

我们刚才所看到的测试仅当输入文件不存在时检查程序是否写入错误信息。这是个很重要的测试：现在让我们测试下，我们将实际打印在文件中找到的匹配项。

我们需要有一个我们知道内容的文件，以便我们可以知道程序应该返回的内容，并且在代码中检查此期望内容。一个想法是将带有自定义内容的文件添加到项目中，并在测试中使用该文件。另一个想法是在我们的测试中创建临时文件。在本教程中，我们将使用后一种方法。主要是因为它更灵活，在其他情况下也是能工作；例如，当你测试更改文件的程序时。

要创建这些临时文件，我们将会使用 [`tempfile`] crate 。让我们将其添加到 `Cargo.toml` 文件的 `dev-dependencies` 块:

```toml
tempfile = "3"
```

[`tempfile`]: https://docs.rs/tempfile/3/tempfile/

这儿是个新的测试用例（你可以在另一个测试用例下面编写），首先创建临时文件(一个“已命名的”文件，以便我们获取它的路径)，用一些文本填充它，然后运行程序看看是否得到了正确的输出。当 `file` 超出作用域(在函数的末尾)时，实际的临时文件将自动被删除。

```rust,ignore
{{#include testing/tests/cli.rs:17:34}}
```

<aside class="exercise">

**读者练习：**
添加传递空字符串作为 pattern 的集成测试。根据需求调整程序。

</aside>

## 要测试什么？

当然，编写集成测试固然很有趣，但编写它们也会花费一些时间，同时还要在程序的行为发生变化时更新它们。为了确保你明智地利用时间，你应该问自己该测试什么。

通常，为用户可以观察到的所有类型的行为编写集成测试是一个好主意。这意味着你不用覆盖到所有极端情况：通常只要有不同类型的示例并依靠单元测试来覆盖极端情况就足够了。

不要把测试的重点放在你不能主动控制的事情上是个好点子，测试为你生成的 `--help` 的确切布局是个坏点子。相反，你可能值只要检查是否存在某些元素。

依赖于程序的性质（Depending on the nature of your program），你还可以尝试添加更多的测试技术。例如，如果你提取了程序的某些部分，并发现自己编写了大量的用例作为单元测试，同时试图找出所有的极端用例，那么你应该研究一下 [`proptest`] 。如果你有一个使用任意文件并对其进行解析的程序，那么请尝试编写 [fuzzer] 用来寻找极端情况下的 bug 。

[`proptest`]: https://docs.rs/proptest
[fuzzer]: https://rust-fuzz.github.io/book/introduction.html

<aside>

**补充：**
你可以在[本书仓库][src]中找到本章中使用的完整且可运行的源代码。

[src]: https://github.com/rust-cli/book/tree/master/src/tutorial/testing

</aside>
