//! .list Rust 原生接口
//! ===================
//!
//! 使用示例:
//! ```rust
//! use list_lang::{List, ListItem};
//!
//! fn main() {
//!     let l = List::new("[apple,banana,cherry]; [1,2,3,4,5];");
//!
//!     // 索引访问 (Index trait)
//!     println!("{}", l[0]);              // [apple,banana,cherry]
//!     println!("{}", l[0][1]);           // banana
//!     println!("{}", l.at(&[2, 3]));      // 4
//!
//!     // 切片
//!     let slice = l.slice(1, 0, 2);       // ["apple", "banana"]
//!     println!("{:?}", slice);
//!
//!     // 迭代 (IntoIterator)
//!     for item in &l[1] {
//!         println!("{}", item);          // 1, 2, 3, 4, 5
//!     }
//!
//!     // 查找
//!     let indices = l.find("banana");    // vec![1]
//!     println!("{:?}", indices);
//! }
//! ```

use crate::parser::{ListData, ListValue};
use std::ops::{Index, IndexMut};
use std::fmt;
use std::path::Path;

/// .list 元素包装器 - 支持索引、切片、迭代
#[derive(Clone)]
pub struct ListItem {
        data: ListData,
        indices: Vec<usize>,
}

impl ListItem {
        pub fn new(data: ListData, indices: Vec<usize>) -> Self {
                ListItem { data, indices }
        }

        /// 获取字符串值
        pub fn value(&self) -> String {
                match self.data.get(&self.indices) {
                        Some(val) => value_to_string(val),
                        None => String::new(),
                }
        }

        /// 获取数组元素
        pub fn array(&self) -> Vec<String> {
                if let Some(idx) = self.indices.last() {
                        self.data.get_array(*idx).unwrap_or_default()
                } else {
                        Vec::new()
                }
        }

        /// 切片操作
        pub fn slice(&self, start: usize, end: usize) -> Vec<String> {
                if let Some(idx) = self.indices.last() {
                        self.data.get_slice(*idx, start, end).unwrap_or_default()
                } else {
                        Vec::new()
                }
        }

        /// 获取长度
        pub fn len(&self) -> usize {
                self.array().len()
        }

        pub fn is_empty(&self) -> bool {
                self.len() == 0
        }
}

impl fmt::Display for ListItem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.value())
        }
}

impl fmt::Debug for ListItem {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "ListItem({})", self.value())
        }
}

impl Index<usize> for ListItem {
        type Output = Self;

        fn index(&self, index: usize) -> &Self {
                panic!("ListItem indexing returns owned value, use .get(index) instead")
        }
}

impl ListItem {
        /// 嵌套索引访问 (返回新的 ListItem)
        pub fn get(&self, index: usize) -> Self {
                let mut new_indices = self.indices.clone();
                new_indices.push(index);
                ListItem {
                        data: self.data.clone(),
                        indices: new_indices,
                }
        }
}

/// 迭代器实现
pub struct ListItemIter {
        items: Vec<String>,
        position: usize,
}

impl Iterator for ListItemIter {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
                if self.position < self.items.len() {
                        let item = self.items[self.position].clone();
                        self.position += 1;
                        Some(item)
                } else {
                        None
                }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
                let remaining = self.items.len() - self.position;
                (remaining, Some(remaining))
        }
}

impl IntoIterator for ListItem {
        type Item = String;
        type IntoIter = ListItemIter;

        fn into_iter(self) -> Self::IntoIter {
                ListItemIter {
                        items: self.array(),
                        position: 0,
                }
        }
}

impl<'a> IntoIterator for &'a ListItem {
        type Item = String;
        type IntoIter = ListItemIter;

        fn into_iter(self) -> Self::IntoIter {
                ListItemIter {
                        items: self.array(),
                        position: 0,
                }
        }
}

/// .list 数据结构 - Rust 原生接口
pub struct List {
        data: ListData,
}

impl List {
        /// 创建新的 List
        pub fn new(content: &str) -> Self {
                List {
                        data: ListData::from_string(content).unwrap_or_else(|e| {
                                eprintln!("解析错误: {}", e);
                                ListData::new()
                        }),
                }
        }

        /// 创建空 List
        pub fn empty() -> Self {
                List { data: ListData::new() }
        }

        /// 从文件加载
        pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
                use std::fs;
                let content = fs::read_to_string(path)
                        .map_err(|e| format!("读取文件失败: {}", e))?;
                Ok(Self::new(&content))
        }

        /// 从二进制文件加载
        #[cfg(feature = "binary")]
        pub fn load_binary<P: AsRef<Path>>(path: P) -> Result<Self, String> {
                ListData::load_binary(path.as_ref().to_str().unwrap_or(""))
                        .map(|data| List { data })
        }

        /// 字符串表示
        pub fn to_string(&self) -> String {
                self.data.to_string()
        }

        /// 长度
        pub fn len(&self) -> usize {
                self.data.len()
        }

        pub fn is_empty(&self) -> bool {
                self.data.is_empty()
        }

        /// 索引访问，返回 ListItem
        pub fn get(&self, index: usize) -> ListItem {
                ListItem {
                        data: self.data.clone(),
                        indices: vec![index],
                }
        }

        /// 多级索引访问
        pub fn at(&self, indices: &[usize]) -> ListItem {
                ListItem {
                        data: self.data.clone(),
                        indices: indices.to_vec(),
                }
        }

        /// 切片
        pub fn slice(&self, start: usize, end: usize) -> Vec<ListItem> {
                let mut result = Vec::new();
                for i in start..end.min(self.len()) {
                        result.push(self.get(i));
                }
                result
        }

        /// 执行命令
        pub fn run(&mut self, command: &str) -> Result<String, String> {
                self.data.execute_command(command)
        }

        /// 查找元素
        pub fn find(&self, pattern: &str) -> Vec<usize> {
                self.data.find(pattern)
        }

        /// 在指定数组中查找
        pub fn find_in(&self, index: usize, pattern: &str) -> Option<Vec<usize>> {
                self.data.find_in_array(index, pattern)
        }

        /// 追加元素
        pub fn append(&mut self, index: usize, value: &str) -> Result<(), String> {
                let val = parse_simple_value(value);
                self.data.append(index, val)
        }

        /// 插入元素
        pub fn insert(&mut self, index: usize, position: usize, value: &str) -> Result<(), String> {
                let val = parse_simple_value(value);
                self.data.insert(index, position, val)
        }

        /// 删除元素
        pub fn delete(&mut self, index: usize) -> Result<(), String> {
                self.data.delete(index)
        }

        /// 替换元素
        pub fn replace(&mut self, index: usize, value: &str) -> Result<(), String> {
                let val = parse_simple_value(value);
                self.data.replace(index, val)
        }

        /// 保存为文本文件
        pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
                use std::fs;
                fs::write(path, self.to_string())
                        .map_err(|e| format!("写入文件失败: {}", e))
        }

        /// 保存为二进制文件
        #[cfg(feature = "binary")]
        pub fn save_binary<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
                self.data.save_binary(path.as_ref().to_str().unwrap_or(""))
        }

        /// 导出为文本
        #[cfg(feature = "binary")]
        pub fn export_text<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
                self.data.export_text(path.as_ref().to_str().unwrap_or(""))
        }

        /// 转换为 Vec<ListItem>
        pub fn to_vec(&self) -> Vec<ListItem> {
                (0..self.len()).map(|i| self.get(i)).collect()
        }

        /// ForEach 遍历
        pub fn for_each<F>(&self, mut f: F)
        where
                F: FnMut(ListItem),
        {
                for i in 0..self.len() {
                        f(self.get(i));
                }
        }

        /// Map 映射
        pub fn map<F, T>(&self, mut f: F) -> Vec<T>
        where
                F: FnMut(ListItem) -> T,
        {
                (0..self.len()).map(|i| f(self.get(i))).collect()
        }

        /// Filter 过滤
        pub fn filter<F>(&self, mut f: F) -> Vec<ListItem>
        where
                F: FnMut(&ListItem) -> bool,
        {
                (0..self.len())
                        .filter_map(|i| {
                                let item = self.get(i);
                                if f(&item) {
                                        Some(item)
                                } else {
                                        None
                                }
                        })
                        .collect()
        }

        /// 获取原始数据引用
        pub fn raw(&self) -> &ListData {
                &self.data
        }

        /// 获取可变数据引用
        pub fn raw_mut(&mut self) -> &mut ListData {
                &mut self.data
        }
}

// 实现 Index trait (注意: 由于 Rust 所有权限制，这里不能完美实现)
// 推荐使用 l.get(index) 方法代替 l[index]
impl Index<usize> for List {
        type Output = ListItem;

        fn index(&self, _index: usize) -> &Self::Output {
                panic!("请使用 l.get(index) 方法代替 l[index]，因为 Rust 的所有权规则限制");
        }
}

// 为了更好的用户体验，提供 get 方法作为替代
impl List {
        /// 使用 [] 操作符风格 (返回拥有的 ListItem)
        ///
        /// 注意：由于 Rust 的所有权规则，这里不能直接实现为 operator[]
        /// 请使用 l.get(0) 或 l.at(&[0, 1])
        pub fn idx(&self, index: usize) -> ListItem {
                self.get(index)
        }
}

impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.to_string())
        }
}

impl fmt::Debug for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "List({})", self.to_string())
        }
}

impl Clone for List {
        fn clone(&self) -> Self {
                List {
                        data: self.data.clone(),
                }
        }
}

/// 迭代器
pub struct ListIter {
        list: List,
        position: usize,
}

impl Iterator for ListIter {
        type Item = ListItem;

        fn next(&mut self) -> Option<Self::Item> {
                if self.position < self.list.len() {
                        let item = self.list.get(self.position);
                        self.position += 1;
                        Some(item)
                } else {
                        None
                }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
                let remaining = self.list.len() - self.position;
                (remaining, Some(remaining))
        }
}

impl IntoIterator for List {
        type Item = ListItem;
        type IntoIter = ListIter;

        fn into_iter(self) -> Self::IntoIter {
                ListIter {
                        list: self,
                        position: 0,
                }
        }
}

impl<'a> IntoIterator for &'a List {
        type Item = ListItem;
        type IntoIter = ListIter;

        fn into_iter(self) -> Self::IntoIter {
                ListIter {
                        list: self.clone(),
                        position: 0,
                }
        }
}

// 辅助函数
fn value_to_string(val: &ListValue) -> String {
        match val {
                ListValue::String(s) => s.clone(),
                ListValue::Array(arr) => format!(
                        "[{}]",
                        arr.iter()
                                .map(|v| value_to_string(v))
                                .collect::<Vec<_>>()
                                .join(",")
                ),
                ListValue::KeyValue(k, v) => format!("{}:{}", k, value_to_string(v)),
        }
}

fn parse_simple_value(s: &str) -> ListValue {
        let trimmed = s.trim();
        if trimmed.contains(',') {
                ListValue::Array(
                        trimmed
                                .split(',')
                                .map(|x| ListValue::String(x.trim().to_string()))
                                .collect(),
                )
        } else {
                ListValue::String(trimmed.to_string())
        }
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_basic_operations() {
                let l = List::new("[a,b,c]; [1,2,3];");

                assert_eq!(l.len(), 2);

                let first = l.get(0);
                assert_eq!(first.value(), "[a,b,c]");

                let second = l.get(1);
                assert_eq!(second.array(), vec!["1", "2", "3"]);
        }

        #[test]
        fn test_nested_access() {
                let l = List::new("[[x,y,z]];");

                let nested = l.get(0).get(0);
                assert_eq!(nested.value(), "x");
        }

        #[test]
        fn test_slice() {
                let l = List::new("[a,b,c,d,e];");

                let slice = l.get(0).slice(1, 3);
                assert_eq!(slice, vec!["b", "c"]);
        }

        #[test]
        fn test_iteration() {
                let l = List::new("[1,2,3];");

                let values: Vec<String> = l.get(0).into_iter().collect();
                assert_eq!(values, vec!["1", "2", "3"]);
        }

        #[test]
        fn test_find() {
                let l = List::new("[apple,banana,cherry]; [dog,cat];");

                let found = l.find("banana");
                assert_eq!(found, vec![0]);
        }

        #[test]
        fn test_modify() {
                let mut l = List::new("[a,b];");

                l.run(".+[0]=c").unwrap();
                assert_eq!(l.to_string(), "[a,b,c];");
        }
}
