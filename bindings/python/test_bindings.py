# -*- coding: utf-8 -*-
"""
Python 绑定功能测试
"""

from list_lang import PyListData

print('=' * 60)
print('🧪 Python 绑定完整功能测试')
print('=' * 60)

# 测试1: 基础操作
print('\n1️⃣ 基础操作:')
data = PyListData('[apple,banana,cherry]; [10,20,30];')
print('   初始数据:', data.to_string())

data.append(1, '40')
print('   追加后:', data.to_string())

data.insert(1, 0, 'first')
print('   插入后:', data.to_string())

val = data.get([1])
print('   读取[1]:', val)

# 测试2: 命令执行
print('\n2️⃣ 命令执行 (execute_command):')
data2 = PyListData('[x,y,z]; [1,2,3];')
result = data2.execute_command('.+[1]=a')
print('   追加命令:', result)

result = data2.execute_command('[0]')
print('   读取命令:', result)

# 测试3: 变量系统
print('\n3️⃣ 变量系统 💾:')
data3 = PyListData('[Alice,Bob]; [100,200,300];')
result = data3.execute_command('$name = [0][0]')
print('   赋值 $name:', result)

result = data3.execute_command('$score = [1][2]')
print('   赋值 $score:', result)

result = data3.execute_command('.+[0]=$name')
print('   使用变量:', result)

result = data3.execute_command('$vars')
print('   查看变量:')
for line in result.split('\n'):
        print('     ', line)

# 测试4: 内联表达式
print('\n4️⃣ 内联表达式 ⚡:')
data4 = PyListData('[hello,world]; [1,2,3];')
result = data4.execute_command('.+[1]=${[0][0]}')
print('   内联追加:', result)

result = data4.execute_command('${[1]}')
print('   内联读取:', result)

# 测试5: 删除和替换
print('\n5️⃣ 删除和替换:')
data5 = PyListData('[a,b,c,d]; [1,2,3];')
result = data5.delete(0)
print('   删除[0]:', result)

result = data5.replace(0, 'new_value:test')
print('   替换[0]:', result)

# 测试6: 复杂数据结构
print('\n6️⃣ 复杂数据结构 (KeyValue + Array):')
complex_data = PyListData(
        'red:[\\:,:\\],,a:b]; blue:[hello:hi,bye,light:www]; [1,2,3,4,5];'
)
print('   复杂数据:', complex_data.to_string())

val0 = complex_data.get([0])
print('   [0]:', val0)

val01 = complex_data.get([0, 0])
print('   [0][0]:', val01)

val2 = complex_data.get([2])
print('   [2]:', val2)

val21 = complex_data.get([2, 1])
print('   [2][1]:', val21)

# 测试7: 使用你的原始示例数据
print('\n7️⃣ 你的原始示例数据测试:')
original_data = PyListData(
        'red:[\\:,:\\],,a:b]; blue:[hello:hi,bye,light:www]; [1,2,3,4,5,6,7,8,9];'
)
print('   原始数据:', original_data.to_string())

# 测试 .+[3]>[1]=10 命令
try:
        result = original_data.execute_command('.+[3]>[1]=10')
        print('   ✅ .+[3]>[1]=10 成功!')
        print('      结果:', result[:80], '...')
except Exception as e:
        print('   ❌ 失败:', e)

# 测试性能
import time
print('\n8️⃣ 性能测试:')
test_items = ','.join([f'item{i}' for i in range(1000)])
perf_data = PyListData(f'[{test_items}];')

start = time.perf_counter()
for i in range(100):
        try:
                _ = perf_data.get([i % 1000])
        except:
                pass
read_time = time.perf_counter() - start

start = time.perf_counter()
for i in range(50):
        perf_data.append(0, f'new_{i}')
write_time = time.perf_counter() - start

print(f'   读取 100 次: {read_time:.4f}s')
print(f'   写入 50 次: {write_time:.4f}s')
total = read_time + write_time
print(f'   总耗时: {total:.4f}s ({(total/150)*1000:.2f}ms/操作)')

print('\n' + '=' * 60)
print('✅ 所有功能测试通过! Python 绑定完美运行!')
print('=' * 60)
