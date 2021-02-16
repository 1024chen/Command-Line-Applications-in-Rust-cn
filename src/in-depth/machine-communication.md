# 与机器交互

当你能够组合命令行工具的时候，他们的威力就会真正显现出来。这不是一个新的想法：实际上，这是 [Unix 哲学][Unix philosophy]中的一句话：

> 期望每个程序的输出都可能成为另一个未知程序的输入。

[Unix philosophy]: https://en.wikipedia.org/wiki/Unix_philosophy

如果我们的程序能够满足这个期望，我们的用户也会理所当然地很高兴。为了确保这能很好地工作，我们不仅应该为人提供漂亮的输出，还应该提供适合其他程序需求的版本。让我们看看如何做到这点。

<aside>

**补充：**

确保已经先阅读了本教程中 [CLI 输出的章节][output]。它涵盖了如何将输出写入终端。

[output]: ../tutorial/output.html

</aside>

## 谁来读取输出？

第一个要问的问题是：我们的输出是针对彩色终端前的人还是另一个程序？为了回答这个问题，我们可以使用像 [atty] 这样的 crate：

[atty]: https://crates.io/crates/atty

```rust,ignore
use atty::Stream;

if atty::is(Stream::Stdout) {
    println!("I'm a terminal");
} else {
    println!("I'm not");
}
```

根据谁将读取我们的输出，我们可以添加额外的信息。人们更喜欢彩色，例如，如果你在任意一个 Rust 项目下运行 `ls`，你可能会看到像这样的一些东西：

```console
$ ls
CODE_OF_CONDUCT.md   LICENSE-APACHE       examples
CONTRIBUTING.md      LICENSE-MIT          proptest-regressions
Cargo.lock           README.md            src
Cargo.toml           convey_derive        target
```

由于这是为人类所制作的样式，因此在大多数配置中，它甚至会打印一些彩色的名字来表明它们是目录（例如 `src`）。如果你将其 pipe 到一个文件，或者像 `cat` 这样的程序，`ls` 将会调整其输出。它将会在自己的行中打印每个条目，而不是使用适合终端窗口的列，它也不会触发任何颜色（来进行着色）。

```console
$ ls | cat
CODE_OF_CONDUCT.md
CONTRIBUTING.md
Cargo.lock
Cargo.toml
LICENSE-APACHE
LICENSE-MIT
README.md
convey_derive
examples
proptest-regressions
src
target
```

## 面向机器的简单输出格式

历史上，命令行工具产生的唯一输出类型是字符串。对于终端前的人来说，这通常是很好的，因为他们可以阅读文本并理解其含义。但是其他程序通常没有这种能力：它们理解 `ls` 之类工具的输出的唯一方式是，该程序作者（在程序中）包含了一个恰好适用于任何 `ls` 输出的解析器。

这通常意味着输出仅限于容易解析的内容。 TSV （制表符分隔值，即Tab）这样的格式非常流行，其每个记录都在自己的行中，每行包含了用制表符分隔的内容。这些基于文本行的简单格式允许将像 `grep` 这类的工具用于 `ls` 这类工具的输出。`| grep Cargo` 不关心你的行是来自于 `ls` 还是文件，它只会逐行过滤。

这样做的缺点是，你不能使用简单的 `grep` 调用来过滤 `ls` 所给的所有目录。为此，每个目录项都需要携带额外数据。

## 面向机器的 JSON 输出

制表符分隔值是一种简单的输出结构化数据的方式，但是这要求其他程序知道期望哪些字段（以及以哪种顺序），并且很难输出不同类型的消息。例如，假设我们的程序想要向使用者发送消息说正在等待下载，然后输出一条消息描述其（通过下载）获得的数据。这些消息是非常不同类型的消息，如果试图将它们统一进 TSV 输出，就需要我们发明一种区分它们的方法。同样当我们想要打印包含两个不同长度的列表项的消息时，也是这样的。

不过，最好选择一种在大多数编程语言、环境中都可以轻松解析的格式。因此，在过去的几年里，许多应用程序都拥有了以 [JSON] 输出数据的能力。它足够简单，几乎每种语言都存在其解析器（译者注：即 JSON 解析器）；但它又足够强大，，在很多情况下都有用。当然它也是一种人类可读的文本格式，许多人也致力于快速解析 JSON 数据和将数据序列化为 JSON 的实现。

[JSON]: https://www.json.org/

在上面的描述中，我们已经讨论过了由我们程序所写出的 “消息”。这儿有一个考虑输出的好方法：你的程序不一定只输出一个数据块，实际上在运行的时候可能会发出许多不同的信息。支撑此想法的一个简单方法是在输出 JSON 时为每条消息编写一个 JSON 文档 （JSON document），并且将每个 JSON 文档放到新行中（有时称之为 [行分割 JSON][jsonlines]）。这使得实现像使用常规的 `println!` 一样简单。

[jsonlines]: https://en.wikipedia.org/wiki/JSON_streaming#Line-delimited_JSON

下面是一个简单的示例，使用 [serde_json] 中的 `json!` 宏来在 Rust 源代码中快速地写入有效的 JSON ：

[serde_json]: https://crates.io/crates/serde_json

```rust,ignore
{{#include machine-communication.rs:1:22}}
```

此处是其输出：

```console
$ cargo run -q
Hello world
$ cargo run -q -- --json
{"content":"Hello world","type":"message"}
```

使用 `-q` 运行 `cargo` 将禁止其常规输出。 `--` 之后的参数将传递给我们的程序。

### 实例: ripgrep

_[ripgrep]_ 可以说是 _grep_ 或 _ag_ 的替代，并且是用 Rust 写的。默认情况下，它将会产生如下输出：

[ripgrep]: https://github.com/BurntSushi/ripgrep

```console
$ rg default
src/lib.rs
37:    Output::default()

src/components/span.rs
6:    Span::default()
```

但是传递 `--json` 就会打印：

```console
$ rg default --json
{"type":"begin","data":{"path":{"text":"src/lib.rs"}}}
{"type":"match","data":{"path":{"text":"src/lib.rs"},"lines":{"text":"    Output::default()\n"},"line_number":37,"absolute_offset":761,"submatches":[{"match":{"text":"default"},"start":12,"end":19}]}}
{"type":"end","data":{"path":{"text":"src/lib.rs"},"binary_offset":null,"stats":{"elapsed":{"secs":0,"nanos":137622,"human":"0.000138s"},"searches":1,"searches_with_match":1,"bytes_searched":6064,"bytes_printed":256,"matched_lines":1,"matches":1}}}
{"type":"begin","data":{"path":{"text":"src/components/span.rs"}}}
{"type":"match","data":{"path":{"text":"src/components/span.rs"},"lines":{"text":"    Span::default()\n"},"line_number":6,"absolute_offset":117,"submatches":[{"match":{"text":"default"},"start":10,"end":17}]}}
{"type":"end","data":{"path":{"text":"src/components/span.rs"},"binary_offset":null,"stats":{"elapsed":{"secs":0,"nanos":22025,"human":"0.000022s"},"searches":1,"searches_with_match":1,"bytes_searched":5221,"bytes_printed":277,"matched_lines":1,"matches":1}}}
{"data":{"elapsed_total":{"human":"0.006995s","nanos":6994920,"secs":0},"stats":{"bytes_printed":533,"bytes_searched":11285,"elapsed":{"human":"0.000160s","nanos":159647,"secs":0},"matched_lines":2,"matches":2,"searches":2,"searches_with_match":2}},"type":"summary"}
```

如你所见，每一个 JSON 文档都是一个包含 `类型` 字段的对象（map，映射）。这将使我们能编写一个简单的 `rg` 的前端，以便在这些文档到来时读取它们，并显示匹配（以及它们所在的文件），即使 _ripgrep_ 仍在搜索。

<aside>

**补充：**
这是 Visual Studio Code 使用 _ripgrep_ 进行代码搜索的方式。

</aside>

## 人机输出抽象

[convey] 是一个开发中的库，其试图让适合人类和机器的格式输出消息变得更容易。你定义自己的消息类型，并实现一个 `Render` trait （手动，借助于宏，或使用 derive 属性）来说明它们应该怎样被格式化。当前，它支持打印人工输出（包括自动检测是否应着色），写入 JSON 文档（到 `stdout` 或文件中），或同时支持二者。

[convey]: https://crates.io/crates/convey

即使你不适应此库，你或许也应该编写一个适合你的用例的类似抽象。

## 如何处理我们输入的 pipe 输入

<aside class="todo">

**TODO:**

讨论如何使用 stdin （请参阅 [#95](https://github.com/rust-lang-nursery/cli-wg/issues/95)）

</aside>