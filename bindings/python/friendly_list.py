# -*- coding: utf-8 -*-
"""
.list Python 友好接口 - 支持原生列表语法
============================================

使用示例:
    from list_lang import List
    
    l = List("[apple,banana,cherry]; [1,2,3,4,5];")
    
    # 索引访问 (像 Python list 一样)
    print(l[0])              # [apple,banana,cherry]
    print(l[0][0])           # apple
    print(l[1][2])           # 3
    
    # 切片操作
    print(l[0][0:2])         # [apple, banana]
    print(l[1][1:4])         # [2, 3, 4]
    
    # 迭代
    for item in l[0]:
        print(item)          # apple, banana, cherry
    
    # 赋值
    l[1] = "new_value"
    
    # 查找
    indices = l.find("apple")
"""

import sys
import importlib

# 导入底层的 PyListData (Rust 编译的模块)
PyListData = None
try:
        _list_lang = importlib.import_module('list_lang')
        PyListData = _list_lang.PyListData
except (ImportError, AttributeError) as e:
        print(f"⚠️ 无法加载底层 list_lang 模块: {e}")
        print("   请确保已运行: maturin develop --release --features python,binary")


class ListItem:
        """
        .list 元素包装器 - 支持索引、切片、迭代
        
        可以是:
        - String: "hello"
        - Array: [item1, item2, ...]
        - KeyValue: key:value
        """
        
        def __init__(self, data: PyListData, indices: list):
                self._data = data
                self._indices = indices
                self._value = None
                
        def _get_value(self) -> str:
                if self._value is None:
                        self._value = self._data.get(self._indices)
                return self._value
        
        @property
        def value(self) -> str:
                return self._get_value()
        
        def __repr__(self) -> str:
                return f"ListItem({self._get_value()})"
        
        def __str__(self) -> str:
                return self._get_value()
        
        def __eq__(self, other) -> bool:
                if isinstance(other, ListItem):
                        return self._get_value() == other._get_value()
                return self._get_value() == str(other)
        
        def __contains__(self, item) -> bool:
                return item in self._get_value()
        
        def __len__(self) -> int:
                try:
                        arr = self._data.get_array(self._indices[0] if len(self._indices) == 1 else self._indices[-1])
                        if arr is not None:
                                return len(arr)
                except:
                        pass
                return len(self._get_value())
        
        def __getitem__(self, key):
                """
                支持多种索引方式:
                
                l[0]       - 第一个元素
                l[0][1]    - 嵌套索引
                l[0][1:3]  - 切片
                l[0][-1]   - 负数索引
                """
                
                # 处理切片
                if isinstance(key, slice):
                        start = 0 if key.start is None else key.start
                        stop = 0 if key.stop is None else key.stop
                        
                        # 处理负数索引
                        if start < 0:
                                length = len(self)
                                start = max(0, length + start)
                        if stop < 0:
                                length = len(self)
                                stop = max(0, length + stop)
                        
                        try:
                                result = self._data.get_slice(
                                        self._indices[-1],
                                        start,
                                        stop
                                )
                                return result or []
                        except Exception as e:
                                raise IndexError(f"切片失败: {e}")
                
                # 处理整数索引
                elif isinstance(key, int):
                        new_indices = self._indices.copy()
                        
                        # 处理负数索引
                        if key < 0:
                                try:
                                        arr_len = len(self)
                                        key = arr_len + key
                                except:
                                        raise IndexError("负数索引超出范围")
                        
                        new_indices.append(key)
                        return ListItem(self._data, new_indices)
                
                else:
                        raise TypeError(f"索引必须是 int 或 slice，不是 {type(key)}")
        
        def __setitem__(self, key, value):
                """支持赋值操作"""
                if isinstance(key, int):
                        indices = self._indices.copy()
                        if key < 0:
                                try:
                                        arr_len = len(self)
                                        key = arr_len + key
                                except:
                                        raise IndexError("负数索引超出范围")
                        indices.append(key)
                        cmd = f".+{format_indices(indices)}={value}"
                        self._data.execute_command(cmd)
                        self._value = None  # 清除缓存
                else:
                        raise TypeError(f"索引必须是 int，不是 {type(key)}")
        
        def __iter__(self):
                """支持迭代"""
                try:
                        arr = self._data.get_array(self._indices[-1] if self._indices else 0)
                        if arr is not None:
                                for i, item in enumerate(arr):
                                        yield ListItem(self._data, [*self._indices, i])
                                return
                except:
                        pass
                
                # 如果不是数组，迭代字符串的每个字符
                for char in self._get_value():
                        yield char
        
        def __bool__(self) -> bool:
                """布尔值判断"""
                val = self._get_value()
                return bool(val and val not in ('[]', '', 'None'))
        
        def split(self, sep=None):
                """分割值（类似 str.split）"""
                val = self._get_value()
                if sep is None:
                        return val.split()
                return val.split(sep)
        
        def startswith(self, prefix: str) -> bool:
                return self._get_value().startswith(prefix)
        
        def endswith(self, suffix: str) -> bool:
                return self._get_value().endswith(suffix)


class List:
        """
        .list 数据结构 - Python 友好接口
        
        完全兼容 Python 原生列表语法:
        - l[0], l[1], l[-1]
        - l[0][1:3] (切片)
        - for item in l[0]: (迭代)
        - l[0] = "value" (赋值)
        - len(l), bool(l)
        
        示例:
            l = List("[a,b,c]; [1,2,3];")
            
            print(l[0])          # [a,b,c]
            print(l[0][0])       # a
            print(l[0][0:2])     # [a, b]
            print(len(l))        # 2
            
            for item in l[0]:
                print(item)      # a, b, c
            
            l[1] = "[x,y,z]"
            print(l.to_string()) # 更新后的数据
        """
        
        def __init__(self, content: str = ""):
                if content:
                        self._data = PyListData(content)
                else:
                        self._data = PyListData(None)
        
        @classmethod
        def from_file(cls, path: str) -> 'List':
                """从文件加载"""
                with open(path, 'r', encoding='utf-8') as f:
                        return cls(f.read())
        
        @classmethod
        def load_binary(cls, path: str) -> 'List':
                """从二进制文件加载"""
                instance = cls.__new__(cls)
                instance._data = PyListData.load_binary(path)
                return instance
        
        def to_string(self) -> str:
                return self._data.to_string()
        
        def save(self, path: str):
                """保存为文本文件"""
                with open(path, 'w', encoding='utf-8') as f:
                        f.write(self.to_string())
        
        def save_binary(self, path: str):
                """保存为二进制文件"""
                self._data.save_binary(path)
        
        def export_text(self, path: str):
                """导出为文本文件"""
                self._data.export_text(path)
        
        def __repr__(self) -> str:
                return f"List({self._data.to_string()})"
        
        def __str__(self) -> str:
                return self._data.to_string()
        
        def __len__(self) -> int:
                return self._data.len()
        
        def __bool__(self) -> bool:
                return not self._data.is_empty()
        
        def __iter__(self):
                """顶层迭代 - 遍历所有元素"""
                for i in range(len(self)):
                        yield self[i]
        
        def __getitem__(self, key):
                """
                支持多种索引:
                
                l[0]       - 第一个元素 (返回 ListItem)
                l[0][1]    - 嵌套访问
                l[0][1:3]  - 切片
                l[-1]      - 最后一个元素
                l[:]       - 所有元素
                """
                
                # 处理切片 (顶层)
                if isinstance(key, slice):
                        start = 0 if key.start is None else key.start
                        stop = len(self) if key.stop is None else key.stop
                        step = 1 if key.step is None else key.step
                        
                        # 处理负数
                        if start < 0:
                                start = max(0, len(self) + start)
                        if stop < 0:
                                stop = max(0, len(self) + stop)
                        
                        result = []
                        for i in range(start, stop, step):
                                result.append(ListItem(self._data, [i]))
                        return result
                
                # 整数索引
                elif isinstance(key, int):
                        if key < 0:
                                key = len(self) + key
                        if key < 0 or key >= len(self):
                                raise IndexError(f"索引 {key} 超出范围 (长度 {len(self)})")
                        return ListItem(self._data, [key])
                
                else:
                        raise TypeError(f"索引必须是 int 或 slice，不是 {type(key)}")
        
        def __setitem__(self, key, value):
                """顶层赋值"""
                if isinstance(key, int):
                        if key < 0:
                                key = len(self) + key
                        cmd = f".+>[{key}]={value}"
                        self._data.execute_command(cmd)
                else:
                        raise TypeError(f"索引必须是 int")
        
        def run(self, command: str) -> str:
                """执行 .list 命令"""
                return self._data.execute_command(command)
        
        def find(self, pattern: str) -> list:
                """查找包含 pattern 的元素索引"""
                return self._data.find(pattern)
        
        def find_in(self, index: int, pattern: str) -> list:
                """在指定数组中查找"""
                return self._data.find_in_array(index, pattern)
        
        def append(self, index: int, value: str):
                """追加元素到数组"""
                self._data.append(index, value)
        
        def insert(self, index: int, position: int, value: str):
                """插入元素到数组"""
                self._data.insert(index, position, value)
        
        def delete(self, index: int):
                """删除元素"""
                self._data.delete(index)
        
        def replace(self, index: int, value: str):
                """替换元素"""
                self._data.replace(index, value)
        
        def get_raw(self, indices: list) -> str:
                """获取原始值 (不包装为 ListItem)"""
                return self._data.get(indices)
        
        @property
        def items(self) -> list:
                """获取所有元素作为列表"""
                return [self[i] for i in range(len(self))]


def format_indices(indices: list) -> str:
        """格式化索引为 .list 格式: [0][1][2]"""
        return ''.join(f'[{i}]' for i in indices)


# 导出便捷函数
def open_list(path: str) -> List:
        """打开 .list 文件"""
        return List.from_file(path)

def new_list(content: str = "") -> List:
        """创建新的 List"""
        return List(content)

# 兼容旧接口
__all__ = ['List', 'ListItem', 'PyListData', 'open_list', 'new_list']
