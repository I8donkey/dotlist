/**
 * .list C++ 友好接口
 * ===================
 *
 * 使用示例:
 *   #include "list_lang.hpp"
 *
 *   List l("[apple,banana,cherry]; [1,2,3,4,5];");
 *
 *   // 索引访问 (operator[])
 *   std::cout << l[0] << std::endl;              // [apple,banana,cherry]
 *   std::cout << l[0][1] << std::endl;             // banana
 *   std::cout << l.at({2, 3}) << std::endl;        // 4
 *
 *   // 切片 (类似 Python)
 *   auto slice = l.slice(1, 0, 2);                  // [apple, banana]
 *
 *   // 范围 for 循环
 *   for (const auto& item : l[1]) {
 *       std::cout << item << std::endl;            // 1, 2, 3, 4, 5
 *   }
 *
 *   // 查找
 *   auto indices = l.find("banana");                // vector<size_t>
 */

#ifndef LIST_LANG_HPP
#define LIST_LANG_HPP

#include <string>
#include <vector>
#include <memory>
#include <stdexcept>
#include <sstream>
#include <algorithm>

// FFI 声明 (需要链接 list_lang 库)
extern "C" {
        // 创建/销毁
        void* list_data_new(const char* content);
        void list_data_free(void* data);
        
        // 基本操作
        const char* list_data_to_string(void* data);
        size_t list_data_len(void* data);
        int list_data_is_empty(void* data);
        
        // 索引访问
        const char* list_data_get(void* data, const size_t* indices, size_t len);
        const char** list_data_get_array(void* data, size_t index, size_t* out_len);
        const char** list_data_get_slice(void* data, size_t index, size_t start, size_t end, size_t* out_len);
        
        // 修改
        const char* list_data_append(void* data, size_t index, const char* value);
        const char* list_data_insert(void* data, size_t index, size_t pos, const char* value);
        const char* list_data_delete(void* data, size_t index);
        const char* list_data_replace(void* data, size_t index, const char* value);
        const char* list_data_execute_command(void* data, const char* command);
        
        // 查找
        size_t* list_data_find(void* data, const char* pattern, size_t* out_len);
        size_t* list_data_find_in_array(void* data, size_t index, const char* pattern, size_t* out_len);
        
        // 文件操作
        int list_data_save_binary(void* data, const char* path);
        void* list_data_load_binary(const char* path);
}

namespace dotlist {

class ListItem;

class List {
private:
        void* _data;
        bool _owner;
        
public:
        List(const std::string& content = "") {
                _data = list_data_new(content.c_str());
                _owner = true;
        }
        
        ~List() {
                if (_owner && _data) {
                        list_data_free(_data);
                }
        }
        
        // 禁止拷贝，允许移动
        List(const List&) = delete;
        List& operator=(const List&) = delete;
        List(List&& other) noexcept : _data(other._data), _owner(other._owner) {
                other._data = nullptr;
        }
        List& operator=(List&& other) noexcept {
                if (this != &other) {
                        if (_owner && _data) list_data_free(_data);
                        _data = other._data;
                        _owner = other._owner;
                        other._data = nullptr;
                }
                return *this;
        }
        
        static List fromFile(const std::string& path) {
                // 需要自己实现文件读取
                std::ifstream file(path);
                std::stringstream buffer;
                buffer << file.rdbuf();
                return List(buffer.str());
        }
        
        static List loadBinary(const std::string& path) {
                List l;
                if (l._data) list_data_free(l._data);
                l._data = list_data_load_binary(path.c_str());
                l._owner = true;
                return l;
        }
        
        // 字符串表示
        std::string toString() const {
                return list_data_to_string(_data);
        }
        
        operator std::string() const {
                return toString();
        }
        
        // 长度
        size_t length() const { return list_data_len(_data); }
        size_t size() const { return list_data_len(_data); }
        bool empty() const { return list_data_is_empty(_data); }
        
        // 索引访问 - 返回 ListItem
        ListItem operator[](size_t index) const;
        
        // 多级索引
        ListItem at(std::initializer_list<size_t> indices) const;
        
        // 切片
        std::vector<std::string> slice(size_t index, size_t start, size_t end) const;
        
        // 执行命令
        std::string run(const std::string& command) const {
                return list_data_execute_command(_data, command.c_str());
        }
        
        // 查找
        std::vector<size_t> find(const std::string& pattern) const {
                size_t outLen = 0;
                size_t* result = list_data_find(_data, pattern.c_str(), &outLen);
                std::vector<size_t> vec(result, result + outLen);
                free(result); // 假设是 malloc 分配的
                return vec;
        }
        
        std::vector<size_t> findIn(size_t index, const std::string& pattern) const {
                size_t outLen = 0;
                size_t* result = list_data_find_in_array(_data, index, pattern.c_str(), &outLen);
                std::vector<size_t> vec(result, result + outLen);
                free(result);
                return vec;
        }
        
        // 修改操作
        void append(size_t index, const std::string& value) {
                list_data_append(_data, index, value.c_str());
        }
        
        void insert(size_t index, size_t pos, const std::string& value) {
                list_data_insert(_data, index, pos, value.c_str());
        }
        
        void remove(size_t index) {
                list_data_delete(_data, index);
        }
        
        void replace(size_t index, const std::string& value) {
                list_data_replace(_data, index, value.c_str());
        }
        
        // 获取原始数据指针 (用于高级用法)
        void* raw() const { return _data; }
};

class ListItem {
private:
        const List* _list;
        std::vector<size_t> _indices;
        mutable std::string _cachedValue;
        
public:
        ListItem(const List* list, std::vector<size_t> indices)
                : _list(list), _indices(std::move(indices)) {}
        
        // 获取值
        std::string value() const {
                if (_cachedValue.empty()) {
                        _cachedValue = list_data_get(_list->raw(), _indices.data(), _indices.size());
                }
                return _cachedValue;
        }
        
        operator std::string() const { return value(); }
        
        // 嵌套索引
        ListItem operator[](size_t index) const {
                auto newIndices = _indices;
                newIndices.push_back(index);
                return ListItem(_list, newIndices);
        }
        
        // 切片
        std::vector<std::string> slice(size_t start, size_t end) const {
                size_t outLen = 0;
                const char** result = list_data_get_slice(
                        _list->raw(),
                        _indices[_indices.size() - 1],
                        start,
                        end,
                        &outLen
                );
                
                std::vector<std::string> vec;
                for (size_t i = 0; i < outLen; i++) {
                        vec.emplace_back(result[i]);
                }
                free(result);
                return vec;
        }
        
        // 获取数组元素列表
        std::vector<std::string> array() const {
                size_t outLen = 0;
                const char** result = list_data_get_array(
                        _list->raw(),
                        _indices[_indices.size() - 1],
                        &outLen
                );
                
                std::vector<std::string> vec;
                for (size_t i = 0; i < outLen; i++) {
                        vec.emplace_back(result[i]);
                }
                free(result);
                return vec;
        }
        
        // 迭代器支持 (简化版)
        class Iterator {
        private:
                const ListItem* _item;
                size_t _pos;
                std::vector<std::string> _array;
        public:
                Iterator(const ListItem* item, size_t pos)
                        : _item(item), _pos(pos), _array(item->array()) {}
                
                std::string operator*() const {
                        return _pos < _array.size() ? _array[_pos] : "";
                }
                
                Iterator& operator++() { ++_pos; return *this; }
                bool operator!=(const Iterator& other) const { return _pos != other._pos; }
        };
        
        Iterator begin() const { return Iterator(this, 0); }
        Iterator end() const { 
                auto arr = array();
                return Iterator(this, arr.size()); 
        }
        
        size_t length() const { return array().size(); }
};

inline ListItem List::operator[](size_t index) const {
        return ListItem(this, {index});
}

inline ListItem List::at(std::initializer_list<size_t> indices) const {
        return ListItem(this, indices);
}

} // namespace list_lang

#endif // LIST_LANG_HPP
