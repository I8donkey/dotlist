# .list 完整项目文档

## 项目概述

.list 是一个用 Rust 编写的高性能多语言语法处理工具，支持 Python、Node.js、C/C++、Go 和 Rust 等多种语言绑定。

## 核心特性

- **超高性能**：Rust 编写，LTO 优化，codegen-units=1，opt-level=3
- **多语言支持**：Python (PyO3)、Node.js (napi-rs)、C/C++ (FFI)、Go (CGO)、Rust
- **零拷贝解析**：内存高效，预分配 Vec 容量
- **CLI/IDE 双模式**：终端 UI + 命令行交互，支持语法高亮
- **二进制压缩**：支持 flate2 + bincode 压缩存储
- **变量系统**：支持变量赋值和引用
- **命令管道**：支持命令管道操作

## 数据格式

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

### 数据结构 (ListValue)

```rust
pub enum ListValue {
    String(String),
    Array(Vec<ListValue>),
    KeyValue(String, Box<ListValue>),
}

pub struct ListData {
    pub items: Vec<ListValue>,
    pub variables: HashMap<String, ListValue>,
    pub language: String,
}
```

## 命令语法

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

## 多语言绑定

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

#### 操作符支持
```python
# 字符串表示
str(data)

# 索引访问
data[0]
data[0][1]

# 合并（+=）
data += data2

# 去除（-=）
data -= data2

# 加法（+）
result = data + data2

# 减法（-）
result = data - data2
```

### Node.js (napi-rs)

#### 安装
```bash
npm install -g @napi-rs/cli
napi build --release --features nodejs
```

#### 使用示例
```javascript
const { JsListData } = require('dotlist');

const data = new JsListData('[apple,banana,cherry]; [red,green,blue];');

async function main() {
    console.log(data.toString());
    console.log(data.get([0]));
    await data.append(0n, 'value');
    await data.executeCommand('[0]');
}
main();
```

### C/C++ (FFI)

#### 头文件
- `bindings/c/list_lang_ffi.h` - C 语言头文件
- `bindings/cpp/list_lang_ffi.hpp` - C++ 头文件

#### 使用示例
```c
#include <stdio.h>
#include "list_lang_ffi.h"

int main() {
    CListData* data = list_data_from_string("[1,2,3,4,5];");
    char* result = list_data_to_string(data);
    printf("%s\n", result);
    string_free(result);
    list_data_free(data);
    return 0;
}
```

#### C++ 示例
```cpp
#include <iostream>
#include "list_lang_ffi.h"

int main() {
    CListData* data = list_data_from_string("[apple,banana]; [1,2,3];");
    std::cout << list_data_to_string(data) << std::endl;
    list_data_free(data);
    return 0;
}
```

#### 编译（MSVC）
```cmd
call "D:\vis_installer\VC\Auxiliary\Build\vcvars64.bat"
cl.exe /EHsc /utf-8 /std:c++17 test.cpp /link /LIBPATH:target\release dotlist.lib ws2_32.lib user32.lib kernel32.lib advapi32.lib ntdll.lib userenv.lib secur32.lib /OUT:test.exe
```

### Go (CGO)

#### 使用示例
```go
package main

import (
    "fmt"
    "dotlist"
)

func main() {
    data, _ := dotlist.NewListDataFromString("[1,2,3,4,5];")
    defer data.Free()

    fmt.Println(data.ToString())
    result, _ := data.Get([]uint{0})
    fmt.Println("[0]:", result)
}
```

### Rust

#### 使用示例
```rust
use dotlist::ListData;

fn main() {
    let mut data = ListData::from_string("[apple,banana]; [1,2,3];").unwrap();
    println!("{}", data.to_string());
    data.append(0, ListValue::String("cherry".to_string()));
}
```

## 项目结构

```
├── Cargo.toml                 # Rust 项目配置
├── src/
│   ├── main.rs               # CLI/IDE 入口
│   ├── lib.rs                # 库入口 (FFI)
│   ├── parser.rs             # 高性能解析器核心
│   ├── cli.rs                # 命令行界面
│   ├── ide.rs                # IDE 界面 + 语法高亮
│   ├── python_bindings.rs    # Python 绑定
│   ├── nodejs_bindings.rs    # Node.js 绑定
│   └── c_ffi.rs              # C/C++ FFI 绑定
├── bindings/
│   ├── python/
│   ├── nodejs/
│   ├── c/
│   └── go/
├── tests/
│   └── test_ffi.cpp          # C++ FFI 测试
└── example.list              # 示例 .list 文件
```

## API 参考

### ListData 核心方法

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `new()` | - | ListData | 创建空数据 |
| `from_string(content)` | &str | Result<Self, String> | 从字符串解析 |
| `to_string()` | - | String | 转换为字符串 |
| `len()` | - | usize | 获取长度 |
| `is_empty()` | - | bool | 是否为空 |
| `get(indices)` | &[usize] | Option<&ListValue> | 按索引读取 |
| `get_array(index)` | usize | Option<Vec<String>> | 获取数组 |
| `get_slice(index, start, end)` | usize, usize, usize | Option<Vec<String>> | 获取切片 |
| `find(pattern)` | &str | Vec<usize> | 查找匹配索引 |
| `find_in_array(index, pattern)` | usize, &str | Option<Vec<usize>> | 在数组中查找 |
| `append(index, value)` | usize, ListValue | Result<(), String> | 追加元素 |
| `insert(index, pos, value)` | usize, usize, ListValue | Result<(), String> | 插入元素 |
| `delete(index)` | usize | Result<(), String> | 删除元素 |
| `replace(index, value)` | usize, ListValue | Result<(), String> | 替换元素 |
| `execute_command(cmd)` | &str | Result<String, String> | 执行命令 |

### C FFI 函数

| 函数 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `list_data_new()` | - | *mut CListData | 创建新数据 |
| `list_data_from_string(content)` | *const c_char | *mut CListData | 从字符串创建 |
| `list_data_free(ptr)` | *mut CListData | - | 释放数据 |
| `list_data_to_string(ptr)` | *const CListData | *mut c_char | 转换为字符串 |
| `list_data_len(ptr)` | *const CListData | usize | 获取长度 |
| `list_data_is_empty(ptr)` | *const CListData | bool | 是否为空 |
| `list_data_get_array(ptr, index, out_len)` | *const CListData, usize, *mut usize | *mut *mut c_char | 获取数组 |
| `list_data_get_slice(ptr, index, start, end, out_len)` | *const CListData, usize, usize, usize, *mut usize | *mut *mut c_char | 获取切片 |
| `list_data_find(ptr, pattern, out_len)` | *const CListData, *const c_char, *mut usize | *mut usize | 查找匹配 |
| `list_data_find_in_array(ptr, index, pattern, out_len)` | *const CListData, usize, *const c_char, *mut usize | *mut usize | 在数组中查找 |
| `list_data_save_binary(ptr, path)` | *const CListData, *const c_char | bool | 保存二进制 |
| `list_data_load_binary(path)` | *const c_char | *mut CListData | 加载二进制 |
| `string_free(ptr)` | *mut c_char | - | 释放字符串 |
| `string_array_free(ptr, len)` | *mut *mut c_char, usize | - | 释放字符串数组 |
| `usize_array_free(ptr)` | *mut usize | - | 释放 usize 数组 |

## 性能优化

### Rust 核心优化
- LTO 链接时优化 (`lto = true`)
- 单个代码生成单元 (`codegen-units = 1`)
- Panic 中止模式 (`panic = "abort"`)
- Release 最高优化 (`opt-level = 3`)

### 解析器性能特性
- 零拷贝字符串处理
- 预分配 Vec 容量
- 正则表达式预编译
- 原地内存修改
- 无堆分配的栈操作
- 二进制压缩存储（flate2 + bincode）

## 构建命令

```bash
# 构建所有语言绑定
cargo build --release --features python,binary

# 仅构建 Rust 核心
cargo build --release

# 运行测试
cargo test

# 生成 C 头文件
cargo build

# 生成 C++ 头文件
cargo build
```

## 许可证

MIT License
