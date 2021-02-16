# 使用配置文件

处理配置可能会很烦人，特别是当支持多个操作系统时，他们可能都有自己的存放短期和长期文件的地方。

对此有多种解决方案，有些解决方案比其他的更底层。

最简单的是使用 [`confy`] crate。它会要求你提供应用程序的名字，并要求你通过一个 `struct`（也就是 `Serialize`, `Deserialize`） 指定配置布局，并且会找出剩余的部分！

```rust,ignore
#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    name: String,
    comfy: bool,
    foo: i64,
}

fn main() -> Result<(), io::Error> {
    let cfg: MyConfig = confy::load("my_app")?;
    println!("{:#?}", cfg);
    Ok(())
}
```

这非常简单易用，当然也放弃了可配置性，但是如果你只需要简单的 config ，这个 crate 或许适合你。

[`confy`]: https://docs.rs/confy/0.3.1/confy/

## 配置环境变量

<aside class="todo">
**TODO**

1. 评估现有的 crate
2. 命令行参数 + 多个配置 + 环境变量
3.  [`configure`] 可以做到所有这些么？其有好的 wrapper 吗?

</aside>

[`configure`]: https://docs.rs/configure/0.1.1/configure/
