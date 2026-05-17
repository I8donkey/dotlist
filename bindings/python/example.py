# Python 示例
from list_lang import PyListData

# 创建.list数据
data = PyListData("red:[\\::\\],a:b]; blue:[hello:hi,bye,light:www]; [1,2,3,4,5,6,7,8,9];")

# 读取数据
print(data.to_string())
print(data.get([0]))  # red:[\::\],a:b]
print(data.get([2]))  # [1,2,3,4,5,6,7,8,9]
print(data.get([2, 1]))  # 2

# 修改数据
print(data.append(2, "10"))  # 追加元素
print(data.insert(2, 1, "10"))  # 插入元素
print(data.delete(2))  # 删除元素
print(data.replace(2, "[1,2,3]"))  # 替换元素

# 执行命令
print(data.execute_command("[0]"))  # 读取索引0
print(data.execute_command(".+[2]=100"))  # 追加到索引2
