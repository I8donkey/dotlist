#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyModule;
#[cfg(feature = "python")]
use pyo3::exceptions::PyValueError;
#[cfg(feature = "python")]
use crate::parser::{ListData, ListValue, value_to_string};

#[cfg(feature = "python")]
#[pyclass]
pub struct PyListData {
        data: ListData,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyListData {
        #[new]
        #[pyo3(signature = (content=None))]
        fn new(content: Option<String>) -> PyResult<Self> {
                let data = match content {
                        Some(c) => ListData::from_string(&c)
                                .map_err(|e| PyValueError::new_err(e))?,
                        None => ListData::new(),
                };
                Ok(PyListData { data })
        }

        fn to_string(&self) -> String {
                self.data.to_string()
        }

        fn __len__(&self) -> usize {
                self.data.len()
        }

        fn __repr__(&self) -> String {
                format!("List({})", self.data.to_string())
        }

        fn __str__(&self) -> String {
                self.data.to_string()
        }

        fn __getitem__(&self, index: usize) -> PyResult<String> {
                self.data.get(&vec![index])
                        .map(|v| value_to_pystring(v))
                        .ok_or_else(|| PyValueError::new_err(format!("索引 {} 超出范围", index)))
        }

        fn __iadd__(&mut self, other: &PyListData) -> () {
                // 合并两个列表
                // 如果 both 都是单个数组元素，合并数组内容
                if self.data.items.len() == 1 && other.data.items.len() == 1 {
                        if let (Some(ListValue::Array(arr1)), Some(ListValue::Array(arr2))) = 
                            (self.data.items.first(), other.data.items.first()) {
                                // 合并数组
                                let mut new_arr = arr1.clone();
                                new_arr.extend(arr2.clone());
                                self.data.items[0] = ListValue::Array(new_arr);
                                return;
                        }
                }
                // 否则直接合并整个 items
                self.data.items.extend(other.data.items.clone());
        }

        fn __add__(&self, other: &PyListData) -> PyResult<Self> {
                // 合并两个列表（非原地操作）
                let mut new_data = self.data.clone();
                
                // 如果 both 都是单个数组元素，合并数组内容
                if new_data.items.len() == 1 && other.data.items.len() == 1 {
                        if let (Some(ListValue::Array(arr1)), Some(ListValue::Array(arr2))) = 
                            (new_data.items.first(), other.data.items.first()) {
                                let mut new_arr = arr1.clone();
                                new_arr.extend(arr2.clone());
                                new_data.items[0] = ListValue::Array(new_arr);
                                return Ok(PyListData { data: new_data });
                        }
                }
                
                // 否则直接合并整个 items
                new_data.items.extend(other.data.items.clone());
                Ok(PyListData { data: new_data })
        }

        fn __isub__(&mut self, other: &PyListData) -> () {
                // 去除 self 中所有 other 中的元素
                // 如果 both 都是单个数组元素，从数组中移除元素
                if self.data.items.len() == 1 && other.data.items.len() == 1 {
                        if let (Some(ListValue::Array(arr1)), Some(ListValue::Array(arr2))) = 
                            (self.data.items.first(), other.data.items.first()) {
                                // 移除前 len(arr2) 个元素
                                if arr1.len() >= arr2.len() {
                                        let new_arr: Vec<_> = arr1.iter()
                                                .skip(arr2.len())
                                                .cloned()
                                                .collect();
                                        self.data.items[0] = ListValue::Array(new_arr);
                                }
                                return;
                        }
                }
                
                // 否则在 items 级别移除
                let other_strs: Vec<String> = other.data.items.iter()
                        .map(|v| value_to_string(v))
                        .collect();
                
                self.data.items.retain(|item| {
                        !other_strs.contains(&value_to_string(item))
                });
        }

        fn __sub__(&self, other: &PyListData) -> PyResult<Self> {
                // 去除 self 中所有 other 中的元素（非原地操作）
                let mut new_data = self.data.clone();
                
                // 如果 both 都是单个数组元素，从数组中移除元素
                if new_data.items.len() == 1 && other.data.items.len() == 1 {
                        if let (Some(ListValue::Array(arr1)), Some(ListValue::Array(arr2))) = 
                            (new_data.items.first(), other.data.items.first()) {
                                // 移除前 len(arr2) 个元素
                                if arr1.len() >= arr2.len() {
                                        let new_arr: Vec<_> = arr1.iter()
                                                .skip(arr2.len())
                                                .cloned()
                                                .collect();
                                        new_data.items[0] = ListValue::Array(new_arr);
                                }
                                return Ok(PyListData { data: new_data });
                        }
                }
                
                // 否则在 items 级别移除
                let other_strs: Vec<String> = other.data.items.iter()
                        .map(|v| value_to_string(v))
                        .collect();
                
                let new_items: Vec<_> = new_data.items.into_iter()
                        .filter(|item| !other_strs.contains(&value_to_string(item)))
                        .collect();
                
                new_data.items = new_items;
                Ok(PyListData { data: new_data })
        }

        fn len(&self) -> usize {
                self.data.len()
        }

        fn is_empty(&self) -> bool {
                self.data.is_empty()
        }

        fn get(&self, indices: Vec<usize>) -> PyResult<String> {
                self.data.get(&indices)
                        .map(|v| value_to_pystring(v))
                        .ok_or_else(|| PyValueError::new_err("索引超出范围"))
        }

        fn get_array(&self, index: usize) -> PyResult<Vec<String>> {
                self.data.get_array(index)
                        .ok_or_else(|| PyValueError::new_err(format!("索引 {} 不是数组或不存在", index)))
        }

        fn get_slice(&self, index: usize, start: Option<usize>, end: Option<usize>) -> PyResult<Vec<String>> {
                let s = start.unwrap_or(0);
                let e = end.unwrap_or(0);
                self.data.get_slice(index, s, e)
                        .ok_or_else(|| PyValueError::new_err(format!("索引 {} 不是数组或切片无效", index)))
        }

        fn find(&self, pattern: String) -> Vec<usize> {
                self.data.find(&pattern)
        }

        fn find_in_array(&self, index: usize, pattern: String) -> PyResult<Vec<usize>> {
                self.data.find_in_array(index, &pattern)
                        .ok_or_else(|| PyValueError::new_err(format!("索引 {} 不是数组", index)))
        }

        fn append(&mut self, index: usize, value: String) -> PyResult<String> {
                let val = parse_simple_value(&value);
                self.data.append(index, val)
                        .map(|_| self.data.to_string())
                        .map_err(|e| PyValueError::new_err(e))
        }

        fn insert(&mut self, index: usize, position: usize, value: String) -> PyResult<String> {
                let val = parse_simple_value(&value);
                self.data.insert(index, position, val)
                        .map(|_| self.data.to_string())
                        .map_err(|e| PyValueError::new_err(e))
        }

        fn delete(&mut self, index: usize) -> PyResult<String> {
                self.data.delete(index)
                        .map(|_| self.data.to_string())
                        .map_err(|e| PyValueError::new_err(e))
        }

        fn replace(&mut self, index: usize, new_value: String) -> PyResult<String> {
                let val = if new_value.starts_with('[') && new_value.ends_with(']') {
                        let inner = &new_value[1..new_value.len()-1];
                        if inner.is_empty() || inner.trim().is_empty() {
                                ListValue::Array(Vec::new())
                        } else {
                                ListValue::Array(
                                        inner.split(',')
                                                .map(|s| ListValue::String(s.trim().to_string()))
                                                .collect()
                                )
                        }
                } else {
                        ListValue::String(new_value)
                };
                self.data.replace(index, val)
                        .map(|_| self.data.to_string())
                        .map_err(|e| PyValueError::new_err(e))
        }

        fn execute_command(&mut self, command: String) -> PyResult<String> {
                self.data.execute_command(&command)
                        .map_err(|e| PyValueError::new_err(e))
        }

        #[cfg(feature = "binary")]
        fn save_binary(&self, path: String) -> PyResult<()> {
                self.data.save_binary(&path)
                        .map_err(|e| PyValueError::new_err(e))
        }

        #[cfg(feature = "binary")]
        #[staticmethod]
        fn load_binary(path: String) -> PyResult<Self> {
                ListData::load_binary(&path)
                        .map(|data| PyListData { data })
                        .map_err(|e| PyValueError::new_err(e))
        }

        #[cfg(feature = "binary")]
        fn export_text(&self, path: String) -> PyResult<()> {
                self.data.export_text(&path)
                        .map_err(|e| PyValueError::new_err(e))
        }

        #[cfg(feature = "binary")]
        #[staticmethod]
        fn is_binary_file(path: &str) -> bool {
                path.ends_with(".listb") || path.ends_with(".listbin")
        }
}

#[cfg(feature = "python")]
fn value_to_pystring(val: &ListValue) -> String {
        match val {
                ListValue::String(s) => s.clone(),
                ListValue::Array(arr) => format!("[{}]", arr.iter()
                        .map(|v| value_to_pystring(v))
                        .collect::<Vec<_>>()
                        .join(",")),
                ListValue::KeyValue(k, v) => format!("{}:{}", k, value_to_pystring(v)),
        }
}

#[cfg(feature = "python")]
fn parse_simple_value(s: &str) -> ListValue {
        let trimmed = s.trim();
        if trimmed.contains(',') {
                ListValue::Array(
                        trimmed.split(',')
                                .map(|x| ListValue::String(x.trim().to_string()))
                                .collect()
                )
        } else {
                ListValue::String(trimmed.to_string())
        }
}

#[cfg(feature = "python")]
#[pymodule(name = "dotlist")]
fn list_lang(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_class::<PyListData>()?;
        Ok(())
}
