![Static Badge](https://img.shields.io/badge/_LICENCE_-_LGPL--2.0_-orange)  ![Auther](https://img.shields.io/badge/%E4%BD%9C%E8%80%85-i8donkey-blue)
### 如果你需要中文版本，请阅读 [中文版本](README.md)

# .list High-Performance Multi-Language Syntax Processing Tool

## ✨ Features

- ⚡ **Ultra High Performance**: Written in Rust
- 🌍 **Multi-Language Support**: Python, Node.js, C/C++, Go, Rust
- 💾 **Zero-Copy Parsing**: Memory efficient, extremely fast
- 🎨 **IDE Mode**: Terminal UI + Real-time syntax highlighting
- 🔧 **CLI Mode**: Command-line interactive interface
- 📦 **Cross-Platform**: Windows/Linux/macOS
- 📁 **Binary Compression**: Support compressed storage, save space

---

## 🚀 Quick Start

### 1. Build Core Library (Rust)

```bash
cargo build --release --features python,binary
```

### 2. CLI Usage

```bash
# Open file and interact
list open example.list

# Create new file (text mode)
list new mydata.list

# Create new file (binary compressed mode)
list new mydata.list -b
```

---

## 📝 Command Syntax

### Read Operations

| Command | Description | Example |
|---------|-------------|---------|
| `[n]` | Read nth element | `[0]` |
| `[n][m]` | Read mth child element of nth element | `[2][1]` |
| `[n]/` | Read key of nth element | `[0]/` |
| `[n]<` | Read value of nth element | `[0]<` |

### Modify Operations

| Command | Description | Example |
|---------|-------------|---------|
| `.+[n]` | Create new array at position n | `.+[3]` |
| `.+[n]=value` | Append value to array at position n | `.+[3]=10` |
| `.+[n]>[m]=value` | Insert value at position m of array n | `.+[3]>[1]=10` |
| `.+>[n]=[...]` | Replace array n with new content | `.+>[3]=[1,2,3]` |
| `.- >[n]` | Delete nth element | `.- >[3]` |

### Move/Insert Operations

| Command | Description | Example |
|---------|-------------|---------|
| `.>[src]>[dest]` | Move [src] to end of [dest] | `.>[0]>[1]` |
| `.>[src]>[dest]<[pos]` | Insert [src] before [dest][pos] | `.>[0]>[1]<[2]` |
| `.>[src]>[dest]>[pos]` | Insert [src] after [dest][pos] | `.>[0]>[1]>[2]` |

### Search Operations

| Command | Description | Example |
|---------|-------------|---------|
| `.?keyword` | Search keyword at root level | `.?apple` |
| `.?keyword>[n]` | Search keyword in [n] | `.?apple>[0]` |
| `.?keyword>[n]-[m]` | Search keyword from [n] to [m] | `.?apple>[0]-[3]` |

### Print Operations

| Command | Description | Example |
|---------|-------------|---------|
| `.print>[n]` | Print content of [n] | `.print>[0]` |
| `.print>keyword` | Print keyword | `.print>hello` |
| `.print>$var` | Print variable | `.print>$myvar` |
| `.print>[n]-[m]` | Print content from [n] to [m] | `.print>[0]-[2]` |

### Variable Operations

| Command | Description | Example |
|---------|-------------|---------|
| `$var=value` | Set variable | `$name=John` |
| `$var` | Read variable | `$name` |
| `$vars` | Show all variables | `$vars` |

### Language Settings

| Command | Description | Example |
|---------|-------------|---------|
| `.setlang=lang` | Set language | `.setlang=en` |
| `.sl=lang` | Set language (short) | `.sl=zh` |

### Other Commands

| Command | Description | Example |
|---------|-------------|---------|
| `.h` or `.help` | Show help | `.h` |
| `exit()` | Exit program | `exit()` |

---

## 📁 Data File Format

### Basic Syntax

```
# Array
[element1, element2, element3]

# Key-Value pair
key:value

# Nested structure
[key1:value1, key2:value2, [subarray]]

# Multi-line data (separated by semicolon)
[apple,banana,cherry]; [red,green,blue]; hello world;
```

### Example Data File (example.list)

```
[apple,banana,cherry]
[red,green,blue]
[key1:value1, key2:value2]
```

---

## 🌍 Multi-Language Bindings

### Python (PyO3)

#### Installation
```bash
pip install maturin
maturin develop --release --features python,binary
```

#### Usage Example
```python
from dotlist import PyListData

# Create data
data = PyListData("[apple,banana,cherry]; [red,green,blue];")

# Operations
print(data.get([0]))           # [apple,banana,cherry]
print(data.to_string())        # Full data
data.append(0, "new_item")     # Append
data.insert(0, 1, "item")      # Insert
data.delete(0)                  # Delete
data.replace(0, "[a,b,c]")     # Replace
result = data.execute_command("[0]")  # Execute command
```

---

### Node.js (napi-rs)

#### Installation
```bash
npm install -g @napi-rs/cli
napi build --release --features nodejs
```

#### Usage Example
```javascript
const { JsListData } = require('dotlist');

// Create data
const data = new JsListData('[apple,banana,cherry]; [red,green,blue];');

// Async operations
async function main() {
        console.log(data.toString());
        console.log(data.get([0]));           // Read
        await data.append(0n, 'value');       // Append
        await data.executeCommand('[0]');     // Execute command
}
main();
```

---

### C/C++ (FFI)

#### Header Generation
```bash
cbindgen --crate dotlist --output bindings/c/dotlist.h
```

#### Usage Example
```c
#include <stdio.h>
#include "dotlist.h"

int main() {
        // Create data
        const char* content = "[1,2,3,4,5];";
        CListData* data = list_data_from_string(content);

        // Read
        char* result = list_data_to_string(data);
        printf("%s\n", result);
        string_free(result);

        // Index access
        size_t idx[] = {0};
        result = list_data_get(data, idx, 1);
        printf("[0]: %s\n", result);
        string_free(result);

        // Modify
        list_data_append(data, 5, "6");
        list_data_insert(data, 0, 0, "0");
        list_data_delete(data, 0);
        list_data_replace(data, 0, "[9,8,7]");

        // Execute command
        result = list_data_execute_command(data, ".+[0]=100");
        printf("Result: %s\n", result);
        string_free(result);

        // Cleanup
        list_data_free(data);
        return 0;
}
```

#### Compilation
```bash
gcc -o example example.c -Ltarget/release -ldotlist -Ibindings/c
```

---

### Go (CGO)

#### Usage Example
```go
package main

import (
        "fmt"
        "dotlist"
)

func main() {
        // Create data
        data, _ := dotlist.NewListDataFromString("[1,2,3,4,5];")
        defer data.Free()

        // Read
        fmt.Println(data.ToString())

        // Index access
        result, _ := data.Get([]uint{0})
        fmt.Println("[0]:", result)

        // Modify
        data.Append(0, "6")
        data.Insert(0, 0, "0")
        data.Delete(0)
        data.Replace(0, "[9,8,7]")

        // Execute command
        result, _ = data.ExecuteCommand(".+[0]=100")
        fmt.Println("Result:", result)
}
```

#### Compilation
```bash
cd bindings/go
go build -o ../example.exe example_test.go
```

---

## 📁 Project Structure

```
e:\.list\
├── Cargo.toml                 # Rust project configuration
├── src/
│   ├── main.rs               # CLI/IDE entry
│   ├── lib.rs                # Library entry (FFI)
│   ├── parser.rs             # High-performance parser core
│   ├── cli.rs                # Command-line interface
│   ├── ide.rs                # IDE interface + syntax highlighting
│   ├── python_bindings.rs    # Python bindings
│   ├── nodejs_bindings.rs    # Node.js bindings
│   └── c_ffi.rs              # C/C++ FFI bindings
├── bindings/
│   ├── python/
│   │   └── example.py        # Python example
│   ├── nodejs/
│   │   └── example.js        # Node.js example
│   ├── c/
│   │   └── example.c         # C/C++ example
│   └── go/
│       ├── dotlist.go      # Go bindings
│       └── example_test.go   # Go example
├── build_all.bat             # One-click build all languages
└── example.list              # Example .list file
```

---

## 🎯 Performance Optimization

### Rust Core Optimizations
- ✅ **LTO Link-time Optimization** (`lto = true`)
- ✅ **Single code generation unit** (`codegen-units = 1`)
- ✅ **Panic abort mode** (`panic = "abort"`)
- ✅ **Release maximum optimization** (`opt-level = 3`)

### Parser Performance Features
- ✅ Zero-copy string handling
- ✅ Pre-allocated Vec capacity
- ✅ Pre-compiled regex patterns
- ✅ In-place memory modification
- ✅ Stack operations without heap allocation
- ✅ Binary compression storage (flate2 + bincode)

---

## 📊 Performance Comparison

| Language | Parsing Speed | Memory Usage | Startup Time |
|----------|--------------|--------------|--------------|
| **Rust** | ⚡⚡⚡ Extremely Fast | Low | Instant |
| **C/C++** | ⚡⚡⚡ Extremely Fast | Lowest | Instant |
| **Go** | ⚡⚡ Fast | Medium | Fast |
| **Node.js** | ⚡ Medium | Medium | Medium |
| **Python** | 🐢 Slow | High | Slow |

---

## 🔧 API Reference

### Core Methods

| Method | Parameters | Return | Description |
|--------|------------|--------|-------------|
| `to_string()` | - | String | Get full data string |
| `get(indices)` | `[]usize` | String | Read by indices |
| `append(index, value)` | `usize, String` | String | Append element |
| `insert(index, pos, value)` | `3 params` | String | Insert element |
| `delete(index)` | `usize` | String | Delete element |
| `replace(index, value)` | `usize, String` | String | Replace element |
| `execute_command(cmd)` | `String` | String | Execute command |

---

## 🛠️ Build Commands

```bash
# Build all language bindings
cargo build --release --features python,binary

# Build Rust core only
cargo build --release

# Run tests
cargo test
```

---

## 🤝 Contributing

Welcome to submit Issues and Pull Requests!

### Development Requirements
- Rust 1.70+
- Python 3.8+ (optional, for Python bindings)
- Node.js 18+ (optional, for Node.js bindings)
- GCC/MinGW (optional, for C/C++ examples)
- Go 1.20+ (optional, for Go bindings)

---

## 📄 License

MIT License

---

**Enjoy high-performance .list syntax processing! 🚀**
