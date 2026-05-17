#[cfg(feature = "nodejs")]
use napi::bindgen_prelude::*;
#[cfg(feature = "nodejs")]
use napi_derive::napi;
#[cfg(feature = "nodejs")]
use crate::parser::{ListData, ListValue};

#[cfg(feature = "nodejs")]
#[napi]
pub struct JsListData {
        data: ListData,
}

#[cfg(feature = "nodejs")]
#[napi]
impl JsListData {
        #[napi(constructor)]
        pub fn new(content: Option<String>) -> Result<Self> {
                let data = match content {
                        Some(c) => ListData::from_string(&c)
                                .map_err(|e| napi::Error::from_reason(e))?,
                        None => ListData::new(),
                };
                Ok(JsListData { data })
        }

        #[napi]
        pub fn to_string(&self) -> String {
                self.data.to_string()
        }

        #[napi]
        pub fn get(&self, indices: Vec<u32>) -> Result<String> {
                let idx: Vec<usize> = indices.iter().map(|&i| i as usize).collect();
                self.data.get(&idx)
                        .map(|v| value_to_jsstring(v))
                        .ok_or_else(|| napi::Error::from_reason("索引超出范围"))
        }

        #[napi]
        pub fn append(&mut self, index: u32, value: String) -> Result<String> {
                let val = parse_simple_value_js(&value);
                self.data.append(index as usize, val)
                        .map(|_| self.data.to_string())
                        .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn insert(&mut self, index: u32, position: u32, value: String) -> Result<String> {
                let val = parse_simple_value_js(&value);
                self.data.insert(index as usize, position as usize, val)
                        .map(|_| self.data.to_string())
                        .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn delete(&mut self, index: u32) -> Result<String> {
                self.data.delete(index as usize)
                        .map(|_| self.data.to_string())
                        .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn replace(&mut self, index: u32, new_value: String) -> Result<String> {
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
                self.data.replace(index as usize, val)
                        .map(|_| self.data.to_string())
                        .map_err(|e| napi::Error::from_reason(e))
        }

        #[napi]
        pub fn execute_command(&mut self, command: String) -> Result<String> {
                self.data.execute_command(&command)
                        .map_err(|e| napi::Error::from_reason(e))
        }
}

#[cfg(feature = "nodejs")]
fn value_to_jsstring(val: &ListValue) -> String {
        match val {
                ListValue::String(s) => s.clone(),
                ListValue::Array(arr) => format!("[{}]", arr.iter()
                        .map(|v| value_to_jsstring(v))
                        .collect::<Vec<_>>()
                        .join(",")),
                ListValue::KeyValue(k, v) => format!("{}:{}", k, value_to_jsstring(v)),
        }
}

#[cfg(feature = "nodejs")]
fn parse_simple_value_js(s: &str) -> ListValue {
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
