use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use std::ptr;
use crate::parser::{ListData, ListValue, value_to_string};

#[repr(C)]
pub struct CListData {
        data: ListData,
}

#[no_mangle]
pub extern "C" fn list_data_new() -> *mut CListData {
        Box::into_raw(Box::new(CListData { data: ListData::new() }))
}

#[no_mangle]
pub extern "C" fn list_data_from_string(content: *const c_char) -> *mut CListData {
        let c_str = unsafe { CStr::from_ptr(content) };
        let content_str = match c_str.to_str() {
                Ok(s) => s,
                Err(_) => return ptr::null_mut(),
        };

        let data = match ListData::from_string(content_str) {
                Ok(d) => d,
                Err(_) => return ptr::null_mut(),
        };

        Box::into_raw(Box::new(CListData { data }))
}

#[no_mangle]
pub extern "C" fn list_data_free(ptr: *mut CListData) {
        if !ptr.is_null() {
                unsafe { drop(Box::from_raw(ptr)); }
        }
}

#[no_mangle]
pub extern "C" fn list_data_to_string(ptr: *const CListData) -> *mut c_char {
        if ptr.is_null() { return ptr::null_mut(); }

        let data = unsafe { &(*ptr).data };
        let result = data.to_string();

        match CString::new(result) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => ptr::null_mut(),
        }
}

#[no_mangle]
pub extern "C" fn list_data_len(ptr: *const CListData) -> usize {
        if ptr.is_null() { return 0; }
        let data = unsafe { &(*ptr).data };
        data.len()
}

#[no_mangle]
pub extern "C" fn list_data_is_empty(ptr: *const CListData) -> c_int {
        if ptr.is_null() { return 1; }
        let data = unsafe { &(*ptr).data };
        if data.is_empty() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn list_data_get(
        ptr: *const CListData,
        indices: *const usize,
        indices_len: usize
) -> *mut c_char {
        if ptr.is_null() || indices.is_null() || indices_len == 0 {
                return ptr::null_mut();
        }

        let data = unsafe { &(*ptr).data };
        let idx_slice = unsafe { std::slice::from_raw_parts(indices, indices_len) };

        match data.get(idx_slice) {
                Some(val) => value_to_cstring(val),
                None => ptr::null_mut(),
        }
}

#[no_mangle]
pub extern "C" fn list_data_get_array(
        ptr: *const CListData,
        index: usize,
        out_len: *mut usize
) -> *mut *mut c_char {
        if ptr.is_null() || out_len.is_null() {
                return ptr::null_mut();
        }

        let data = unsafe { &(*ptr).data };
        match data.get_array(index) {
                Some(arr) => {
                        unsafe { *out_len = arr.len(); }
                        let mut result: Vec<*mut c_char> = Vec::new();
                        for s in arr {
                                match CString::new(s) {
                                        Ok(cs) => result.push(cs.into_raw()),
                                        Err(_) => result.push(ptr::null_mut()),
                                }
                        }
                        let ptrs = result.as_mut_ptr();
                        std::mem::forget(result);
                        ptrs
                },
                None => ptr::null_mut(),
        }
}

#[no_mangle]
pub extern "C" fn list_data_get_slice(
        ptr: *const CListData,
        index: usize,
        start: usize,
        end: usize,
        out_len: *mut usize
) -> *mut *mut c_char {
        if ptr.is_null() || out_len.is_null() {
                return ptr::null_mut();
        }

        let data = unsafe { &(*ptr).data };
        match data.get_slice(index, start, end) {
                Some(arr) => {
                        unsafe { *out_len = arr.len(); }
                        let mut result: Vec<*mut c_char> = Vec::new();
                        for s in arr {
                                match CString::new(s) {
                                        Ok(cs) => result.push(cs.into_raw()),
                                        Err(_) => result.push(ptr::null_mut()),
                                }
                        }
                        let ptrs = result.as_mut_ptr();
                        std::mem::forget(result);
                        ptrs
                },
                None => ptr::null_mut(),
        }
}

#[no_mangle]
pub extern "C" fn list_data_find(
        ptr: *const CListData,
        pattern: *const c_char,
        out_len: *mut usize
) -> *mut usize {
        if ptr.is_null() || pattern.is_null() || out_len.is_null() {
                return ptr::null_mut();
        }

        let data = unsafe { &(*ptr).data };
        let pat_str = unsafe { CStr::from_ptr(pattern).to_str().unwrap_or("") };
        let indices = data.find(pat_str);

        unsafe { *out_len = indices.len(); }
        let mut result = indices;
        let ptrs = result.as_mut_ptr();
        std::mem::forget(result);
        ptrs
}

#[no_mangle]
pub extern "C" fn list_data_find_in_array(
        ptr: *const CListData,
        index: usize,
        pattern: *const c_char,
        out_len: *mut usize
) -> *mut usize {
        if ptr.is_null() || pattern.is_null() || out_len.is_null() {
                return ptr::null_mut();
        }

        let data = unsafe { &(*ptr).data };
        let pat_str = unsafe { CStr::from_ptr(pattern).to_str().unwrap_or("") };
        match data.find_in_array(index, pat_str) {
                Some(indices) => {
                        unsafe { *out_len = indices.len(); }
                        let mut result = indices;
                        let ptrs = result.as_mut_ptr();
                        std::mem::forget(result);
                        ptrs
                },
                None => ptr::null_mut(),
        }
}

#[no_mangle]
pub extern "C" fn list_data_append(
        ptr: *mut CListData,
        index: usize,
        value: *const c_char
) -> c_int {
        if ptr.is_null() || value.is_null() { return -1; }

        let data = unsafe { &mut (*ptr).data };
        let val_str = unsafe { CStr::from_ptr(value) };
        let val = match val_str.to_str() {
                Ok(s) => parse_simple_value_c(s),
                Err(_) => return -1,
        };

        match data.append(index, val) {
                Ok(()) => 0,
                Err(_) => -1,
        }
}

#[no_mangle]
pub extern "C" fn list_data_insert(
        ptr: *mut CListData,
        index: usize,
        position: usize,
        value: *const c_char
) -> c_int {
        if ptr.is_null() || value.is_null() { return -1; }

        let data = unsafe { &mut (*ptr).data };
        let val_str = unsafe { CStr::from_ptr(value) };
        let val = match val_str.to_str() {
                Ok(s) => parse_simple_value_c(s),
                Err(_) => return -1,
        };

        match data.insert(index, position, val) {
                Ok(()) => 0,
                Err(_) => -1,
        }
}

#[no_mangle]
pub extern "C" fn list_data_delete(ptr: *mut CListData, index: usize) -> c_int {
        if ptr.is_null() { return -1; }

        let data = unsafe { &mut (*ptr).data };
        match data.delete(index) {
                Ok(()) => 0,
                Err(_) => -1,
        }
}

#[no_mangle]
pub extern "C" fn list_data_replace(
        ptr: *mut CListData,
        index: usize,
        new_value: *const c_char
) -> c_int {
        if ptr.is_null() || new_value.is_null() { return -1; }

        let data = unsafe { &mut (*ptr).data };
        let new_val_str = unsafe { CStr::from_ptr(new_value) };
        let new_val_str_match = match new_val_str.to_str() {
                Ok(s) => s,
                Err(_) => return -1,
        };

        let val = if new_val_str_match.starts_with('[') && new_val_str_match.ends_with(']') {
                let inner = &new_val_str_match[1..new_val_str_match.len()-1];
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
                ListValue::String(new_val_str_match.to_string())
        };

        match data.replace(index, val) {
                Ok(()) => 0,
                Err(_) => -1,
        }
}

#[no_mangle]
pub extern "C" fn list_data_execute_command(
        ptr: *mut CListData,
        command: *const c_char
) -> *mut c_char {
        if ptr.is_null() || command.is_null() { return ptr::null_mut(); }

        let data = unsafe { &mut (*ptr).data };
        let cmd_str = unsafe { CStr::from_ptr(command) };
        let cmd = match cmd_str.to_str() {
                Ok(s) => s,
                Err(_) => return ptr::null_mut(),
        };

        match data.execute_command(cmd) {
                Ok(result) => match CString::new(result) {
                        Ok(c_string) => c_string.into_raw(),
                        Err(_) => ptr::null_mut(),
                },
                Err(_) => ptr::null_mut(),
        }
}

#[no_mangle]
pub extern "C" fn list_data_save_binary(ptr: *const CListData, path: *const c_char) -> c_int {
        if ptr.is_null() || path.is_null() { return -1; }
        let data = unsafe { &(*ptr).data };
        let path_str = unsafe { CStr::from_ptr(path).to_str().unwrap_or("") };
        match data.save_binary(path_str) {
                Ok(()) => 0,
                Err(_) => -1,
        }
}

#[no_mangle]
pub extern "C" fn list_data_load_binary(path: *const c_char) -> *mut CListData {
        if path.is_null() { return ptr::null_mut(); }
        let path_str = unsafe { CStr::from_ptr(path).to_str().unwrap_or("") };
        match ListData::load_binary(path_str) {
                Ok(data) => Box::into_raw(Box::new(CListData { data })),
                Err(_) => ptr::null_mut(),
        }
}

#[no_mangle]
pub extern "C" fn string_free(ptr: *mut c_char) {
        if !ptr.is_null() {
                unsafe { drop(CString::from_raw(ptr)); }
        }
}

#[no_mangle]
pub extern "C" fn string_array_free(ptr: *mut *mut c_char, len: usize) {
        if !ptr.is_null() {
                unsafe {
                        let slice = std::slice::from_raw_parts_mut(ptr, len);
                        for s_ptr in slice {
                                if !s_ptr.is_null() {
                                        drop(CString::from_raw(*s_ptr));
                                }
                        }
                        drop(Box::from_raw(ptr));
                }
        }
}

#[no_mangle]
pub extern "C" fn usize_array_free(ptr: *mut usize, _len: usize) {
        if !ptr.is_null() {
                unsafe { drop(Box::from_raw(ptr)); }
        }
}

fn value_to_cstring(val: &ListValue) -> *mut c_char {
        let result = value_to_string(val);
        match CString::new(result) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => ptr::null_mut(),
        }
}

fn parse_simple_value_c(s: &str) -> ListValue {
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
