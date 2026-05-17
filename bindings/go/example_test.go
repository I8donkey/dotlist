// Go 示例
package main

import (
        "fmt"
        "list_lang"
)

func main() {
        // 创建.list数据
        content := `red:[\::\],a:b]; blue:[hello:hi,bye,light:www]; [1,2,3,4,5,6,7,8,9];`
        data, err := list_lang.NewListDataFromString(content)
        if err != nil {
                fmt.Println("Error:", err)
                return
        }
        defer data.Free()

        // 读取数据
        fmt.Println(data.ToString())

        result, err := data.Get([]uint{0})
        if err != nil {
                fmt.Println("Error:", err)
        } else {
                fmt.Println("[0]:", result)
        }

        result, err = data.Get([]uint{2, 1})
        if err != nil {
                fmt.Println("Error:", err)
        } else {
                fmt.Println("[2][1]:", result)
        }

        // 追加元素
        err = data.Append(2, "10")
        if err != nil {
                fmt.Println("Error:", err)
        } else {
                fmt.Println("After append:", data.ToString())
        }

        // 执行命令
        result, err = data.ExecuteCommand("[0]")
        if err != nil {
                fmt.Println("Error:", err)
        } else {
                fmt.Println("Command [0]:", result)
        }
}
