// C/C++ 示例
#include <stdio.h>
#include <stdlib.h>
#include "list_lang.h"

int main() {
        // 创建.list数据
        const char* content = "red:[\\::\\],a:b]; blue:[hello:hi,bye,light:www]; [1,2,3,4,5,6,7,8,9];";
        CListData* data = list_data_from_string(content);

        if (!data) {
                printf("Failed to parse .list content\n");
                return 1;
        }

        // 读取数据
        char* result = list_data_to_string(data);
        printf("%s\n", result);
        string_free(result);

        // 读取索引0
        size_t indices[] = {0};
        result = list_data_get(data, indices, 1);
        if (result) {
                printf("[0]: %s\n", result);
                string_free(result);
        }

        // 读取索引2的子元素
        size_t indices2[] = {2, 1};
        result = list_data_get(data, indices2, 2);
        if (result) {
                printf("[2][1]: %s\n", result);
                string_free(result);
        }

        // 追加元素
        int ret = list_data_append(data, 2, "10");
        if (ret == 0) {
                result = list_data_to_string(data);
                printf("After append: %s\n", result);
                string_free(result);
        }

        // 执行命令
        result = list_data_execute_command(data, "[0]");
        if (result) {
                printf("Command [0]: %s\n", result);
                string_free(result);
        }

        // 清理
        list_data_free(data);

        return 0;
}
