# 打包和发布 Rust 工具

如果你确信你的程序已经准备好供其他人使用了，那么是时候打包和发布它了！

有几种打包和发布的办法，我们将会介绍其中三种，从“最快捷设置”到“对用户最方便”

## 最快捷： `cargo publish`

最简单的发布应用程序的办法是使用 cargo 。还记得如何将外部依赖添加到我们的项目吗？ Cargo 从 crate 默认的 “crate 注册局”（"crate registry"）—— [crates.io] 下载 crate 。使用 `cargo publish` ，你也可以将 crates 发布到 [crates.io] 。这适用于所有 crate ，包括具有二进制目标文件的 crate 。

将 crate 发布到 [crates.io][crates.io] 是很简单的：如果你还没有账号，就在 [crates.io][crates.io] 上创建一个。目前，这是通过 GitHub 账号授权完成的 ，所以你需要先拥有一个 GitHub 账号（并且已经在浏览器上登录了）。接下来，在本地机器上使用 Cargo 登录。为此，转到你的 [crates.io 账户页面][crates.io account page] ，创建一个新的令牌（token），然后运行 `cargo login <your-new-token>` 。每台计算机上只需要做一次。你可以在 Cargo 的[发布指南][publishing guide]中了解更多关于此的信息。

现在你已经知道了 Cargo 和 crates.io ，那么说明你已经准备好发布 crate 了。在你发布一个新 crate （或版本）之前，最好再次打开 `Cargo.toml` 并确保添加了必要的元数据。你可以在[Cargo 清单格式][cargo's manifest format]文档中找到所有你可以设置的可能的字段。以下是一些常见条目的速览：

```toml
[package]
name = "grrs"
version = "0.1.0"
authors = ["Your Name <your@email.com>"]
license = "MIT OR Apache-2.0"
description = "A tool to search files"
readme = "README.md"
homepage = "https://github.com/you/grrs"
repository = "https://github.com/you/grrs"
keywords = ["cli", "search", "demo"]
categories = ["command-line-utilities"]
```

<aside class="note">

**注意：**
该样例包含了 Rust 项目通用选项的强制许可证字段：编译器本身也使用相同的许可证（译者注：即 `license = "MIT OR Apache-2.0"` 字段）。它同样也指向一个 `README.md` 文件。它应该包含对项目的简要描述，并且这个 `README.md` 文件不仅被作为你在 crates.io 页面，同样也会被作为 GitHub 的默认库页面。（译者注：其实如果经常浏览 crates.io ，你会发现很多项目的 page 和 GitHub 默认展示的 readme.md 是一样的，当然渲染的样式略有差别，这也就看出来 crates.io + Cargo + GitHub或其他托管 的精巧之处了。）

</aside>

[crates.io]: https://crates.io/
[crates.io account page]: https://crates.io/me
[publishing guide]: https://doc.rust-lang.org/1.39.0/cargo/reference/publishing.html
[cargo's manifest format]: https://doc.rust-lang.org/1.39.0/cargo/reference/manifest.html

### 怎样从 crates.io 安装二进制文件

我们已经看到了如何在 crates.io 上发布 crate ，你可能想要知道如何安装它。与库不同，当运行 `cargo build` （或类似命令）时 cargo 会为你下载并编译，你需要告诉 cargo 明确安装二进制文件。

这通过 `cargo install <crate-name>` 来完成。默认情况下，它将下载 crate ，编译所有其包含的二进制目标文件（在 "release" 模式下，可能需要多花点时间）并将它们复制进 `~/.cargo/bin/` 目录（请确保你的 shell 能知道在此找到二进制文件！）。（译者注：也就是说你的 `~/.cargo/bin/` 目录得在环境变量中）

也可以从 git 库中安装 crate ，仅安装指定 crate 的二进制文件，并且指定一个其他的目录来安装它们。更多细节请参阅 `cargo install --help` 命令。

### 什么时候使用

`cargo install` 是安装 二进制 crate 的一个简单办法。开发人员使用起来非常方便，但是有一些很明显的缺点：因为它将会从头开始编译源代码，所以你开发的工具的用户需要在他们的机器上安装 Rust、cargo 和 所有其他的你的程序所要求的系统依赖项，同时编译大型的 Rust 代码库也可能会花上一些时间。（译者吐槽：可能俩字去掉，编译时间不是一些好嘛，经历过痛苦的人都懂）

此外，也没有简便的方法来使用 cargo 更新工具：用户需要在某个时候再次运行 `cargo install` ，并且传递 `--force` 标签覆盖旧版的二进制文件。这是一项[缺失的特性][cargo-issue-2082]，然而，你可以用[这样的][cargo-update]子命令来安装并添加它。（译者注：随着 Rust 与 Cargo 版本的推进与问题修复，上面的问题已不再是问题了）

[cargo-issue-2082]: https://github.com/rust-lang/cargo/issues/2082
[cargo-update]: https://crates.io/crates/cargo-update

最好将此用于针对其他 Rust 开发者 的发行工具。例如：许多 cargo 子命令例如 `cargo-tree` 或 `cargo-outdated` 都可以用它来安装。

## 分发二进制文件

Rust 是一种可编译为本地代码（native code ，实际上就是人类不可读的本地二进制可执行代码） 并且默认会静态链接所有依赖项的语言。当你在包含一个名叫 `grrs` 的二进制（crate）的程序上运行 `cargo build` ，你最终会得到一个名叫 `grrs` 的文件。尝试一下：使用 `cargo build`，将会在 `target/debug/grrs` ，并且当你运行 `cargo build --release`，将会在 `target/release/grrs` 。除非你使用了明确需要在目标系统上安装外部库的 crate （例如，系统版本的 OpenSSL），否则此二进制文件将只依赖于通用系统库。这意味着你（编译或下载）得到这么一个文件，将它发给与你使用相同操作系统的人，他们就可以运行这个文件。

这已经相当强大了！它解决了我们刚才的 `cargo install` 的两个缺点：它不需要在用户的机器上安装 Rust ，而且不用花时间编译就能立即运行二进制文件。

所以，正如我们所见，`cargo build`已经为我们构建了二进制文件。唯一的问题是，这不能保证在所有平台上都能正常工作。如果你的 Windows 机器上运行了 `cargo build` ，你将不会得到一个在 Mac 上默认可工作的二进制文件。那么有没有一种办法可以为所有感兴趣的平台（也就是你想要生成的平台）自动生成这些二进制文件呢？（译者注：如果你看过[官方的][rustc-book-en]或者[我所翻译的][rustc-book-ch] `rustc 手册`，你应该一眼就知道 rustc 或 cargo 对此的解决办法）

[rustc-book-en]:https://doc.rust-lang.org/rustc/index.html
[rustc-book-ch]:https://github.com/1024chen/The-rustc-book-cn

### 在 CI 上构建二进制发行版

如果你的工具是开源的并且托管在 GitHub 上，那么设置免费的 CI（持续集成）服务例如 [Travis CI] 是非常容易的。（还有运行在其他托管平台上的服务，但是 Travis 十分流行。）每次将代码更改 push 到库的时候就会在虚拟机中运行命令，这些命令和运行的机器类型（译者注：即选择的虚拟机运行环境）是可配置的。例如：一个好想法是在安装了 Rust 和一些通用构建工具的机器上运行 `cargo test` 。如果失败了，你就可以知道在最近的代码改进中存在问题了。（译者注：本书原项目就是使用 Travis 进行持续集成的）

[Travis CI]: https://travis-ci.com/

我们也可以以此构建二进制文件并且上传到 GitHub ！缺失，如果我们运行 `cargo build --release` 并上传二进制文件到某处，就说明我们已经准备完了，对吗？并不完全是的，我们依然需要确保我们构建的二进制文件与尽可能多的系统兼容。例如，在 Linux 上，我们可以不以当前系统为目标进行编译，而是将 `x86_64-unknown-linux-musl` 作为编译目标，以不依赖默认系统库。在 macOS 上，我们可以设置 `MACOSX_DEPLOYMENT_TARGET` 为 `10.7` 以仅依赖于 10.7 和更早版本的系统功能。

你可以在[这里][wasm-pack-travis]看到一个使用该方法构建 Linux 和 macOS 二进制文件和[这里][wasm-pack-appveyor]的构建 Windows (使用 AppVeyor) 二进制文件的示例。

[wasm-pack-travis]: https://github.com/rustwasm/wasm-pack/blob/51e6351c28fbd40745719e6d4a7bf26dadd30c85/.travis.yml#L74-L91
[wasm-pack-appveyor]: https://github.com/rustwasm/wasm-pack/blob/51e6351c28fbd40745719e6d4a7bf26dadd30c85/.appveyor.yml

另一种方法是使用包含所有需要用来构建二进制文件的工具的预构建（Docker）镜像。这也使得我们更简单地将更多平台作为目标。 [trust] 项目包含了可以包含进项目的脚本，以及关于如何设置的说明。它还包括对使用 AppVeyor 的 Windows 的支持。

如果你更愿意在本地进行设置并且在自个儿的机器上生成发行文件（ release files ），你依然有必要看一下 trust 项目。它在内部使用 [cross][cross] ，其工作方式与 Cargo 类似，但是将命令转发给 Docker 容器中的 cargo 进程。镜像的定义也可以在 [cross 的库][cross]中找到。
[cross' repository][cross].

[trust]: https://github.com/japaric/trust
[cross]: https://github.com/rust-embedded/cross

### 如何安装这些二进制文件
你可以将用户指向发行页面，页面可能看起来[像这样][wasm-pack-release]，（译者注：示例页面拉到最下面）并且她们可以下载我们刚刚创建的组件（ artifacts ）。我们刚生成的发行组件没什么特殊的：最终它们只是包含了我们二进制文件的归档文件！这意味着你的工具的使用者可以使用其浏览器下载它们，解压缩它们（通常会自动发生），并将二进制文件复制到它们喜欢的位置。

[wasm-pack-release]: https://github.com/rustwasm/wasm-pack/releases/tag/v0.5.1

这确实需要一些手动 “安装” 程序的经验，因此你需要在 README 文件中添加有关如何安装此程序的部分。（译者注：说是安装，其实更应该形容为下载）

<aside class="note">

**注意：**
如果你使用 [trust] 构建你的二进制文件并将其添加到了 GitHub （对应程序）发行版本中，你也可以告诉人们运行 `curl -LSfs https://japaric.github.io/trust/install.sh | sh -s -- --git your-name/repo-name` （如果你认为此可以简化操作）。

</aside>

### 什么时候使用

拥有二进制发行版本通常是一个好主意，这几乎没有任何缺点。这不能解决用户必须手动安装和更新你的工具的问题，但是他们可以快速获取最新版本而无需安装 Rust 。                                                                       

### 除了二进制文件之外还要打包什么

现在，当用户下载了我们的发行版本之后，他们可能会获得一个仅包含二进制文件的 `.tar.gz` （之类的）文件。所以，在我们的示例项目中，他们只会得到一个可运行的 `grrs` 文件。但是我们的库中已经有一些他们可能想要的文件。例如，告诉用户如何使用这个工具的 README 文件还有 license 文件。因为在之前已经有了它们，所以很容易添加。

还有一些有趣的文件，特别适用于命令行工具：除了 README 文件之外，还提供一个手册页面，以及向 shell 添加可能的标签补全的配置文件，如何？你可以手动编写这些文件，但是我们使用的参数解析库（ clap ）可以为我们生成所有的这些文件。有关更多详细信息，请参阅深入讨论中的 [这个章节][clap-man-pages]

[clap-man-pages]: ../in-depth/docs.html


## 将应用程序放入包存储库

到目前我们所看到的两种方法都不是通常在机器上安装软件的方法。特别是大多数操作系统上使用全局包管理器安装的命令行工具。这样做的好处对用户来说是显而易见的：如果可以像安装其他的工具一样安装程序，就不需要考虑如何安装程序。这些包管理器还允许用户在新版本可用时更新程序。

遗憾的是，支持不同的系统意味着你必须了解这些不同的系统是如何工作的。对于一些来说，可能就只需要向库中添加一个文件（例如，为 macOS 用户的 `brew` 添加一个像[这样][rg-formula]的 formula 文件），但是对于其他的，你可能会经常需要自己发送补丁，并将你的工具添加到它们的存储库中。有一些很有用的工具，例如 [cargo-rpm](https://crates.io/crates/cargo-rpm) ， [cargo-deb](https://crates.io/crates/cargo-deb) 和 [cargo-aur](https://crates.io/crates/cargo-aur)，但是描述它们如何工作以及如何正确为这些不同的系统打包工具不在本章的范围内。

[rg-formula]: https://github.com/BurntSushi/ripgrep/blob/31adff6f3c4bfefc9e77df40871f2989443e6827/pkg/brew/ripgrep-bin.rb

相反，让我们看一下用 Rust 编写的工具，可以在许多不同的包管理器中使用。

### 一个示例： ripgrep

[ripgrep][ripgrep] 是用 Rust 编写的 `grep`/`ack`/`ag` 的替代品。它相当地成功并且被打包用于许多操作系统：只需要查看它的 README 文件中的 [安装部分][rg-install] ！ 

注意它列出了一些如何安装的不同选项：它以指向包含二进制文件的 GitHub 发行版本的链接开头，所以你可以直接下载；然后它还列出了如何使用一堆不同的软件包管理器安装它；最后，你还可以使用 `cargo install` 安装它。

这儿似乎有个好点子：不选择本章节介绍的任何一个方法，而是从 `cargo install` 开始，添加二进制版本，最后才使用系统包管理器发布你的工具。

[ripgrep]: https://github.com/BurntSushi/ripgrep
[rg-install]: https://github.com/BurntSushi/ripgrep/tree/31adff6f3c4bfefc9e77df40871f2989443e6827#installation
