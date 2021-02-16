# 一些有用的 crate
总是会有一些新的 crate 发布，有些 crate 在命令行应用的开发中会很有用。

## 本书中所引用过的 crate
- [anyhow](https://crates.io/crates/anyhow) - 提供 `anyhow::Error` 以进行简单的错误处理
- [asset_cmd](https://crates.io/crates/assert_cmd) - 简化 CLI 的集成测试
- [atty](https://crates.io/crates/atty) - 检测应用程序是否运行在 tty 上。
- [clap-verbosity-flag](https://crates.io/crates/clap-verbosity-flag) - 添加 `--verbose` 标签到 structopt CLI
- [clap](https://crates.io/crates/clap) - 命令行参数解析器
- [confy](https://crates.io/crates/confy) - 无样板的配置管理
- [convey](https://crates.io/crates/convey) - 简化人机输出
- [crossbeam-channel](https://crates.io/crates/crossbeam-channel) - 为消息传递提供多生产者——多消费者 channel
- [ctrlc](https://crates.io/crates/ctrlc) - 简易 ctrl-c 处理程序
- [env_logger](https://crates.io/crates/env_logger) - 通过环境变量实现日志配置
- [exitcode](https://crates.io/crates/exitcode) - 系统退出码常量
- [human-panic](https://crates.io/crates/human-panic) - panic 消息处理程序
- [indicatif](https://crates.io/crates/indicatif) - 进度条和微框
- [log](https://crates.io/crates/log) - 在实现之上提供日志抽象
- [predicates](https://crates.io/crates/predicates) - 实现布尔值谓词函数（boolean-valued predicate functions）
- [proptest](https://crates.io/crates/proptest) - 属性测试框架
- [serde_json](https://crates.io/crates/serde_json) - 序列化、反序列化为 JSON
- [signal-hook](https://crates.io/crates/signal-hook) - 处理 UNIX 信号
- [structopt](https://crates.io/crates/structopt) - 解析命令行参数为一个结构体
- [tokio](https://crates.io/crates/tokio) - 异步运行时
- [wasm-pack](https://crates.io/crates/wasm-pack) - 用于构建 WebAssembly 的工具

## 其他 crate
由于众多的 Rust ceate 在持续不断地变化，一个查找 crate 的好地方是 [lib.rs](https://lib.rs/) 的 crate 索引。以下是一些可能会对构建 CLI 有用的（ lib.rs 中索引的）特定类别：

- [命令行界面](https://lib.rs/command-line-interface)
- [配置](https://lib.rs/config)
- [数据库接口](https://lib.rs/database)
- [编码](https://lib.rs/encoding)
- [文件系统](https://lib.rs/filesystem)
- [HTTP 客户端](https://lib.rs/web-programming/http-client)
- [操作系统](https://lib.rs/os)
