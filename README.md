![LICENSE](https://img.shields.io/badge/_LICENCE_-_LGPL--2.0_-orange)  ![Auther](https://img.shields.io/badge/%E4%BD%9C%E8%80%85-i8donkey-blue)
### if you are in english, please read [the english version](README_en.md)

# .list 高性能多语言语法处理工具

## ✨ 特性

- ⚡ **超高性能**: Rust编写
- 🌍 **多语言支持**: Python, Node.js, C/C++, Go, Rust
- 💾 **零拷贝解析**: 内存高效，速度极快
- 🎨 **IDE模式**: 终端UI + 实时语法高亮
- 🔧 **CLI模式**: 命令行交互界面
- 📦 **跨平台**: Windows/Linux/macOS
- 📁 **二进制压缩**: 支持压缩存储，节省空间

---

## 🚀 快速开始

### 1. 编译核心库 (Rust)

```bash
cargo build --release --features python,binary
```

### 2. CLI模式使用

```bash
# 打开文件并交互操作
list open example.list

# 创建新文件（文本模式）
list new mydata.list

# 创建新文件（二进制压缩模式）
list new mydata.list -b
```

---

## 📝 命令语法

### 读取操作

| 命令 | 功能 | 示例 |
|------|------|------|
| `[n]` | 读取第 n 个元素 | `[0]` |
| `[n][m]` | 读取第 n 个元素的第 m 个子元素 | `[2][1]` |
| `[n]/` | 读取第 n 个元素的键 | `[0]/` |
| `[n]<` | 读取第 n 个元素的值 | `[0]<` |

### 修改操作

| 命令 | 功能 | 示例 |
|------|------|------|
| `.+[n]` | 在第 n 个位置创建新数组 | `.+[3]` |
| `.+[n]=值` | 将值追加到第 n 个数组 | `.+[3]=10` |
| `.+[n]>[m]=值` | 将值插入到第 n 个数组的第 m 位置 | `.+[3]>[1]=10` |
| `.+>[n]=[...]` | 替换第 n 个数组为新内容 | `.+>[3]=[1,2,3]` |
| `.- >[n]` | 删除第 n 个元素 | `.- >[3]` |

### 移动/插入操作

| 命令 | 功能 | 示例 |
|------|------|------|
| `.>[src]>[dest]` | 把 [src] 移动到 [dest] 的末尾 | `.>[0]>[1]` |
| `.>[src]>[dest]<[pos]` | 把 [src] 插入到 [dest][pos] 的前面 | `.>[0]>[1]<[2]` |
| `.>[src]>[dest]>[pos]` | 把 [src] 插入到 [dest][pos] 的后面 | `.>[0]>[1]>[2]` |

### 查询操作

| 命令 | 功能 | 示例 |
|------|------|------|
| `.?关键词` | 在根级别查询关键词 | `.?apple` |
| `.?关键词>[n]` | 在 [n] 中查询关键词 | `.?apple>[0]` |
| `.?关键词>[n]-[m]` | 在 [n] 到 [m] 中查询关键词 | `.?apple>[0]-[3]` |

### 打印操作

| 命令 | 功能 | 示例 |
|------|------|------|
| `.print>[n]` | 打印 [n] 的内容 | `.print>[0]` |
| `.print>关键词` | 打印关键词 | `.print>hello` |
| `.print>$变量` | 打印变量 | `.print>$myvar` |
| `.print>[n]-[m]` | 打印 [n] 到 [m] 的内容 | `.print>[0]-[2]` |

### 变量操作

| 命令 | 功能 | 示例 |
|------|------|------|
| `$var=值` | 设置变量 | `$name=John` |
| `$var` | 读取变量 | `$name` |
| `$vars` | 显示所有变量 | `$vars` |

### 语言设置

| 命令 | 功能 | 示例 |
|------|------|------|
| `.setlang=语言` | 设置语言 | `.setlang=zh` |
| `.sl=语言` | 设置语言（简写） | `.sl=en` |

### 其他命令

| 命令 | 功能 | 示例 |
|------|------|------|
| `.h` 或 `.help` | 显示帮助信息 | `.h` |
| `exit()` | 退出程序 | `exit()` |

---

## 📁 数据文件格式

### 基础语法

```
# 数组
[元素1, 元素2, 元素3]

# 键值对
key:value

# 嵌套结构
[key1:value1, key2:value2, [子数组]]

# 多行数据（用分号分隔）
[apple,banana,cherry]; [red,green,blue]; hello world;
```

### 示例数据文件 (example.list)

```
[apple,banana,cherry]
[red,green,blue]
[key1:value1, key2:value2]
```

---

## 🌍 多语言绑定

### Python (PyO3)

#### 安装
```bash
pip install maturin
maturin develop --release --features python,binary
```

#### 使用示例
```python
from dotlist import PyListData

# 创建数据
data = PyListData("[apple,banana,cherry]; [red,green,blue];")

# 操作
print(data.get([0]))           # [apple,banana,cherry]
print(data.to_string())        # 完整数据
data.append(0, "new_item")     # 追加
data.insert(0, 1, "item")      # 插入
data.delete(0)                  # 删除
data.replace(0, "[a,b,c]")     # 替换
result = data.execute_command("[0]")  # 执行命令
```

---

### Node.js (napi-rs)

#### 安装
```bash
npm install -g @napi-rs/cli
napi build --release --features nodejs
```

#### 使用示例
```javascript
const { JsListData } = require('dotlist');

// 创建数据
const data = new JsListData('[apple,banana,cherry]; [red,green,blue];');

// 异步操作
async function main() {
        console.log(data.toString());
        console.log(data.get([0]));           // 读取
        await data.append(0n, 'value');       // 追加
        await data.executeCommand('[0]');     // 执行命令
}
main();
```

---

### C/C++ (FFI)

#### 头文件生成
```bash
cbindgen --crate dotlist --output bindings/c/dotlist.h
```

#### 使用示例
```c
#include <stdio.h>
#include "dotlist.h"

int main() {
        // 创建数据
        const char* content = "[1,2,3,4,5];";
        CListData* data = list_data_from_string(content);

        // 读取
        char* result = list_data_to_string(data);
        printf("%s\n", result);
        string_free(result);

        // 索引访问
        size_t idx[] = {0};
        result = list_data_get(data, idx, 1);
        printf("[0]: %s\n", result);
        string_free(result);

        // 修改
        list_data_append(data, 5, "6");
        list_data_insert(data, 0, 0, "0");
        list_data_delete(data, 0);
        list_data_replace(data, 0, "[9,8,7]");

        // 命令执行
        result = list_data_execute_command(data, ".+[0]=100");
        printf("Result: %s\n", result);
        string_free(result);

        // 清理
        list_data_free(data);
        return 0;
}
```

#### 编译
```bash
gcc -o example example.c -Ltarget/release -ldotlist -Ibindings/c
```

---

### Go (CGO)

#### 使用示例
```go
package main

import (
        "fmt"
        "dotlist"
)

func main() {
        // 创建数据
        data, _ := dotlist.NewListDataFromString("[1,2,3,4,5];")
        defer data.Free()

        // 读取
        fmt.Println(data.ToString())

        // 索引访问
        result, _ := data.Get([]uint{0})
        fmt.Println("[0]:", result)

        // 修改
        data.Append(0, "6")
        data.Insert(0, 0, "0")
        data.Delete(0)
        data.Replace(0, "[9,8,7]")

        // 命令执行
        result, _ = data.ExecuteCommand(".+[0]=100")
        fmt.Println("Result:", result)
}
```

#### 编译
```bash
cd bindings/go
go build -o ../example.exe example_test.go
```

---

## 📁 项目结构

```
e:\.list\
├── Cargo.toml                 # Rust项目配置
├── src/
│   ├── main.rs               # CLI/IDE入口
│   ├── lib.rs                # 库入口 (FFI)
│   ├── parser.rs             # 高性能解析器核心
│   ├── cli.rs                # 命令行界面
│   ├── ide.rs                # IDE界面+语法高亮
│   ├── python_bindings.rs    # Python绑定
│   ├── nodejs_bindings.rs    # Node.js绑定
│   └── c_ffi.rs              # C/C++ FFI绑定
├── bindings/
│   ├── python/
│   │   └── example.py        # Python示例
│   ├── nodejs/
│   │   └── example.js        # Node.js示例
│   ├── c/
│   │   └── example.c         # C/C++示例
│   └── go/
│       ├── dotlist.go      # Go绑定
│       └── example_test.go   # Go示例
├── build_all.bat             # 一键编译所有语言
└── example.list              # 示例.list文件
```

---

## 🎯 性能优化

### Rust核心优化
- ✅ **LTO链接时优化** (`lto = true`)
- ✅ **单个代码生成单元** (`codegen-units = 1`)
- ✅ **Panic中止模式** (`panic = "abort"`)
- ✅ **Release最高优化** (`opt-level = 3`)

### 解析器性能特性
- ✅ 零拷贝字符串处理
- ✅ 预分配Vec容量
- ✅ 正则表达式预编译
- ✅ 原地内存修改
- ✅ 无堆分配的栈操作
- ✅ 二进制压缩存储（flate2 + bincode）

---

## 📊 性能对比

| 语言 | 解析速度 | 内存占用 | 启动时间 |
|------|---------|---------|---------|
| **Rust** | ⚡⚡⚡ 极快 | 低 | 即时 |
| **C/C++** | ⚡⚡⚡ 极快 | 最低 | 即时 |
| **Go** | ⚡⚡ 快 | 中 | 快 |
| **Node.js** | ⚡ 中等 | 中 | 中 |
| **Python** | 🐢 慢 | 高 | 慢 |

---

## 🔧 API参考

### 核心方法

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `to_string()` | - | String | 获取完整数据字符串 |
| `get(indices)` | `[]usize` | String | 按索引读取 |
| `append(index, value)` | `usize, String` | String | 追加元素 |
| `insert(index, pos, value)` | `3个参数` | String | 插入元素 |
| `delete(index)` | `usize` | String | 删除元素 |
| `replace(index, value)` | `usize, String` | String | 替换元素 |
| `execute_command(cmd)` | `String` | String | 执行命令 |

---

## 🛠️ 构建命令

```bash
# 构建所有语言绑定
cargo build --release --features python,binary

# 仅构建Rust核心
cargo build --release

# 运行测试
cargo test
```

---

## 🤝 贡献指南

欢迎提交Issue和Pull Request！

### 开发环境要求
- Rust 1.70+
- Python 3.8+ (可选，用于Python绑定)
- Node.js 18+ (可选，用于Node.js绑定)
- GCC/MinGW (可选，用于C/C++示例)
- Go 1.20+ (可选，用于Go绑定)

---

## 📄 许可证

MIT License

---

**享受高性能的.list语法处理体验！🚀**
