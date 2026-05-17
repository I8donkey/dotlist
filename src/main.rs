use clap::{Parser, Subcommand};
mod parser;
mod cli;
mod ide;

#[cfg(feature = "python")]
mod python_bindings;

#[cfg(feature = "nodejs")]
mod nodejs_bindings;

#[cfg(not(any(feature = "python", feature = "nodejs")))]
mod c_ffi;

#[derive(Subcommand)]
enum Commands {
        /// 创建新的 .list 文件
        New {
                #[arg(help = "文件路径 (例如: data.list)")]
                file: String,
                #[arg(short, long, help = "使用二进制压缩存储格式")]
                binary: bool,
                #[arg(long, help = "初始内容 (可选)")]
                content: Option<String>,
        },
        /// 打开 .list 文件并进入交互模式
        Open {
                #[arg(help = "文件路径")]
                file: String,
                #[arg(short, long, help = "强制使用二进制格式读取")]
                binary: bool,
                #[arg(short, long, help = "自动识别文件格式 (默认)")]
                auto: bool,
        },
        /// 启动 IDE 模式 (需要终端支持)
        Ide {
                #[arg(help = "文件路径")]
                file: String,
                #[arg(short, long, help = "使用二进制格式")]
                binary: bool,
        },
}

#[derive(Parser)]
#[command(name = "list")]
#[command(about = "高性能.list语法处理工具", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
        #[command(subcommand)]
        command: Option<Commands>,
}

fn main() {
        let args = Cli::parse();

        match args.command {
                Some(Commands::New { file, binary, content }) => {
                        cmd_new(&file, binary, content);
                }
                Some(Commands::Open { file, binary, auto }) => {
                        // 逻辑说明:
                        // - 如果指定了 -b/--binary，强制二进制模式
                        // - 如果指定了 -a/--auto 或什么都不指定，使用自动识别（默认）
                        // - auto 参数是为了明确指定自动识别模式
                        cmd_open(&file, binary);
                }
                Some(Commands::Ide { file, binary }) => {
                        if binary {
                                eprintln!("IDE模式暂不支持二进制格式");
                                std::process::exit(1);
                        }
                        let _ = ide::run_ide(&file);
                }
                None => {
                        print_help();
                }
        }
}

fn cmd_new(file_path: &str, use_binary: bool, initial_content: Option<String>) {
        println!("创建新文件: {}", file_path);

        let content = initial_content.unwrap_or_else(|| {
                if use_binary {
                        "[binary_mode];".to_string()
                } else {
                        "[empty];".to_string()
                }
        });

        if use_binary {
                #[cfg(feature = "binary")]
                {
                        match parser::ListData::from_string(&content) {
                                Ok(data) => {
                                        match data.save_binary(file_path) {
                                                Ok(_) => {
                                                        println!("✓ 已创建二进制文件: {}", file_path);
                                                        println!("  大小: {} bytes", std::fs::metadata(file_path).map(|m| m.len()).unwrap_or(0));
                                                        println!("  格式: 压缩二进制 (.listb)");
                                                        println!("\n打开方式:");
                                                        println!("  list open -b {}", file_path);
                                                        println!("  list open -a {}  # 自动识别", file_path);
                                                }
                                                Err(e) => {
                                                        eprintln!("错误: 无法保存二进制文件: {}", e);
                                                        std::process::exit(1);
                                                }
                                        }
                                }
                                Err(e) => {
                                        eprintln!("错误: 解析初始内容失败: {}", e);
                                        std::process::exit(1);
                                }
                        }
                }
                #[cfg(not(feature = "binary"))]
                {
                        eprintln!("错误: 二进制功能未编译，请使用 --features binary 编译");
                        std::process::exit(1);
                }
        } else {
                match std::fs::write(file_path, &content) {
                        Ok(_) => {
                                println!("✓ 已创建文本文件: {}", file_path);
                                println!("  大小: {} bytes", content.len());
                                println!("  格式: 纯文本 (.list)");
                                println!("\n打开方式:");
                                println!("  list open {}", file_path);
                                println!("  list open -a {}  # 自动识别", file_path);
                        }
                        Err(e) => {
                                eprintln!("错误: 无法创建文件: {}", e);
                                std::process::exit(1);
                        }
                }
        }

        println!("\n快速开始:");
        println!("  1. 打开文件进行编辑: list open {}", file_path);
        if !use_binary {
                println!("  2. 或直接查看内容: type {}", file_path);
        }
        println!("  3. 输入 exit() 退出交互模式");
}

fn cmd_open(file_path: &str, force_binary: bool) {
        let is_binary_file = file_path.ends_with(".listb") ||
                           file_path.ends_with(".listbin") ||
                           force_binary;

        if is_binary_file {
                #[cfg(feature = "binary")]
                {
                        match parser::ListData::load_binary(file_path) {
                                Ok(mut data) => {
                                        println!("✓ 已加载二进制文件: {}", file_path);
                                        println!("  格式: 压缩二进制");
                                        println!("  元素数: {}", data.items.len());
                                        run_interactive_binary(&mut data, file_path);
                                }
                                Err(e) => {
                                        if !force_binary {
                                                println!("尝试以文本模式加载...");
                                                cli::run_cli(file_path);
                                        } else {
                                                eprintln!("错误: 无法加载二进制文件: {}", e);
                                                std::process::exit(1);
                                        }
                                }
                        }
                }
                #[cfg(not(feature = "binary"))]
                {
                        eprintln!("错误: 二进制功能未编译");
                        std::process::exit(1);
                }
        } else {
                cli::run_cli(file_path);
        }
}

#[cfg(feature = "binary")]
fn run_interactive_binary(data: &mut parser::ListData, file_path: &str) {
        use std::io::{self, Write};

        println!("\n输入命令 (输入 exit() 退出):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        loop {
                input.clear();
                if io::stdin().read_line(&mut input).is_err() { break; }

                let cmd = input.trim();
                if cmd.is_empty() { print!("> "); io::stdout().flush().unwrap(); continue; }

                match data.execute_command(cmd) {
                        Ok(result) => {
                                println!("{}", result);

                                if let Err(e) = data.save_binary(file_path) {
                                        eprintln!("警告: 无法保存文件: {}", e);
                                }
                        }
                        Err(e) => {
                                eprintln!("错误: {}", e);
                        }
                }

                print!("> ");
                io::stdout().flush().unwrap();
        }
}

fn print_help() {
        println!(r#"
list - 高性能.list语法处理工具

用法:
  list new <文件> [-b] [--content <内容>]   创建新的 .list 文件
  list open <文件> [-b] [-a]                 打开 .list 文件
  list ide <文件>                            启动 IDE 模式

选项:
  -b, --binary    使用/强制使用二进制压缩格式
  -a, --auto      自动识别文件格式 (默认)
  --content       初始内容 (仅用于 list new)

示例:
  list new data.list                     # 创建空文本文件
  list new data.list -b                  # 创建二进制压缩文件
  list new data.listb -b "[1,2,3];"     # 创建带内容的二进制文件
  
  list open data.list                    # 打开文本文件
  list open data.listb                   # 自动识别为二进制
  list open data.list -b                 # 强制二进制模式
  list open data.list -a                 # 自动检测格式
  
  list ide data.list                     # 启动 IDE 模式

文件格式:
  .list   纯文本格式 (可读可编辑)
  .listb  二进制压缩格式 (高性能)

更多帮助: list <command> --help
"#);
}
