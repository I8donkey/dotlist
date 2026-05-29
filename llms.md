# .list 项目文档

## 项目概述

.list 是一个用 Rust 编写的高性能多语言语法处理工具，支持 Python、Node.js、C/C++、Go 和 Rust 等多种语言绑定。

## 核心特性

- **超高性能**：Rust 编写，LTO 优化
- **多语言支持**：Python、Node.js、C/C++、Go、Rust
- **零拷贝解析**：内存高效
- **CLI/IDE 双模式**：终端 UI + 命令行交互
- **二进制压缩**：支持压缩存储

## 快速开始

```bash
# 编译核心库
cargo build --release --features python,binary

# CLI 使用
list open example.list
```

## 数据格式

```
[apple,banana,cherry]; [red,green,blue]; key:value;
```

## 多语言绑定

### Python
```python
from dotlist import PyListData
data = PyListData("[apple,banana];")
print(data.get([0]))
```

### Node.js
```javascript
const { JsListData } = require('dotlist');
const data = new JsListData('[apple,banana];');
```

### C/C++
```c
CListData* data = list_data_from_string("[apple,banana];");
```

### Go
```go
data, _ := dotlist.NewListDataFromString("[apple,banana];")
```

## 项目结构

```
├── src/
│   ├── main.rs          # CLI/IDE 入口
│   ├── parser.rs        # 解析器核心
│   ├── c_ffi.rs         # C FFI
│   ├── python_bindings.rs
│   └── nodejs_bindings.rs
├── bindings/
│   ├── python/
│   ├── nodejs/
│   ├── c/
│   └── go/
└── Cargo.toml
```

## 许可证

MIT License
