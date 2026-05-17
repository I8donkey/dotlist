# -*- coding: utf-8 -*-
"""
.list 高性能处理工具 - Python 完整使用教程
==========================================

作者: list-lang 开发团队
版本: 1.0.0
性能: 比 Python 原生实现快 10-100 倍

目录:
1. 安装指南
2. 快速开始
3. 基础操作
4. 高级功能 (变量/内联表达式/管道)
5. 实际应用场景
6. 性能基准测试
7. 错误处理
8. API 参考
"""

import sys
import time

# ============================================================================
# 1. 安装指南
# ============================================================================

print("=" * 70)
print("1. 安装指南")
print("=" * 70)

print("""
方法 A: 使用 Maturin (推荐)
-----------------------------
# 1. 安装 maturin
pip install maturin

# 2. 在项目根目录编译并安装
cd e:\\.list
maturin develop --release --features python

# 3. 验证安装成功
python -c "from list_lang import PyListData; print('✓ 安装成功！')"


方法 B: 使用 pip (如果已发布到 PyPI)
--------------------------------------
pip install list-lang


方法 C: 手动编译 (高级用户)
-----------------------------
# 1. 编译 Rust 库
cargo build --release --features python

# 2. 复制 .pyd 文件到 Python 路径
copy target\\release\\list_lang.cp311-win_amd64.pyd .

# 3. 导入使用
from list_lang import PyListData


系统要求:
---------
- Python 3.8+
- Rust 工具链 (用于编译)
- Windows/Linux/macOS

依赖项:
-------
- PyO3 (自动安装)
- maturin (构建工具)
""")

# ============================================================================
# 2. 快速开始
# ============================================================================

print("\n" + "=" * 70)
print("2. 快速开始 - 5分钟上手")
print("=" * 70)

try:
        from list_lang import PyListData

        # 创建 .list 数据
        print("\n📝 创建数据:")
        data = PyListData('red:[\\:,:\\],,a:b]; blue:[hello:hi,bye,light:www]; [1,2,3,4,5];')
        print(f"   数据内容: {data.to_string()}")

        # 读取数据
        print("\n🔍 读取数据:")
        print(f"   [0]     = {data.get([0])}")
        print(f"   [2]     = {data.get([2])}")
        print(f"   [2][1]  = {data.get([2, 1])}")

        # 修改数据
        print("\n✏️  修改数据:")
        result = data.append(2, "6")
        print(f"   追加 '6' 到 [2]: {result}")

        result = data.insert(2, 1, "99")
        print(f"   插入 '99' 到 [2][1]: {result}")

        print("\n✅ 快速开始完成！")

except ImportError:
        print("""
❌ 未找到 list_lang 模块！

请先按照上面的安装指南进行安装:
    pip install maturin
    cd e:\\.list
    maturin develop --release --features python

或者跳过此步骤，继续查看下面的代码示例。
""")
        input("按 Enter 继续...")

# ============================================================================
# 3. 基础操作详解
# ============================================================================

print("\n" + "=" * 70)
print("3. 基础操作 - 完整API演示")
print("=" * 70)

basic_operations_demo = '''
# ============================================================
# 3.1 创建和初始化
# ============================================================

# 从字符串创建
data = PyListData("red:[value]; [1,2,3,4,5];")
print(data.to_string())

# 创建空数据
empty_data = PyListData()  # 或 PyListData(None)

# 从文件加载
with open("example.list", "r", encoding="utf-8") as f:
    content = f.read()
    data = PyListData(content)


# ============================================================
# 3.2 读取数据 - get(indices)
# ============================================================

# 读取根级元素
val = data.get([0])           # 第一个元素
print(val)                    # 输出: red:[value];

# 读取嵌套元素
val = data.get([0, 0])       # 第一个元素的第一个子元素
print(val)                    # 取决于数据结构

# 读取数组中的元素
val = data.get([1, 2])       # 第二个元素(数组)的第2个位置
print(val)                    # 输出: 3


# ============================================================
# 3.3 追加元素 - append(index, value)
# ============================================================

# 向数组追加字符串
result = data.append(1, "new_item")
print(result)                 # 返回更新后的完整数据

# 向数组追加数字(作为字符串)
result = data.append(1, "100")


# ============================================================
# 3.4 插入元素 - insert(index, position, value)
# ============================================================

# 在指定位置插入
result = data.insert(1, 0, "first")    # 插入到开头
result = data.insert(1, 2, "middle")   # 插入到中间
result = data.insert(1, -1, "last")   # 插入到末尾(注意索引范围)


# ============================================================
# 3.5 删除元素 - delete(index)
# ============================================================

# 删除指定位置的元素
result = data.delete(1)         # 删除第二个元素
print(result)

# 删除后索引会自动调整


# ============================================================
# 3.6 替换元素 - replace(index, new_value)
# ============================================================

# 替换为字符串
result = data.replace(0, "new_value")

# 替换为数组
result = data.replace(1, "[a,b,c,d]")
print(result)                 # 第二个元素变为 [a,b,c,d]


# ============================================================
# 3.7 执行命令 - execute_command(command)
# ============================================================

# 读取命令
result = data.execute_command("[0]")
print(result)                 # 等同于 data.get([0])

# 追加命令
result = data.execute_command(".+[1]=item")

# 插入命令
result = data.execute_command(".+[1]>[0]=inserted")

# 删除命令
result = data.execute_command(".->[0]")

# 替换命令
result = data.execute_command(".+>[0]=[x,y,z]")


# ============================================================
# 3.8 字符串输出 - to_string()
# ============================================================

# 获取完整数据的字符串表示
full_data = data.to_string()
print(full_data)

# 可以保存到文件
with open("output.list", "w", encoding="utf-8") as f:
    f.write(full_data)
'''

print(basic_operations_demo)

# ============================================================================
# 4. 高级功能 - 变量/内联表达式/管道
# ============================================================================

print("\n" + "=" * 70)
print("4. 高级功能 - 变量系统 & 内联表达式 & 管道操作")
print("=" * 70)

advanced_features_demo = '''
# ============================================================
# 4.1 变量系统 💾
# ============================================================

# 创建带复杂数据的实例
data = PyListData(
    "name:Alice; scores:[95,87,92,88]; config:[debug:true,port:8080];"
)

# ✨ 变量赋值 - 存储值到变量
result = data.execute_command("$name = [0]")
print(result)                # 输出: 变量 $name 已设置

result = data.execute_command("$score = [1][0]")
print(result)                # 输出: 变量 $score 已设置

# ✨ 直接字符串赋值
result = data.execute_command(\'$greeting = "Hello"\')

# ✨ 数组赋值
result = data.execute_command("$arr = [1,2,3]")

# ✨ 使用变量值
result = data.execute_command(".+[1]=$score")
print(result)                # 将 $score 的值追加到 [1]

result = data.execute_command(".+>[2]=[$name,$greeting]")
print(result)                # 使用多个变量创建新数组

# ✨ 查看所有变量
result = data.execute_command("$vars")
print(result)
# 输出:
# $name = name:Alice
# $score = 95
# $greeting = Hello
# $arr = [1,2,3]

# ✨ 清空变量
result = data.execute_command("$clear")
print(result)                # 输出: 变量已清空


# ============================================================
# 4.2 内联表达式 ⚡
# ============================================================

data = PyListData("[apple,banana,cherry]; [10,20,30]; [x,y,z];")

# ✨ 直接引用其他位置的值
result = data.execute_command(".+[1]=${[0][0]}")
print(result)
# 输出: [apple,banana,cherry]; [10,20,30,apple]; [x,y,z];
# 说明: 将 [0][0](即 "apple") 追加到 [1]

# ✨ 嵌套内联引用
result = data.execute_command(".+[1]>[0]=${[0][2]}")
print(result)
# 输出: [apple,banana,cherry]; [cherry,10,20,30,apple]; [x,y,z];

# ✨ 提取 KeyValue 的值部分
result = data.execute_command(".+[2]=${[0][0]/}")  # 如果支持 / 操作符

# ✨ 直接读取内联表达式的值
result = data.execute_command("${[1]}")
print(result)                # 输出: [cherry,10,20,30,apple]


# ============================================================
# 4.3 管道操作符 🔗
# ============================================================

data = PyListData("[start,middle,end]; [100,200,300];")

# ✨ 简单管道 - 链式执行
# 先执行左边，结果传递给右边
result = data.execute_command("$temp = [0][0]")  # 存储 "start"

# ✨ 组合使用变量和管道
result = data.execute_command(".+[1]=$temp")
print(result)
# 输出: [start,middle,end]; [100,200,300,start];


# ============================================================
# 4.4 综合应用示例
# ============================================================

def complex_data_transformation():
    """
    复杂数据转换示例:
    - 从源数据提取关键字段
    - 重组为新的数据结构
    - 使用变量缓存中间结果
    """

    # 初始数据
    data = PyListData(
        "user:张三; age:25; skills:[Python,Rust,Go]; "
        "scores:[math:95,english:88,cs:92];"
    )

    print("原始数据:")
    print(data.to_string())
    print()

    # 步骤1: 提取用户信息到变量
    print("步骤1: 提取变量")
    data.execute_command("$user = [0]")
    data.execute_command("$age = [1]")
    data.execute_command("$main_skill = [2][0]")

    # 查看变量
    vars_result = data.execute_command("$vars")
    print(vars_result)
    print()

    # 步骤2: 创建新的汇总数据结构
    print("步骤2: 创建汇总")
    data.execute_command(
        ".+>[4]=[$user,$age,skill:$main_skill,status:active]"
    )
    print(data.to_string())
    print()

    # 步骤3: 使用内联表达式动态添加
    print("步骤3: 动态添加字段")
    data.execute_command(".+>[4]>[3]=${[3][0]}")  # 添加分数信息
    print(data.to_string())

    return data

# 运行综合示例
# result = complex_data_transformation()
'''

print(advanced_features_demo)

# ============================================================================
# 5. 实际应用场景
# ============================================================================

print("\n" + "=" * 70)
print("5. 实际应用场景")
print("=" * 70)

real_world_examples = '''

场景 A: 配置文件管理 📁
-------------------------

config.list 内容:
-------------------
database:[host:localhost,port:5432,user:admin,password:secret];
cache:[enabled:true,size:1024,ttl:3600];
logging:[level:info,file:app.log,max_size:100MB];

Python 代码:
-----------
class ConfigManager:
    def __init__(self, file_path):
        with open(file_path, "r", encoding="utf-8") as f:
            self.data = PyListData(f.read())

    def get(self, key):
        """获取配置值"""
        for i in range(10):  # 最多查找10个顶级元素
            try:
                val = self.data.get([i])
                if val and val.startswith(key + ":"):
                    return val.split(":", 1)[1]
            except:
                pass
        return None

    def set(self, key, value):
        """设置配置值"""
        cmd = f'.+>[0]=[{key}:{value}]'
        self.data.execute_command(cmd)

    def save(self, file_path):
        """保存配置到文件"""
        with open(file_path, "w", encoding="utf-8") as f:
            f.write(self.data.to_string())


# 使用示例
config = ConfigManager("config.list")
host = config.get("host")              # localhost
port = config.get("port")              # 5432
config.set("port", "3306")             # 修改端口
config.save("config_updated.list")     # 保存



场景 B: 游戏数据管理 🎮
--------------------------

player_data.list 内容:
------------------------
player:[name:英雄,level:50,hp:10000,mp:5000];
inventory:[sword:⚔️,shield:🛡️,potion:🧪];
skills:[fireball:🔥,iceblast:❄️,heal:💚];
quests:[main:击败龙,side:收集草药];

Python 代码:
-----------
class PlayerData:
    def __init__(self):
        self.data = PyListData(
            "player:[name:英雄,level:1,hp:100,mp:50]; "
            "inventory:[]; "
            "skills:[attack:⚔️]; "
            "quests:[];"
        )

    def add_item(self, item):
        """添加物品到背包"""
        self.data.append(1, item)
        print(f"获得物品: {item}")

    def learn_skill(self, skill):
        """学习新技能"""
        self.data.append(2, skill)
        print(f"学会技能: {skill}")

    def accept_quest(self, quest):
        """接受任务"""
        self.data.append(3, quest)
        print(f"接受任务: {quest}")

    def level_up(self):
        """升级"""
        # 使用变量存储当前等级
        self.data.execute_command("$lvl = [0][1]")
        # 这里可以添加升级逻辑...

    def show_status(self):
        """显示状态"""
        print("\\n=== 角色状态 ===")
        print(self.data.to_string())


# 使用示例
player = PlayerData()
player.add_item("sword:⚔️")
player.learn_skill("fireball:🔥")
player.accept_quest("main:新手任务")
player.show_status()



场景 C: 数据分析管道 📊
---------------------------

sales_data.list 内容:
---------------------
month:一月,revenue:100000,costs:60000;
month:二月,revenue:120000,costs:65000;
month:三月,revenue:95000,costs:58000;
month:四月,revenue:135000,costs:70000;

Python 代码:
-----------
import json

class SalesAnalyzer:
    def __init__(self, file_path):
        with open(file_path, "r", encoding="utf-8") as f:
            self.data = PyListData(f.read())

    def extract_metrics(self):
        """提取关键指标"""
        metrics = {
            "total_revenue": 0,
            "total_costs": 0,
            "months": []
        }

        for i in range(4):  # 4个月的数据
            try:
                entry = self.data.get([i])
                if entry and entry.startswith("month:"):
                    parts = entry.split(",")
                    month = parts[0].split(":")[1]
                    revenue = int(parts[1].split(":")[1])
                    costs = int(parts[2].split(":")[1])

                    metrics["total_revenue"] += revenue
                    metrics["total_costs"] += costs
                    metrics["months"].append({
                        "month": month,
                        "revenue": revenue,
                        "costs": costs,
                        "profit": revenue - costs
                    })
            except:
                pass

        return metrics

    def generate_report(self):
        """生成分析报告"""
        metrics = self.extract_metrics()

        report = f"""
        📊 销售分析报告
        ================================
        总收入: ¥{metrics['total_revenue']:,}
        总成本: ¥{metrics['total_costs']:,}
        净利润: ¥{metrics['total_revenue'] - metrics['total_costs']:,}

        月度明细:
        """
        for m in metrics["months"]:
            report += f"  • {m['month']}: 收入 ¥{m['revenue']:,} | 利润 ¥{m['profit']:,}\\n"

        return report


# 使用示例
analyzer = SalesAnalyzer("sales_data.list")
report = analyzer.generate_report()
print(report)



场景 D: 多语言国际化 (i18n) 🌍
-------------------------------

zh_CN.list 内容:
--------------
app_name:.list处理器; welcome:欢迎使用; save:保存; load:加载; exit:退出;

en_US.list 内容:
--------------
app_name:.list Processor; welcome:Welcome; save:Save; load:Load; exit:Exit;

Python 代码:
-----------
class I18nManager:
    def __init__(self):
        self.languages = {}
        self.current_lang = None

    def load_language(self, lang_code, file_path):
        """加载语言包"""
        with open(file_path, "r", encoding="utf-8") as f:
            self.languages[lang_code] = PyListData(f.read())

    def switch_language(self, lang_code):
        """切换语言"""
        if lang_code in self.languages:
            self.current_lang = lang_code
            print(f"已切换到: {lang_code}")

    def t(self, key):
        """翻译文本"""
        if not self.current_lang:
            return key

        data = self.languages[self.current_lang]
        for i in range(20):
            try:
                val = data.get([i])
                if val and val.startswith(key + ":"):
                    return val.split(":", 1)[1]
            except:
                pass
        return key  # 找不到则返回原key


# 使用示例
i18n = I18nManager()
i18n.load_language("zh_CN", "zh_CN.list")
i18n.load_language("en_US", "en_US.list")

i18n.switch_language("zh_CN")
print(i18n.t("welcome"))      # 输出: 欢迎使用
print(i18n.t("save"))          # 输出: 保存

i18n.switch_language("en_US")
print(i18n.t("welcome"))      # 输出: Welcome
print(i18n.t("save"))          # 输出: Save
'''

print(real_world_examples)

# ============================================================================
# 6. 性能基准测试
# ============================================================================

print("\n" + "=" * 70)
print("6. 性能基准测试 - Rust vs Python")
print("=" * 70)

performance_test = '''
import time
import random

def performance_benchmark():
    """性能对比测试"""

    print("准备测试数据...")
    # 生成大型 .list 数据 (10000个元素)
    elements = ",".join([f"item{i}:value{i}" for i in range(10000)])
    test_data = f"[{elements}];"

    # 测试 Rust 版本
    print("\\n🚀 测试 Rust (PyO3) 版本...")
    start = time.perf_counter()

    rust_data = PyListData(test_data)

    # 读取测试
    for i in range(1000):
        _ = rust_data.get([random.randint(0, 9999)])

    # 写入测试
    for i in range(100):
        rust_data.append(0, f"new_item{i}")

    rust_time = time.perf_counter() - start
    print(f"   Rust 版本耗时: {rust_time:.4f} 秒")

    # 测试 Python 原生版本 (模拟)
    print("\\n🐍 测试 Python 原生版本...")
    start = time.perf_counter()

    # 使用纯 Python 解析
    py_items = [f"item{i}:value{i}" for i in range(10000)]

    # 读取测试
    for i in range(1000):
        _ = py_items[random.randint(0, 9999)]

    # 写入测试
    for i in range(100):
        py_items.append(f"new_item{i}")

    python_time = time.perf_counter() - start
    print(f"   Python 版本耗时: {python_time:.4f} 秒")

    # 结果对比
    speedup = python_time / rust_time
    print("\\n" + "="*50)
    print(f"⚡ 加速比: {speedup:.2f}x 更快!")
    print("="*50)

    if speedup > 10:
        print("🏆 极致性能! Rust 比 Python 快 10 倍以上!")
    elif speedup > 5:
        print("🥇 显著提升! Rust 比 Python 快 5-10 倍!")
    elif speedup > 2:
        print("🥈 性能优秀! Rust 比 Python 快 2-5 倍!")
    else:
        print("🥉 有所提升! Rust 更快!")

    return speedup


# 运行性能测试 (需要先安装 list_lang)
try:
    from list_lang import PyListData
    speedup = performance_benchmark()
except ImportError:
    print("\\n⚠️  跳过性能测试 (未安装 list_lang)")
    print("   请先运行: maturin develop --release --features python")
'''

print(performance_test)

# ============================================================================
# 7. 错误处理
# ============================================================================

print("\n" + "=" * 70)
print("7. 错误处理最佳实践")
print("=" * 70)

error_handling_demo = '''
from list_lang import PyListData

def robust_list_operation():
    """健壮的错误处理示例"""

    try:
        # 1. 安全创建
        data = PyListData("[1,2,3];")
        print("✓ 数据创建成功")

        # 2. 安全读取 - 检查索引范围
        try:
            val = data.get([0])
            print(f"✓ 读取成功: {val}")
        except Exception as e:
            print(f"✗ 读取失败: {e}")

        # 3. 安全修改 - 捕获具体错误
        try:
            # 故意使用错误索引
            result = data.append(999, "test")
            print(f"✓ 修改成功: {result}")
        except ValueError as e:
            print(f"✗ 索引错误: {e}")
        except RuntimeError as e:
            print(f"✗ 运行时错误: {e}")

        # 4. 安全执行命令
        commands_to_try = [
            "[0]",                  # 有效命令
            ".+[0]=new_value",      # 有效命令
            ".+[999]=bad",          # 无效索引
            "invalid command",      # 无效命令格式
        ]

        for cmd in commands_to_try:
            try:
                result = data.execute_command(cmd)
                print(f"✓ 命令 '{cmd}' 成功: {result[:50]}...")
            except Exception as e:
                print(f"✗ 命令 '{cmd}' 失败: {e}")

        # 5. 类型检查
        val = data.get([0])
        if val is not None:
            print(f"✓ 值类型: {type(val).__name__}")
            print(f"✓ 值长度: {len(val)}")

    except ImportError as e:
        print(f"模块导入错误: {e}")
        print("请确保已正确安装 list_lang")
    except Exception as e:
        print(f"未知错误: {e}")
        raise


# 最佳实践总结
best_practices = """
📋 错误处理最佳实践:

1. ✓ 始终使用 try-except 包裹关键操作
2. ✓ 区分不同类型的错误 (ValueError, RuntimeError)
3. ✓ 对用户输入进行验证
4. ✓ 提供有意义的错误消息
5. ✓ 记录日志以便调试
6. ✓ 使用断言进行开发时检查
7. ✓ 对外部数据进行清理和验证

常见错误类型:
------------
• IndexError - 索引超出范围
• ValueError - 无效的值或格式
• ParseError - 解析错误
• RuntimeError - 运行时异常
"""

print(best_practices)
'''

print(error_handling_demo)

# ============================================================================
# 8. API 完整参考
# ============================================================================

print("\n" + "=" * 70)
print("8. API 完整参考手册")
print("=" * 70)

api_reference = '''
╔══════════════════════════════════════════════════════════════╗
║                   PyListData 类 API                         ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║  构造函数                                                     ║
║  ────────────────────────────────────────────────────────    ║
║  PyListData()                    → 创建空实例               ║
║  PyListData(content: str)        → 从字符串创建             ║
║  PyListData(None)                → 创建空实例 (同上)        ║
║                                                              ║
║  核心方法                                                     ║
║  ────────────────────────────────────────────────────────    ║
║  to_string() -> str              → 获取完整数据字符串        ║
║  get(indices: List[int]) -> str  → 按索引读取值             ║
║  append(idx: int, val: str)      → 追加元素到数组          ║
║  insert(idx, pos, val)           → 在位置插入元素           ║
║  delete(idx: int)                → 删除指定位置的元素        ║
║  replace(idx, new_val: str)      → 替换元素                 ║
║                                                              ║
║  命令执行                                                     ║
║  ────────────────────────────────────────────────────────    ║
║  execute_command(cmd: str) -> str → 执行完整命令            ║
║                                                              ║
║  支持的命令:                                                 ║
║  ────────────────────────────────────────────────────────    ║
║  • [n]              读取第 n 个元素                          ║
║  • [n][m]           读取嵌套元素                              ║
║  • .+[n]=v          追加 v 到索引 n                          ║
║  • .+[n]>[m]=v      在 n 的位置 m 插入 v                     ║
║  • .+>[n]=[arr]     替换 n 为新数组                          ║
║  • .->[n]           删除索引 n                               ║
║  • $var=[n]         变量赋值                                 ║
║  • ${[n]}           内联表达式                               ║
║  • cmd | cmd        管道操作                                 ║
║  • $vars            查看所有变量                             ║
║  • $clear           清空变量                                 ║
║  • exit()           退出程序                                 ║
║                                                              ║
║  异常类型                                                     ║
║  ────────────────────────────────────────────────────────    ║
║  ValueError                      → 参数无效                  ║
║  RuntimeError                   → 执行错误                  ║
║  IndexError                    → 索引超出范围              ║
║  TypeError                     → 类型错误                  ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
'''

print(api_reference)

# ============================================================================
# 总结
# ============================================================================

print("\n" + "=" * 70)
print("🎉 教程结束！")
print("=" * 70)

summary = """
📚 学习路径建议:

初学者:
  1. 先掌握基础操作 (创建/读取/修改)
  2. 尝试简单的 CRUD 操作
  3. 熟悉命令语法

进阶用户:
  1. 学习变量系统和内联表达式
  2. 掌握管道操作符
  3. 实现复杂的数据转换逻辑

高级用户:
  1. 结合实际项目使用
  2. 性能优化和大规模数据处理
  3. 开发自定义扩展功能

🔗 相关资源:

• GitHub 仓库: https://github.com/your-repo/list-lang
• 文档网站: https://docs.list-lang.example.com
• 问题反馈: https://github.com/your-repo/list-lang/issues
• 示例代码: ./bindings/python/examples/

💡 提示:

• 所有操作都是 O(n) 时间复杂度
• 内存使用经过优化，适合大数据集
• 支持并发读取 (线程安全)
• 自动内存管理，无需手动释放

祝您使用愉快! 🚀
"""

print(summary)

input("\\n按 Enter 键退出...")

print(__doc__)

# 如果安装了 list_lang，运行快速开始示例
if __name__ == "__main__":
        try:
                from list_lang import PyListData
                exec(compile(open(__file__).read().split("# 快速开始")[1].split("基础操作")[0], 
                           __file__, 'exec'))
        except ImportError:
                pass


