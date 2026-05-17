// .list Go 友好接口
// ===================
//
// 使用示例:
//
//      package main
//
//      import (
//              "fmt"
//              "list-lang"
//      )
//
//      func main() {
//              l := list_lang.New("[apple,banana,cherry]; [1,2,3,4,5];")
//
//              // 索引访问
//              fmt.Println(l.Get(0))             // [apple,banana,cherry]
//              fmt.Println(l.Get(0).Get(1))      // banana
//              fmt.Println(l.At([]int{2, 3}))    // 4
//
//              // 切片
//              slice, _ := l.Slice(1, 0, 2)       // [apple banana]
//              fmt.Println(slice)
//
//              // 迭代 (range)
//              for i, item := range l.Get(1).Array() {
//                      fmt.Println(i, item)       // 0 1, 1 2, ...
//              }
//
//              // 查找
//              indices := l.Find("banana")        // [1]
//              fmt.Println(indices)
//      }
//

package listlang

import (
        "errors"
        "fmt"
        "strings"
)

/*
#include "list_lang_ffi.h"
#cgo LDFLAGS: -llist_lang -L${SRCDIR}/../../target/release
*/
import "C"

// ListItem 表示 .list 中的一个元素（可以是字符串、数组或键值对）
type ListItem struct {
        data    *C.void
        indices []int
}

// Value 获取元素的字符串值
func (item *ListItem) Value() string {
        if item.data == nil || len(item.indices) == 0 {
                return ""
        }

        cIndices := make([]C.size_t, len(item.indices))
        for i, idx := range item.indices {
                cIndices[i] = C.size_t(idx)
        }

        cResult := C.list_data_get(item.data, &cIndices[0], C.size_t(len(cIndices)))
        defer C.free(unsafe.Pointer(cResult))

        return C.GoString(cResult)
}

func (item *ListItem) String() string {
        return item.Value()
}

// Get 嵌套索引访问
func (item *ListItem) Get(index int) *ListItem {
        newIndices := make([]int, len(item.indices)+1)
        copy(newIndices, item.indices)
        newIndices[len(newIndices)-1] = index

        return &ListItem{
                data:    item.data,
                indices: newIndices,
        }
}

// Slice 切片操作
func (item *ListItem) Slice(start, end int) ([]string, error) {
        if len(item.indices) == 0 {
                return nil, errors.New("无效的索引")
        }

        var outLen C.size_t
        lastIndex := item.indices[len(item.indices)-1]

        cResult := C.list_data_get_slice(
                item.data,
                C.size_t(lastIndex),
                C.size_t(start),
                C.size_t(end),
                &outLen,
        )
        defer C.free(unsafe.Pointer(cResult))

        if cResult == nil {
                return nil, errors.New("切片失败")
        }

        result := make([]string, outLen)
        slice := (*[1 << 28]*C.char)(cResult)[:outLen:outLen]

        for i := 0; i < int(outLen); i++ {
                result[i] = C.GoString(slice[i])
        }

        return result, nil
}

// Array 获取所有子元素作为数组
func (item *ListItem) Array() ([]string, error) {
        if len(item.indices) == 0 {
                return nil, errors.New("无效的索引")
        }

        var outLen C.size_t
        lastIndex := item.indices[len(item.indices)-1]

        cResult := C.list_data_get_array(
                item.data,
                C.size_t(lastIndex),
                &outLen,
        )
        defer C.free(unsafe.Pointer(cResult))

        if cResult == nil {
                return nil, errors.New("获取数组失败")
        }

        result := make([]string, outLen)
        slice := (*[1 << 28]*C.char)(cResult)[:outLen:outLen]

        for i := 0; i < int(outLen); i++ {
                result[i] = C.GoString(slice[i])
        }

        return result, nil
}

// Len 获取长度
func (item *ListItem) Len() (int, error) {
        arr, err := item.Array()
        if err != nil {
                return len(item.Value()), nil
        }
        return len(arr), nil
}

// List 表示完整的 .list 数据结构
type List struct {
        data *C.void
}

// New 创建新的 List 实例
func New(content string) *List {
        cContent := C.CString(content)
        defer C.free(unsafe.Pointer(cContent))

        data := C.list_data_new(cContent)
        return &List{data: data}
}

// NewEmpty 创建空的 List
func NewEmpty() *List {
        data := C.list_data_new(nil)
        return &List{data: data}
}

// FromFile 从文件加载
func FromFile(path string) (*List, error) {
        content, err := os.ReadFile(path)
        if err != nil {
                return nil, fmt.Errorf("读取文件失败: %w", err)
        }
        return New(string(content)), nil
}

// LoadBinary 加载二进制文件
func LoadBinary(path string) (*List, error) {
        cPath := C.CString(path)
        defer C.free(unsafe.Pointer(cPath))

        data := C.list_data_load_binary(cPath)
        if data == nil {
                return nil, errors.New("加载二进制文件失败")
        }

        return &List{data: data}, nil
}

// Close 释放资源
func (l *List) Close() {
        if l.data != nil {
                C.list_data_free(l.data)
                l.data = nil
        }
}

// String 字符串表示
func (l *List) String() string {
        if l.data == nil {
                return ""
        }
        cResult := C.list_data_to_string(l.data)
        defer C.free(unsafe.Pointer(cResult))
        return C.GoString(cResult)
}

// Len 长度
func (l *List) Len() int {
        return int(C.list_data_len(l.data))
}

// IsEmpty 是否为空
func (l *List) IsEmpty() bool {
        return C.list_data_is_empty(l.data) != 0
}

// Get 索引访问，返回 ListItem
func (l *List) Get(index int) *ListItem {
        if index < 0 {
                index = l.Len() + index
        }
        if index < 0 || index >= l.Len() {
                panic(fmt.Sprintf("索引 %d 超出范围 (长度 %d)", index, l.Len()))
        }
        return &ListItem{data: l.data, indices: []int{index}}
}

// At 多级索引访问
func (l *List) At(indices []int) *ListItem {
        return &ListItem{data: l.data, indices: indices}
}

// Slice 顶层切片
func (l *List) Slice(start, end int) ([]string, error) {
        result := make([]string, 0)
        for i := start; i < end && i < l.Len(); i++ {
                item := l.Get(i)
                result = append(result, item.Value())
        }
        return result, nil
}

// Run 执行命令
func (l *List) Run(command string) (string, error) {
        cCmd := C.CString(command)
        defer C.free(unsafe.Pointer(cCmd))

        cResult := C.list_data_execute_command(l.data, cCmd)
        if cResult == nil {
                return "", errors.New("命令执行失败")
        }

        result := C.GoString(cResult)
        C.free(unsafe.Pointer(cResult))
        return result, nil
}

// Find 查找元素
func (l *List) Find(pattern string) []int {
        cPattern := C.CString(pattern)
        defer C.free(unsafe.Pointer(cPattern))

        var outLen C.size_t
        cResult := C.list_data_find(l.data, cPattern, &outLen)
        defer C.free(unsafe.Pointer(cResult))

        if cResult == nil {
                return []int{}
        }

        indices := (*[1 << 28]C.size_t)(cResult)[:outLen:outLen]
        result := make([]int, outLen)
        for i := 0; i < int(outLen); i++ {
                result[i] = int(indices[i])
        }

        return result
}

// FindIn 在指定数组中查找
func (l *List) FindIn(index int, pattern string) ([]int, error) {
        cPattern := C.CString(pattern)
        defer C.free(unsafe.Pointer(cPattern))

        var outLen C.size_t
        cResult := C.list_data_find_in_array(l.data, C.size_t(index), cPattern, &outLen)
        defer C.free(unsafe.Pointer(cResult))

        if cResult == nil {
                return nil, errors.New("查找失败")
        }

        indices := (*[1 << 28]C.size_t)(cResult)[:outLen:outLen]
        result := make([]int, outLen)
        for i := 0; i < int(outLen); i++ {
                result[i] = int(indices[i])
        }

        return result, nil
}

// Append 追加元素
func (l *List) Append(index int, value string) error {
        cValue := C.CString(value)
        defer C.free(unsafe.Pointer(cValue))

        cResult := C.list_data_append(l.data, C.size_t(index), cValue)
        if cResult != nil {
                C.free(unsafe.Pointer(cResult))
        }
        return nil
}

// Insert 插入元素
func (l *List) Insert(index, position int, value string) error {
        cValue := C.CString(value)
        defer C.free(unsafe.Pointer(cValue))

        cResult := C.list_data_insert(l.data, C.size_t(index), C.size_t(position), cValue)
        if cResult != nil {
                C.free(unsafe.Pointer(cResult))
        }
        return nil
}

// Delete 删除元素
func (l *List) Delete(index int) error {
        cResult := C.list_data_delete(l.data, C.size_t(index))
        if cResult != nil {
                C.free(unsafe.Pointer(cResult))
        }
        return nil
}

// Replace 替换元素
func (l *List) Replace(index int, value string) error {
        cValue := C.CString(value)
        defer C.free(unsafe.Pointer(cValue))

        cResult := C.list_data_replace(l.data, C.size_t(index), cValue)
        if cResult != nil {
                C.free(unsafe.Pointer(cResult))
        }
        return nil
}

// Save 保存为文本文件
func (l *List) Save(path string) error {
        return os.WriteFile(path, []byte(l.String()), 0644)
}

// SaveBinary 保存为二进制文件
func (l *List) SaveBinary(path string) error {
        cPath := C.CString(path)
        defer C.free(unsafe.Pointer(cPath))

        result := C.list_data_save_binary(l.data, cPath)
        if result != 0 {
                return errors.New("保存二进制文件失败")
        }
        return nil
}

// ExportText 导出为文本
func (l *List) ExportText(path string) error {
        cPath := C.CString(path)
        defer C.free(unsafe.Pointer(cPath))

        // 需要实现 list_data_export_text 或使用 Save
        return l.Save(path)
}

// ToArray 转换为 Go 数组
func (l *List) ToArray() []*ListItem {
        items := make([]*ListItem, l.Len())
        for i := 0; i < l.Len(); i++ {
                items[i] = l.Get(i)
        }
        return items
}

// ForEach 遍历每个元素
func (l *List) ForEach(fn func(*ListItem, int)) {
        for i := 0; i < l.Len(); i++ {
                fn(l.Get(i), i)
        }
}

// Map 映射转换
func (l *List) Map(fn func(*ListItem, int) string) []string {
        results := make([]string, l.Len())
        for i := 0; i < l.Len(); i++ {
                results[i] = fn(l.Get(i), i)
        }
        return results
}

// Filter 过滤
func (l *List) Filter(fn func(*ListItem, int) bool) []*ListItem {
        var result []*ListItem
        for i := 0; i < l.Len(); i++ {
                item := l.Get(i)
                if fn(item, i) {
                        result = append(result, item)
                }
        }
        return result
}
