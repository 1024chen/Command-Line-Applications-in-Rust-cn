use structopt::StructOpt;
use serde_json::json;

/// 在文件中搜索一个模式并显示包含该模式的行。
#[derive(StructOpt)]
struct Cli {
    /// 输出JSON而不是人类可读的消息
    #[structopt(long = "json")]
    json: bool,
}

fn main() {
    let args = Cli::from_args();
    if args.json {
        println!("{}", json!({
            "type": "message",
            "content": "Hello world",
        }));
    } else {
        println!("Hello world");
    }
}
