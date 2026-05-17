use crate::parser::ListData;
use std::fs;
use std::io::{self, Write};

pub fn run_cli(file_path: &str) {
        let content = fs::read_to_string(file_path).unwrap_or_else(|e| {
                eprintln!("无法读取文件 {}: {}", file_path, e);
                std::process::exit(1);
        });

        let mut data = match ListData::from_string(&content) {
                Ok(d) => d,
                Err(e) => {
                        eprintln!("解析错误: {}", e);
                        std::process::exit(1);
                }
        };

        println!("已加载: {}", file_path);
        println!("输入命令 (输入 exit() 退出):");
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

                                fs::write(file_path, &data.to_string()).unwrap_or_else(|e| {
                                        eprintln!("警告: 无法保存文件: {}", e);
                                });
                        }
                        Err(e) => {
                                eprintln!("错误: {}", e);
                        }
                }

                print!("> ");
                io::stdout().flush().unwrap();
        }
}
