use regex::Regex;
use std::collections::HashMap;

#[cfg(feature = "binary")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "binary", derive(Serialize, Deserialize))]
pub enum ListValue {
        String(String),
        Array(Vec<ListValue>),
        KeyValue(String, Box<ListValue>),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "binary", derive(Serialize, Deserialize))]
pub struct ListData {
        pub items: Vec<ListValue>,
        pub variables: HashMap<String, ListValue>,
        pub language: String,
}

impl ListData {
        pub fn new() -> Self {
                ListData {
                        items: Vec::new(),
                        variables: HashMap::new(),
                        language: "en".to_string(),
                }
        }

        pub fn from_string(content: &str) -> Result<Self, String> {
                let mut data = ListData::new();
                let chars: Vec<char> = content.chars().collect();
                let mut pos = 0;

                while pos < chars.len() {
                        match chars[pos] {
                                '[' | 'a'..='z' | 'A'..='Z' | '_' => {
                                        let (value, new_pos) = parse_value(&chars, pos)?;
                                        data.items.push(value);
                                        pos = new_pos;
                                }
                                ';' => {
                                        pos += 1;
                                }
                                ',' | ' ' | '\n' | '\r' | '\t' => {
                                        pos += 1;
                                }
                                '\\' => {
                                        let (val, new_pos) = parse_escaped_string(&chars, pos)?;
                                        data.items.push(val);
                                        pos = new_pos;
                                }
                                c => {
                                        return Err(format!("意外的字符 '{}' 在位置 {}", c, pos));
                                }
                        }
                }
                Ok(data)
        }

        pub fn to_string(&self) -> String {
                self.items.iter()
                        .map(|v| value_to_string(v))
                        .collect::<Vec<_>>()
                        .join("; ")
                        + ";"
        }

        pub fn len(&self) -> usize {
                self.items.len()
        }

        pub fn is_empty(&self) -> bool {
                self.items.is_empty()
        }

        pub fn get_array(&self, index: usize) -> Option<Vec<String>> {
                match self.get(&[index]) {
                        Some(ListValue::Array(arr)) => Some(
                                arr.iter().map(|v| value_to_string(v)).collect()
                        ),
                        _ => None,
                }
        }

        pub fn get_slice(&self, index: usize, start: usize, end: usize) -> Option<Vec<String>> {
                match self.get(&[index]) {
                        Some(ListValue::Array(arr)) => {
                                let actual_end = if end == 0 || end > arr.len() { arr.len() } else { end };
                                let actual_start = if start > arr.len() { arr.len() } else { start };
                                if actual_start >= actual_end { return Some(Vec::new()); }
                                Some(arr[actual_start..actual_end].iter()
                                        .map(|v| value_to_string(v))
                                        .collect())
                        },
                        _ => None,
                }
        }

        pub fn find(&self, pattern: &str) -> Vec<usize> {
                let mut results = Vec::new();
                for (i, item) in self.items.iter().enumerate() {
                        let s = value_to_string(item);
                        if s.contains(pattern) || s.starts_with(pattern) {
                                results.push(i);
                        }
                }
                results
        }

        pub fn find_in_array(&self, index: usize, pattern: &str) -> Option<Vec<usize>> {
                match self.get(&[index]) {
                        Some(ListValue::Array(arr)) => {
                                let mut indices = Vec::new();
                                for (i, item) in arr.iter().enumerate() {
                                        let s = value_to_string(item);
                                        if s.contains(pattern) || s == pattern {
                                                indices.push(i);
                                        }
                                }
                                Some(indices)
                        },
                        _ => None,
                }
        }

        pub fn get(&self, indices: &[usize]) -> Option<&ListValue> {
                if indices.is_empty() { return None; }
                let mut current = self.items.get(indices[0])?;
                for &idx in &indices[1..] {
                        current = match current {
                                ListValue::Array(arr) => arr.get(idx)?,
                                ListValue::KeyValue(_, v) => {
                                        let val = v.as_ref() as &ListValue;
                                        match val {
                                                ListValue::Array(arr) => arr.get(idx)?,
                                                _ => return None,
                                        }
                                }
                                _ => return None,
                        };
                }
                Some(current)
        }

        pub fn append(&mut self, index: usize, value: ListValue) -> Result<(), String> {
                match self.items.get_mut(index) {
                        Some(ListValue::Array(arr)) => {
                                arr.push(value);
                                Ok(())
                        }
                        _ => Err(format!("索引 {} 不是数组", index)),
                }
        }

        pub fn insert(&mut self, index: usize, position: usize, value: ListValue) -> Result<(), String> {
                match self.items.get_mut(index) {
                        Some(ListValue::Array(arr)) => {
                                if position <= arr.len() {
                                        arr.insert(position, value);
                                        Ok(())
                                } else {
                                        Err(format!("位置 {} 超出范围", position))
                                }
                        }
                        _ => Err(format!("索引 {} 不是数组", index)),
                }
        }

        pub fn delete(&mut self, index: usize) -> Result<(), String> {
                if index < self.items.len() {
                        self.items.remove(index);
                        Ok(())
                } else {
                        Err(format!("索引 {} 超出范围", index))
                }
        }

        pub fn replace(&mut self, index: usize, new_value: ListValue) -> Result<(), String> {
                if index < self.items.len() {
                        self.items[index] = new_value;
                        Ok(())
                } else {
                        Err(format!("索引 {} 超出范围", index))
                }
        }

        pub fn execute_command(&mut self, command: &str) -> Result<String, String> {
                let cmd = command.trim();

                if cmd.is_empty() {
                        return Ok(self.to_string());
                }

                if cmd == "exit()" || cmd == "exit" {
                        std::process::exit(0);
                }

                if cmd == ".h" || cmd == ".help" {
                        return Ok(self.get_help());
                }

                // [n]< - 读取第 n 个元素的值（反向访问）
                if cmd.ends_with('<') && cmd.starts_with('[') {
                        let re = Regex::new(r"\[(\d+)\]<").unwrap();
                        if let Some(caps) = re.captures(cmd) {
                                let idx: usize = caps[1].parse::<usize>().map_err(|e| e.to_string())?;
                                if let Some(val) = self.items.get(idx) {
                                        return Ok(value_to_string(val));
                                }
                                return Err("索引超出范围".to_string());
                        }
                }

                if cmd.starts_with('$') && cmd.contains('=') {
                        return self.handle_variable_assignment(cmd);
                }

                if cmd.contains('|') {
                        return self.handle_pipeline(cmd);
                }

                if cmd.starts_with("${") && cmd.ends_with('}') {
                        let inner = &cmd[2..cmd.len()-1];
                        if inner.starts_with('[') {
                                let indices = parse_indices(inner)?;
                                return match self.get(&indices) {
                                        Some(val) => Ok(value_to_string(val)),
                                        None => Err("索引超出范围".to_string()),
                                };
                        }
                }

                let resolved_cmd = self.resolve_references(cmd);
                self.execute_single_command(&resolved_cmd)
        }

        fn handle_query(&self, cmd: &str) -> Result<String, String> {
                // 格式: .?关键词>[n] 或 .?关键词>[n]-[m]
                let re = Regex::new(r"\.\?([^>]+)(?:>(?:\[(\d+)\](?:-\[(\d+)\])?))?").unwrap();
                if let Some(caps) = re.captures(cmd) {
                        let keyword = &caps[1];
                        let start_idx = caps.get(2).map(|m| m.as_str().parse::<usize>().ok()).flatten();
                        let end_idx = caps.get(3).map(|m| m.as_str().parse::<usize>().ok()).flatten();

                        let mut results = Vec::new();

                        if let (Some(start), Some(end)) = (start_idx, end_idx) {
                                // 在 [start] 到 [end] 范围内查询
                                for i in start..=end {
                                        if let Some(val) = self.items.get(i) {
                                                if let Some(found) = self.search_in_value(val, keyword) {
                                                        results.push(format!("[{}]: {}", i, found));
                                                }
                                        }
                                }
                        } else if let Some(idx) = start_idx {
                                // 在 [idx] 中查询
                                if let Some(val) = self.items.get(idx) {
                                        if let Some(found) = self.search_in_value(val, keyword) {
                                                results.push(format!("[{}]: {}", idx, found));
                                        }
                                }
                        } else {
                                // 在根级别查询
                                for (i, val) in self.items.iter().enumerate() {
                                        if let Some(found) = self.search_in_value(val, keyword) {
                                                results.push(format!("[{}]: {}", i, found));
                                        }
                                }
                        }

                        if results.is_empty() {
                                return Ok(format!("未找到关键词: {}", keyword));
                        }
                        return Ok(results.join("\n"));
                }
                Err("无效的查询命令".to_string())
        }

        fn search_in_value(&self, val: &ListValue, keyword: &str) -> Option<String> {
                match val {
                        ListValue::String(s) => {
                                if s.contains(keyword) {
                                        Some(s.clone())
                                } else {
                                        None
                                }
                        }
                        ListValue::Array(arr) => {
                                for item in arr {
                                        if let Some(found) = self.search_in_value(item, keyword) {
                                                return Some(found);
                                        }
                                }
                                None
                        }
                        ListValue::KeyValue(k, v) => {
                                if k.contains(keyword) {
                                        Some(format!("{}: {}", k, value_to_string(v)))
                                } else {
                                        self.search_in_value(v, keyword)
                                }
                        }
                }
        }

        fn handle_print(&self, arg: &str) -> Result<String, String> {
                // 格式: .print>[n] 或 .print>关键词 或 .print>$变量 或 .print>[n]-[m]
                let re = Regex::new(r"(?:\[(\d+)\](?:-\[(\d+)\])?)|(?:(\$[a-zA-Z_][a-zA-Z0-9_]*))|(.+$)").unwrap();

                // 检查是否是范围打印 [n]-[m]
                let range_re = Regex::new(r"^\[(\d+)\]-\[(\d+)\]$").unwrap();
                if let Some(caps) = range_re.captures(arg) {
                        let start: usize = caps[1].parse::<usize>().map_err(|e| e.to_string())?;
                        let end: usize = caps[2].parse::<usize>().map_err(|e| e.to_string())?;
                        let mut results = Vec::new();
                        for i in start..=end {
                                if let Some(val) = self.items.get(i) {
                                        results.push(format!("[{}]: {}", i, value_to_string(val)));
                                }
                        }
                        return Ok(results.join("\n"));
                }

                // 检查是否是变量 $var
                if arg.starts_with('$') {
                        let var_name = &arg[1..];
                        if let Some(val) = self.variables.get(var_name) {
                                return Ok(value_to_string(val));
                        }
                        return Err(format!("变量不存在: {}", var_name));
                }

                // 检查是否是索引 [n]
                if arg.starts_with('[') && arg.ends_with(']') {
                        // 简单索引 [n]
                        let simple_re = Regex::new(r"^\[(\d+)\]$").unwrap();
                        if let Some(caps) = simple_re.captures(arg) {
                                let idx: usize = caps[1].parse::<usize>().map_err(|e| e.to_string())?;
                                if let Some(val) = self.items.get(idx) {
                                        return Ok(value_to_string(val));
                                }
                                return Err("索引超出范围".to_string());
                        }
                        // 嵌套索引 [n][m]
                        let indices = parse_indices(arg)?;
                        if let Some(val) = self.get(&indices) {
                                return Ok(value_to_string(val));
                        }
                        return Err("索引超出范围".to_string());
                }

                // 否则作为普通字符串打印
                Ok(arg.to_string())
        }

        fn handle_move_insert(&mut self, cmd: &str) -> Result<String, String> {
                // 格式:
                // .>[src]>[dest] - 移动到末尾
                // .>[src]>[dest]<[pos] - 插入到前面
                // .>[src]>[dest]>[pos] - 插入到后面

                // 先尝试匹配带位置的模式 .>[src]>[dest]<[pos] 或 .>[src]>[dest]>[pos]
                let pos_re = Regex::new(r"^\.>\[(\d+)\]>\[(\d+)\]<\[(\d+)\]$").unwrap();
                if let Some(caps) = pos_re.captures(cmd) {
                        let src_idx: usize = caps[1].parse::<usize>().map_err(|e| e.to_string())?;
                        let dest_idx: usize = caps[2].parse::<usize>().map_err(|e| e.to_string())?;
                        let pos: usize = caps[3].parse::<usize>().map_err(|e| e.to_string())?;

                        if src_idx >= self.items.len() {
                                return Err(format!("源索引 {} 超出范围", src_idx));
                        }
                        if dest_idx >= self.items.len() {
                                return Err(format!("目标索引 {} 超出范围", dest_idx));
                        }

                        let src_val = self.items.remove(src_idx);
                        if let Some(ListValue::Array(dest_arr)) = self.items.get(dest_idx) {
                                let actual_pos = pos.min(dest_arr.len());
                                if let Some(ListValue::Array(arr)) = self.items.get_mut(dest_idx) {
                                        arr.insert(actual_pos, src_val);
                                }
                        } else {
                                // 如果目标不是数组，先移除后重新插入
                                self.items.insert(dest_idx, src_val);
                        }
                        return Ok(self.to_string());
                }

                let pos_re2 = Regex::new(r"^\.>\[(\d+)\]>\[(\d+)\]>\[(\d+)\]$").unwrap();
                if let Some(caps) = pos_re2.captures(cmd) {
                        let src_idx: usize = caps[1].parse::<usize>().map_err(|e| e.to_string())?;
                        let dest_idx: usize = caps[2].parse::<usize>().map_err(|e| e.to_string())?;
                        let pos: usize = caps[3].parse::<usize>().map_err(|e| e.to_string())?;

                        if src_idx >= self.items.len() {
                                return Err(format!("源索引 {} 超出范围", src_idx));
                        }
                        if dest_idx >= self.items.len() {
                                return Err(format!("目标索引 {} 超出范围", dest_idx));
                        }

                        let src_val = self.items.remove(src_idx);
                        if let Some(ListValue::Array(arr)) = self.items.get_mut(dest_idx) {
                                let actual_pos = (pos + 1).min(arr.len() + 1);
                                arr.insert(actual_pos, src_val);
                        } else {
                                self.items.insert(dest_idx, src_val);
                        }
                        return Ok(self.to_string());
                }

                // 简单移动到末尾 .>[src]>[dest]
                let simple_re = Regex::new(r"^\.>\[(\d+)\]>\[(\d+)\]$").unwrap();
                if let Some(caps) = simple_re.captures(cmd) {
                        let src_idx: usize = caps[1].parse::<usize>().map_err(|e| e.to_string())?;
                        let dest_idx: usize = caps[2].parse::<usize>().map_err(|e| e.to_string())?;

                        if src_idx >= self.items.len() {
                                return Err(format!("源索引 {} 超出范围", src_idx));
                        }
                        if dest_idx >= self.items.len() {
                                return Err(format!("目标索引 {} 超出范围", dest_idx));
                        }

                        // 先移除源值
                        let src_val = self.items.remove(src_idx);
                        
                        // 如果源索引小于目标索引，目标索引需要减1
                        let adjusted_dest_idx = if src_idx < dest_idx {
                                dest_idx - 1
                        } else {
                                dest_idx
                        };
                        
                        // 检查目标是否存在且是数组
                        if let Some(ListValue::Array(arr)) = self.items.get_mut(adjusted_dest_idx) {
                                arr.push(src_val);
                        } else {
                                return Err(format!("目标 [{}] 不是数组", dest_idx));
                        }
                        return Ok(self.to_string());
                }

                Err("无效的移动/插入命令".to_string())
        }

        fn handle_variable_assignment(&mut self, cmd: &str) -> Result<String, String> {
                let re = Regex::new(r"\$(\w+)\s*=\s*(.+)").unwrap();
                if let Some(caps) = re.captures(cmd) {
                        let var_name = caps[1].to_string();
                        let value_expr = caps[2].trim();

                        let value = if value_expr.starts_with('[') {
                                let indices = parse_indices(value_expr)?;
                                match self.get(&indices) {
                                        Some(val) => val.clone(),
                                        None => return Err(format!("索引超出范围: {}", value_expr)),
                                }
                        } else if value_expr.starts_with('"') && value_expr.ends_with('"') {
                                ListValue::String(value_expr[1..value_expr.len()-1].to_string())
                        } else if value_expr.starts_with('[') && value_expr.ends_with(']') {
                                let inner = &value_expr[1..value_expr.len()-1];
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
                                let resolved = self.resolve_references(value_expr);
                                parse_simple_value(&resolved)
                        };

                        self.variables.insert(var_name.clone(), value);
                        Ok(format!("变量 ${} 已设置", var_name))
                } else {
                        Err("无效的变量赋值语法".to_string())
                }
        }

        fn handle_pipeline(&mut self, cmd: &str) -> Result<String, String> {
                let commands: Vec<&str> = cmd.split('|').collect();
                let mut last_result = String::new();

                for (i, pipe_cmd) in commands.iter().enumerate() {
                        let trimmed = pipe_cmd.trim();
                        if i == 0 {
                                last_result = self.execute_single_command(trimmed)?;
                        } else {
                                if trimmed.contains('=') {
                                        let resolved = format!("{}{}{}", 
                                                &trimmed[..trimmed.find('=').unwrap()],
                                                &last_result,
                                                &trimmed[trimmed.find('=').unwrap()+1..]
                                        );
                                        last_result = self.execute_single_command(&resolved)?;
                                } else {
                                        last_result = self.execute_single_command(trimmed)?;
                                }
                        }
                }

                Ok(last_result)
        }

        fn resolve_references(&self, cmd: &str) -> String {
                let mut result = cmd.to_string();

                let inline_re = Regex::new(r"\$\{([^}]+)\}").unwrap();
                result = inline_re.replace_all(&result, |caps: &regex::Captures| {
                        let expr = &caps[1];
                        if expr.starts_with('[') {
                                if let Ok(indices) = parse_indices(expr) {
                                        if let Some(val) = self.get(&indices) {
                                                return value_to_string(val);
                                        }
                                }
                        }
                        expr.to_string()
                }).to_string();

                let var_re = Regex::new(r"\$(\w+)").unwrap();
                result = var_re.replace_all(&result, |caps: &regex::Captures| {
                        let var_name = &caps[1];
                        if let Some(val) = self.variables.get(var_name) {
                                value_to_string(val)
                        } else {
                                format!("${}", var_name)
                        }
                }).to_string();

                result
        }

        fn execute_single_command(&mut self, cmd: &str) -> Result<String, String> {
                let cmd = cmd.trim();

                if cmd.starts_with("${") && cmd.ends_with('}') {
                        let inner = &cmd[2..cmd.len()-1];
                        if inner.starts_with('[') {
                                let indices = parse_indices(inner)?;
                                return match self.get(&indices) {
                                        Some(val) => Ok(value_to_string(val)),
                                        None => Err("索引超出范围".to_string()),
                                };
                        }
                }

                if cmd.starts_with('[') {
                        let indices = parse_indices(cmd)?;
                        return match self.get(&indices) {
                                Some(val) => Ok(value_to_string(val)),
                                None => Err("索引超出范围".to_string()),
                        };
                }

                // .+[n] - 创建新数组（不包含 = 和 >）
                if cmd.starts_with(".+[") && !cmd.contains('=') && !cmd.contains('>') {
                        let re = Regex::new(r"^\.\+\[(\d+)\]$").unwrap();
                        if let Some(caps) = re.captures(cmd) {
                                let idx: usize = caps[1].parse::<usize>().map_err(|e| e.to_string())?;
                                while self.items.len() <= idx {
                                        self.items.push(ListValue::Array(Vec::new()));
                                }
                                self.items[idx] = ListValue::Array(Vec::new());
                                return Ok(self.to_string());
                        }
                }

                if cmd.starts_with(".+[") && !cmd.contains('>') {
                        let re = Regex::new(r"\.\+\[(\d+)\]=(.+)").unwrap();
                        if let Some(caps) = re.captures(cmd) {
                                let idx: usize = caps[1].parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
                                let val_str = &caps[2];
                                let val = if val_str.starts_with('[') && val_str.ends_with(']') {
                                        let inner = &val_str[1..val_str.len()-1];
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
                                        parse_simple_value(val_str)
                                };
                                self.append(idx, val)?;
                                return Ok(self.to_string());
                        }
                        return Err("无效的追加命令".to_string());
                }

                if cmd.starts_with(".+>[") && cmd.contains('=') && !cmd.contains("]>[") {
                        let re = Regex::new(r"\.\+>\[(\d+)\]=\[([^\]]*)\]").unwrap();
                        if let Some(caps) = re.captures(cmd) {
                                let idx: usize = caps[1].parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
                                let arr_content = &caps[2];
                                let arr = if arr_content.is_empty() || arr_content.trim().is_empty() {
                                        Vec::new()
                                } else {
                                        arr_content.split(',')
                                                .map(|s| ListValue::String(s.trim().to_string()))
                                                .collect()
                                };
                                self.replace(idx, ListValue::Array(arr))?;
                                return Ok(self.to_string());
                        }
                        return Err("无效的替换命令".to_string());
                }

                if cmd.starts_with(".+[") && cmd.contains('>') && !cmd.starts_with(".+>[") {
                        let re = Regex::new(r"\.\+\[(\d+)\]>\[(\d+)\]=(.+)").unwrap();
                        if let Some(caps) = re.captures(cmd) {
                                let idx: usize = caps[1].parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
                                let pos_idx: usize = caps[2].parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
                                let val = parse_simple_value(&caps[3]);
                                self.insert(idx, pos_idx, val)?;
                                return Ok(self.to_string());
                        }
                        return Err("无效的插入命令".to_string());
                }

                if cmd.starts_with(".+>[") {
                        let re = Regex::new(r"\.\+>\[(\d+)\]>\[(\d+)\]=(.+)").unwrap();
                        if let Some(caps) = re.captures(cmd) {
                                let idx: usize = caps[1].parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
                                let pos_idx: usize = caps[2].parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
                                let val = parse_simple_value(&caps[3]);
                                self.insert(idx, pos_idx, val)?;
                                return Ok(self.to_string());
                        }
                        return Err("无效的插入命令".to_string());
                }

                if cmd.starts_with(".->[") {
                        let re = Regex::new(r"\.\->\[(\d+)\]").unwrap();
                        if let Some(caps) = re.captures(cmd) {
                                let idx: usize = caps[1].parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
                                self.delete(idx)?;
                                return Ok(self.to_string());
                        }
                        return Err("无效的删除命令".to_string());
                }

                if cmd.starts_with("$vars") || cmd == "$v" || cmd == "$variables" {
                        let vars: Vec<String> = self.variables.iter()
                                .map(|(k, v)| format!("{} = {}", k, value_to_string(v)))
                                .collect();
                        if vars.is_empty() {
                                return Ok("(无变量)".to_string());
                        }
                        return Ok(vars.join("\n"));
                }

                if cmd.starts_with("$clear") {
                        self.variables.clear();
                        return Ok("变量已清空".to_string());
                }

                // .h 或 .help - 显示帮助
                if cmd == ".h" || cmd == ".help" {
                        return Ok(self.get_help());
                }

                // .setlang= 或 .sl= - 设置语言
                if cmd.starts_with(".setlang=") || cmd.starts_with(".sl=") {
                        let lang = if cmd.starts_with(".setlang=") {
                                &cmd[9..]
                        } else {
                                &cmd[4..]
                        };
                        self.language = lang.trim().to_string();
                        return Ok(format!("语言已设置为: {}", self.language));
                }

                // .?关键词 - 查询命令
                if cmd.starts_with(".?") {
                        return self.handle_query(cmd);
                }

                // .print>xxx - 打印命令
                if cmd.starts_with(".print>") {
                        return self.handle_print(&cmd[7..]);
                }

                // .>[src]>[dest] - 移动/插入命令
                if cmd.starts_with(".>[") {
                        return self.handle_move_insert(cmd);
                }

                Err(format!("未知命令: {}", cmd))
        }
}

fn parse_value(chars: &[char], start: usize) -> Result<(ListValue, usize), String> {
        if chars[start] == '[' {
                return parse_array(chars, start);
        }

        if is_name_start(chars[start]) {
                return parse_key_or_string(chars, start);
        }

        if chars[start].is_numeric() {
                return parse_number(chars, start);
        }

        Err(format!("无法解析值，起始字符: '{}'", chars[start]))
}

fn parse_escaped_string(chars: &[char], start: usize) -> Result<(ListValue, usize), String> {
        assert_eq!(chars[start], '\\');
        let mut pos = start + 1;
        let mut result = String::new();
        result.push('\\');

        while pos < chars.len() {
                let c = chars[pos];
                match c {
                        ',' | ']' | ';' | '[' => {
                                result.push(c);
                                return Ok((ListValue::String(result), pos + 1));
                        }
                        ' ' | '\n' | '\r' | '\t' => {
                                result.push(c);
                                pos += 1;
                        }
                        _ => {
                                result.push(c);
                                pos += 1;
                        }
                }
        }

        Ok((ListValue::String(result), pos))
}

fn is_name_start(c: char) -> bool {
        c.is_alphabetic() || c == '_'
}

fn parse_number(chars: &[char], start: usize) -> Result<(ListValue, usize), String> {
        let mut pos = start;
        let mut has_dot = false;

        while pos < chars.len() {
                match chars[pos] {
                        '0'..='9' => { pos += 1; }
                        '.' if !has_dot => { has_dot = true; pos += 1; }
                        _ => break,
                }
        }

        let num_str: String = chars[start..pos].iter().collect();
        Ok((ListValue::String(num_str), pos))
}

fn is_name_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
}

fn parse_array(chars: &[char], start: usize) -> Result<(ListValue, usize), String> {
        assert_eq!(chars[start], '[');
        let mut pos = start + 1;
        let mut arr = Vec::new();

        while pos < chars.len() {
                match chars[pos] {
                        ']' => {
                                return Ok((ListValue::Array(arr), pos + 1));
                        }
                        ',' | ' ' | '\n' | '\r' | '\t' => {
                                pos += 1;
                                continue;
                        }
                        '\\' => {
                                let (val, new_pos) = parse_escaped_string_in_array(chars, pos)?;
                                arr.push(val);
                                pos = new_pos;
                        }
                        '[' | 'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                                let (val, new_pos) = parse_value(chars, pos)?;
                                arr.push(val);
                                pos = new_pos;
                        }
                        ':' => {
                                let (val, new_pos) = parse_colon_value(chars, pos)?;
                                arr.push(val);
                                pos = new_pos;
                        }
                        c => {
                                return Err(format!("数组中意外的字符 '{}' 在位置 {}", c, pos));
                        }
                }
        }

        Err("未闭合的数组".to_string())
}

fn parse_escaped_string_in_array(chars: &[char], start: usize) -> Result<(ListValue, usize), String> {
        assert_eq!(chars[start], '\\');
        let mut pos = start;
        let mut result = String::new();

        while pos < chars.len() {
                let c = chars[pos];

                match c {
                        '\\' => {
                                if pos + 1 < chars.len() {
                                        let next = chars[pos + 1];
                                        result.push('\\');
                                        result.push(next);
                                        pos += 2;

                                        match next {
                                                ':' | ']' | '[' | ',' | ';' | '\\' => {
                                                        break;
                                                }
                                                _ => continue,
                                        }
                                } else {
                                        result.push(c);
                                        pos += 1;
                                        break;
                                }
                        }
                        ',' | ']' | ';' | '[' => {
                                if !result.is_empty() {
                                        result.push(c);
                                }
                                pos += 1;
                                break;
                        }
                        ' ' | '\n' | '\r' | '\t' => {
                                if !result.ends_with(' ') {
                                        result.push(c);
                                }
                                pos += 1;
                        }
                        _ => {
                                result.push(c);
                                pos += 1;

                                if result.len() > 2 && !result.starts_with('\\') {
                                        let last_char = result.chars().last().unwrap();
                                        if last_char == ',' || last_char == ':' || last_char == ']' {
                                                break;
                                        }
                                }
                        }
                }
        }

        Ok((ListValue::String(result.trim_end().to_string()), pos))
}

fn parse_colon_value(chars: &[char], start: usize) -> Result<(ListValue, usize), String> {
        assert_eq!(chars[start], ':');
        let mut pos = start + 1;
        let mut result = String::new();
        result.push(':');

        while pos < chars.len() {
                let c = chars[pos];
                match c {
                        ',' | ']' | ';' => {
                                result.push(c);
                                pos += 1;
                                break;
                        }
                        '[' => {
                                let (arr_val, new_pos) = parse_array(chars, pos)?;
                                return Ok((ListValue::KeyValue(
                                        result.trim_matches(':').trim().to_string(),
                                        Box::new(arr_val)
                                ), new_pos));
                        }
                        '\\' => {
                                result.push(c);
                                pos += 1;
                                if pos < chars.len() {
                                        result.push(chars[pos]);
                                        pos += 1;
                                }
                        }
                        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' | ':' => {
                                result.push(c);
                                pos += 1;
                        }
                        ' ' | '\n' | '\r' | '\t' => {
                                pos += 1;
                                if !result.ends_with(' ') {
                                        result.push(' ');
                                }
                        }
                        _ => {
                                pos += 1;
                        }
                }
        }

        let parts: Vec<&str> = result.split(':').collect();
        if parts.len() >= 2 {
                let key = parts[0].trim().to_string();
                let val = parts[1..].join(":").trim().to_string();
                Ok((ListValue::KeyValue(key, Box::new(ListValue::String(val))), pos))
        } else {
                Ok((ListValue::String(result), pos))
        }
}

fn parse_key_or_string(chars: &[char], start: usize) -> Result<(ListValue, usize), String> {
        let mut pos = start;

        let name_start = pos;
        while pos < chars.len() && is_name_char(chars[pos]) {
                pos += 1;
        }
        let name: String = chars[name_start..pos].iter().collect();

        if pos < chars.len() && chars[pos] == ':' {
                pos += 1;

                if pos < chars.len() && chars[pos] == '[' {
                        let (val, new_pos) = parse_array(chars, pos)?;
                        Ok((ListValue::KeyValue(name, Box::new(val)), new_pos))
                } else {
                        let val_start = pos;
                        while pos < chars.len() && !is_terminator(chars[pos]) {
                                pos += 1;
                        }
                        let val: String = chars[val_start..pos].iter().collect();
                        Ok((ListValue::KeyValue(name, Box::new(ListValue::String(val))), pos))
                }
        } else {
                Ok((ListValue::String(name), pos))
        }
}

fn is_terminator(c: char) -> bool {
        matches!(c, ',' | ']' | ';')
}

pub fn value_to_string(val: &ListValue) -> String {
        match val {
                ListValue::String(s) => s.clone(),
                ListValue::Array(arr) => {
                        format!("[{}]", arr.iter()
                                .map(|v| value_to_string(v))
                                .collect::<Vec<_>>()
                                .join(","))
                }
                ListValue::KeyValue(k, v) => format!("{}:{}", k, value_to_string(v)),
        }
}

fn parse_indices(input: &str) -> Result<Vec<usize>, String> {
        let re = Regex::new(r"\d+").unwrap();
        re.find_iter(input)
                .map(|m| m.as_str().parse::<usize>().map_err(|e| e.to_string()))
                .collect()
}

fn parse_simple_value(s: &str) -> ListValue {
        let trimmed = s.trim();
        if trimmed.contains(',') {
                return ListValue::Array(
                        trimmed
                                .split(',')
                                .map(|x| ListValue::String(x.trim().to_string()))
                                .collect()
                );
        } else {
                return ListValue::String(trimmed.to_string());
        }
}

impl ListData {
        pub fn get_help(&self) -> String {
                r#".list 命令帮助
================

【读取操作】
  [n]           - 读取第 n 个元素
  [n][m]        - 读取第 n 个元素的第 m 个子元素
  [n]/          - 读取第 n 个元素的键
  [n]<          - 读取第 n 个元素的值

【修改操作】
  .+[n]         - 在第 n 个位置创建新数组
  .+[n]=值      - 将值追加到第 n 个数组
  .+[n]>[m]=值  - 将值插入到第 n 个数组的第 m 位置
  .+>[n]=[...]  - 替换第 n 个数组为新内容
  .->[n]        - 删除第 n 个元素

【移动/插入操作】
  .>[src]>[dest]       - 把 [src] 移动到 [dest] 的末尾
  .>[src]>[dest]<[pos] - 把 [src] 插入到 [dest][pos] 的前面
  .>[src]>[dest]>[pos] - 把 [src] 插入到 [dest][pos] 的后面

【查询操作】
  .?关键词              - 在根级别查询关键词
  .?关键词>[n]          - 在 [n] 中查询关键词
  .?关键词>[n]-[m]      - 在 [n] 到 [m] 中查询关键词

【打印操作】
  .print>[n]           - 打印 [n] 的内容
  .print>关键词        - 打印关键词
  .print>$变量         - 打印变量
  .print>[n]-[m]       - 打印 [n] 到 [m] 的内容

【变量操作】
  $var=值       - 设置变量
  $var          - 读取变量
  $vars         - 显示所有变量

【语言设置】
  .setlang=en   - 设置语言为英文
  .sl=zh        - 设置语言为中文

【其他命令】
  .h 或 .help   - 显示此帮助
  exit()        - 退出程序
"#.to_string()
        }
}

#[cfg(feature = "binary")]
impl ListData {
        const BINARY_MAGIC: &[u8; 4] = b"LIST";
        const BINARY_VERSION: u8 = 1;

        pub fn save_binary(&self, path: &str) -> Result<(), String> {
                use std::io::Write;
                use flate2::write::ZlibEncoder;
                use flate2::Compression;

                let serialized = bincode::serialize(self)
                        .map_err(|e| format!("序列化失败: {}", e))?;

                // 先压缩数据
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
                encoder.write_all(&serialized)
                        .map_err(|e| format!("压缩数据失败: {}", e))?;

                let compressed = encoder.finish()
                        .map_err(|e| format!("压缩完成失败: {}", e))?;

                // 创建完整文件内容：magic + version + 压缩数据
                let mut file_content = Vec::new();
                file_content.extend_from_slice(Self::BINARY_MAGIC);
                file_content.push(Self::BINARY_VERSION);
                file_content.extend_from_slice(&compressed);

                std::fs::write(path, &file_content)
                        .map_err(|e| format!("写入文件失败: {}", e))?;

                Ok(())
        }

        pub fn load_binary(path: &str) -> Result<Self, String> {
                use std::io::Read;
                use flate2::read::ZlibDecoder;

                let file_content = std::fs::read(path)
                        .map_err(|e| format!("读取文件失败: {}", e))?;

                if file_content.len() < 5 {
                        return Err("文件太小，不是有效的二进制文件".to_string());
                }

                let magic = &file_content[0..4];
                if magic != Self::BINARY_MAGIC {
                        return Err("无效的文件格式 (magic不匹配)".to_string());
                }

                let version = file_content[4];
                if version != Self::BINARY_VERSION {
                        return Err(format!("不支持的版本: {}", version));
                }

                let mut decoder = ZlibDecoder::new(&file_content[5..]);
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)
                        .map_err(|e| format!("解压失败: {}", e))?;

                let data: ListData = bincode::deserialize(&decompressed)
                        .map_err(|e| format!("反序列化失败: {}", e))?;

                Ok(data)
        }

        pub fn export_text(&self, path: &str) -> Result<(), String> {
                std::fs::write(path, self.to_string())
                        .map_err(|e| format!("导出文本失败: {}", e))
        }

        pub fn convert_to_binary(text_path: &str, binary_path: &str) -> Result<(), String> {
                let content = std::fs::read_to_string(text_path)
                        .map_err(|e| format!("读取文本文件失败: {}", e))?;
                let data = ListData::from_string(&content)?;
                data.save_binary(binary_path)
        }

        pub fn convert_to_text(binary_path: &str, text_path: &str) -> Result<(), String> {
                let data = ListData::load_binary(binary_path)?;
                data.export_text(text_path)
        }
}

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn test_parse_example() {
                let content = "red:[\\:,:\\],,a:b]; blue:[hello:hi,bye,light:www]; [1,2,3,4,5,6,7,8,9];";
                let data = ListData::from_string(content).expect("解析失败");

                println!("完整数据: {}", data.to_string());

                // 测试 [0] - 应该返回 red 的键值对
                let val0 = data.get(&[0]);
                assert!(val0.is_some(), "[0] 应该存在");
                println!("[0] = {:?}", val0);

                // 测试 [1] - 应该返回 blue 的键值对
                let val1 = data.get(&[1]);
                assert!(val1.is_some(), "[1] 应该存在");
                println!("[1] = {:?}", val1);

                // 测试 [2] - 应该返回数组 [1,2,3,4,5,6,7,8,9]
                let val2 = data.get(&[2]);
                assert!(val2.is_some(), "[2] 应该存在");
                println!("[2] = {:?}", val2);
        }

        #[test]
        fn test_nested_access() {
                let content = "[1,2,3,4,5,6,7,8,9];";
                let data = ListData::from_string(content).unwrap();

                // [0] 是整个数组, [0][1] 是数组的第1个元素(即"2")
                let val = data.get(&[0, 1]);
                if let Some(ListValue::String(s)) = val {
                        assert_eq!(s, "2", "[0][1] 应该是 '2'");
                        println!("✓ [0][1] = {}", s);
                } else {
                        panic!("[0][1] 应该返回字符串 '2', 实际: {:?}", val);
                }
        }

        #[test]
        fn test_keyvalue_array_access() {
                let content = "blue:[hello:hi,bye,light:www];";
                let data = ListData::from_string(content).unwrap();

                // [0] 应该返回 KeyValue("blue", Array([...]))
                let val0 = data.get(&[0]);
                assert!(val0.is_some(), "[0] 应该存在");

                // 尝试访问 KeyValue 内部的数组
                let val00 = data.get(&[0, 0]);
                println!("[0][0] = {:?}", val00);
        }

        #[test]
        fn test_all_commands() {
                let content = "red:[\\:,:\\],,a:b]; blue:[hello:hi,bye,light:www]; [1,2,3,4,5,6,7,8,9];";
                let mut data = ListData::from_string(content).unwrap();

                // 测试读取命令
                let result = data.execute_command("[2]");
                assert!(result.is_ok(), "[2] 命令应该成功");
                println!("[2] = {}", result.unwrap());

                // 测试追加命令 .+[n]=v
                let result = data.execute_command(".+[2]=10");
                assert!(result.is_ok(), ".+[2]=10 命令应该成功");
                println!("追加后: {}", result.unwrap());

                // 测试插入命令 .+[n]>[m]=v
                let result = data.execute_command(".+[2]>[1]=99");
                assert!(result.is_ok(), ".+[2]>[1]=99 命令应该成功");
                println!("插入后: {}", result.unwrap());

                // 测试删除命令 .->[n]
                let result = data.execute_command(".->[2]");
                assert!(result.is_ok(), ".->[2] 命令应该成功");
                println!("删除后: {}", result.unwrap());

                // 测试替换命令 .+>[n]=[arr]
                let mut data2 = ListData::from_string("[1,2,3];").unwrap();
                println!("替换前: {}", data2.to_string());
                let result = data2.execute_command(".+>[0]=[9,8,7]");
                println!("替换命令结果: {:?}", result);
                if let Ok(ref s) = result {
                        println!("替换后: {}", s);
                }
                assert!(result.is_ok(), ".+>[0]=[9,8,7] 命令应该成功");
        }

        #[test]
        fn test_variable_system() {
                let content = "[hello,world,test];";
                let mut data = ListData::from_string(content).unwrap();

                // 测试变量赋值 - 从索引读取
                let result = data.execute_command("$myvar = [0]");
                assert!(result.is_ok(), "变量赋值应该成功");
                println!("变量赋值: {}", result.unwrap());
                assert!(data.variables.contains_key("myvar"), "变量 myvar 应该存在");

                // 测试变量引用
                let result = data.execute_command("$vars");
                assert!(result.is_ok());
                println!("变量列表:\n{}", result.unwrap());

                // 测试使用变量值进行追加
                let result = data.execute_command(".+[0]=$myvar");
                assert!(result.is_ok(), "使用变量追加应该成功");
                println!("使用变量追加后: {}", result.unwrap());

                // 测试直接字符串赋值
                let result = data.execute_command("$name = \"Alice\"");
                assert!(result.is_ok());
                println!("字符串赋值: {}", result.unwrap());

                // 测试清空变量
                let result = data.execute_command("$clear");
                assert!(result.is_ok());
                println!("清空变量: {}", result.unwrap());
        }

        #[test]
        fn test_inline_expressions() {
                let content = "[apple,banana,cherry]; [1,2,3,4,5];";
                let mut data = ListData::from_string(content).unwrap();

                // 测试内联表达式 - 直接在命令中使用 ${}
                let result = data.execute_command(".+[1]=${[0][0]}");
                assert!(result.is_ok(), "内联表达式应该成功");
                println!("内联表达式追加后: {}", result.unwrap());

                // 测试嵌套内联表达式
                let result = data.execute_command(".+[1]>[0]=${[0][1]}");
                assert!(result.is_ok(), "嵌套内联表达式应该成功");
                println!("嵌套内联插入后: {}", result.unwrap());

                // 测试读取带内联表达式的值
                let result = data.execute_command("${[1]}");
                println!("内联读取结果: {:?}", result);
                if let Err(e) = &result {
                        println!("错误: {}", e);
                }
                assert!(result.is_ok());
                println!("内联读取: {}", result.unwrap());
        }

        #[test]
        fn test_pipeline_operations() {
                let content = "[start,middle,end]; [10,20,30];";
                let mut data = ListData::from_string(content).unwrap();

                // 测试简单管道 - 读取并显示
                let result = data.execute_command("[0] | [0]");
                assert!(result.is_ok(), "简单管道应该成功");
                println!("管道读取: {}", result.unwrap());

                // 测试管道用于链式操作
                let result = data.execute_command("$temp = [0][0]");
                assert!(result.is_ok());
                println!("设置临时变量: {}", result.unwrap());

                let result = data.execute_command(".+[1]=$temp");
                assert!(result.is_ok());
                println!("管道式追加后: {}", result.unwrap());
        }
}
