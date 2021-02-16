# 为命令行应用呈现文档

CLI 应用的文档通常由命令中 `--help` 部分和手册（`man`）页面组成。 

当使用 `clap`v3（撰写本文时，在未发布的 beta 版本中）时通过 `man` 后端 两者都可以自动生成。

```rust,ignore
#[derive(Clap)]
pub struct Head {
    /// file to load
    #[clap(parse(from_os_str))]
    pub file: PathBuf,
    /// how many lines to print
    #[clap(short = "n", default_value = "5")]
    pub count: usize,
}
```

其次，你需要使用一个 `build.rs` 在编译时根据应用程序中代码的定义去生成手册文件。

有一些事需要牢记（例如，如何打包二进制文件），但是现在，我们只是将 `man` 文件简单地放在了 `src` 文件夹旁。

```rust,ignore
use clap::IntoApp;
use clap_generate::gen_manuals;

#[path="src/cli.rs"]
mod cli;

fn main() {
    let app = cli::Head::into_app();
    for man in gen_manuals(&app) {
        let name = "head.1";
        let mut out = fs::File::create(name).unwrap();
        use std::io::Write;
        out.write_all(man.render().as_bytes()).unwrap();
    }
}
```

当你现在编译应用程序时，你的项目目录中会有一个 `head.1` 文件。

如果你在 `man` 中打开它，你就会欣赏到你的免费文档了。
