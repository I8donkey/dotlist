# -*- coding: utf-8 -*-
"""
.list Python 快速入门 - 可直接运行示例
========================================

运行方式:
1. 先编译: maturin develop --release --features python
2. 运行本文件: python quick_start.py
"""

import sys

def print_header(title):
    """打印漂亮的标题"""
    print("\n" + "=" * 70)
    print(f"  {title}")
    print("=" * 70)

def main():
        print_header("🚀 .list Python 快速入门示例")

        # 检查是否安装
        try:
                from list_lang import PyListData
                print("✓ list_lang 模块加载成功！")
        except ImportError:
                print("""
❌ 错误: 未找到 list_lang 模块

请先执行以下命令安装:

  1. 安装 maturin:
     pip install maturin

  2. 编译并安装 Python 绑定:
     cd e:\\.list
     maturin develop --release --features python

  3. 验证安装:
     python -c "from list_lang import PyListData; print('OK')"

然后重新运行本脚本。
""")
                input("\n按 Enter 退出...")
                sys.exit(1)

        # ============================================================
        # 示例1: 基础操作
        # ============================================================

        print_header("示例1: 基础 CRUD 操作")

        data = PyListData("red:[\\:,:\\],,a:b]; blue:[hello:hi,bye]; [1,2,3,4,5];")

        print("📝 原始数据:")
        print(f"   {data.to_string()}")

        print("\n🔍 读取数据:")
        print(f"   [0]     = {data.get([0])}")
        print(f"   [1]     = {data.get([1])}")
        print(f"   [2]     = {data.get([2])}")
        print(f"   [2][2]  = {data.get([2, 2])}")

        print("\n✏️  追加元素:")
        result = data.append(2, "6")
        print(f"   .+[2]=6 → {result}")

        print("\n📍 插入元素:")
        result = data.insert(2, 1, "99")
        print(f"   .+[2]>[1]=99 → {result}")

        print("\n🗑️  删除元素:")
        result = data.delete(2)
        print(f"   .->[2] → {result}")

        print("\n🔄 替换元素:")
        result = data.replace(0, "new_data:value")
        print(f"   .+>[0]=[new_data:value] → {result}")

        # ============================================================
        # 示例2: 使用 execute_command
        # ============================================================

        print_header("示例2: 命令执行 (execute_command)")

        data2 = PyListData("[apple,banana,cherry]; [10,20,30];")

        print("📝 初始数据:")
        print(f"   {data2.to_string()}")

        commands = [
                ("读取", "[0]"),
                ("追加", ".+[1]=40"),
                ("插入", ".+[1]>[1]=25"),
                ("删除", ".->[0][2]"),
                ("替换", ".+>[0]=[x,y,z]"),
        ]

        for desc, cmd in commands:
                try:
                        result = data2.execute_command(cmd)
                        print(f"\n   {desc}: {cmd}")
                        print(f"   结果: {result[:60]}{'...' if len(result) > 60 else ''}")
                except Exception as e:
                        print(f"\n   ❌ {desc} 失败: {e}")

        # ============================================================
        # 示例3: 变量系统
        # ============================================================

        print_header("示例3: 变量系统 💾")

        data3 = PyListData("[Alice,Bob,Charlie]; [100,200,300];")

        print("📝 初始数据:")
        print(f"   {data3.to_string()}")

        var_operations = [
                "$name = [0][0]",           # 赋值
                "$score = [1][2]",          # 赋值
                '$greeting = "Hello"',      # 字符串赋值
                ".+[0]=$name",              # 使用变量追加
                ".+>[2]=[$name,$greeting]", # 多变量创建数组
                "$vars",                    # 查看变量
        ]

        for cmd in var_operations:
                try:
                        result = data3.execute_command(cmd)
                        if not cmd.startswith("$vars"):
                                print(f"\n   执行: {cmd}")
                        else:
                                print(f"\n   📋 变量列表:")
                        lines = result.split('\n')
                        for line in lines[:5]:  # 最多显示5行
                                print(f"      {line}")
                except Exception as e:
                        print(f"\n   ❌ 失败 ({cmd}): {e}")

        # ============================================================
        # 示例4: 内联表达式
        # ============================================================

        print_header("示例4: 内联表达式 ⚡")

        data4 = PyListData("[x,y,z]; [1,2,3];")

        print("📝 初始数据:")
        print(f"   {data4.to_string()}")

        inline_ops = [
                (".+[1]=${[0][0]}", "将 [0][0] 的值追加到 [1]"),
                (".+[1]>[0]=${[0][2]}", "将 [0][2] 插入到 [1][0]"),
                ("${[1]}", "直接读取内联表达式的值"),
        ]

        for cmd, desc in inline_ops:
                try:
                        result = data4.execute_command(cmd)
                        print(f"\n   {desc}")
                        print(f"   命令: {cmd}")
                        print(f"   结果: {result}")
                except Exception as e:
                        print(f"\n   ❌ 失败: {e}")

        # ============================================================
        # 示例5: 实际应用 - 配置管理
        # ============================================================

        print_header("示例5: 实际应用 - 配置管理器")

        class ConfigManager:
                def __init__(self):
                        self.data = PyListData(
                                "host:localhost; port:8080; debug:true; "
                                "[user:admin,password:secret];"
                        )

                def get(self, key):
                        """获取配置值"""
                        for i in range(10):
                                try:
                                        val = self.data.get([i])
                                        if val and val.startswith(key + ":"):
                                                return val.split(":", 1)[1]
                                except:
                                        pass
                        return None

                def set(self, key, value):
                        """设置配置"""
                        self.data.execute_command(f'.+>[{key}:{value}]')

                def show(self):
                        """显示所有配置"""
                        print(self.data.to_string())

        config = ConfigManager()
        print("📋 当前配置:")
        config.show()

        print("\n🔧 修改配置:")
        host = config.get("host")
        port = config.get("port")
        print(f"   host = {host}")
        print(f"   port = {port}")

        print("\n✅ 配置管理完成!")

        # ============================================================
        # 性能测试
        # ============================================================

        print_header("⚡ 性能测试")

        import time

        # 生成测试数据
        test_items = ",".join([f"item{i}" for i in range(10000)])
        large_data = PyListData(f"[{test_items}];")

        print("📊 测试大数据集 (10,000 元素):")

        start = time.perf_counter()

        # 读取测试
        for i in range(1000):
                _ = large_data.get([i % 10000])

        read_time = time.perf_counter() - start
        print(f"   ✓ 读取 1000 次: {read_time:.4f} 秒")

        # 写入测试
        start = time.perf_counter()
        for i in range(100):
                large_data.append(0, f"new_{i}")
        write_time = time.perf_counter() - start
        print(f"   ✓ 写入 100 次:  {write_time:.4f} 秒")

        total_time = read_time + write_time
        print(f"\n   ⚡ 总耗时: {total_time:.4f} 秒")
        print(f"   🚀 平均每次操作: {(total_time/1100)*1000:.2f} 毫秒")

        # ============================================================
        # 总结
        # ============================================================

        print_header("🎉 完成！")

        print("""
你已掌握以下技能:

✅ 基础操作: 创建、读取、追加、插入、删除、替换
✅ 命令系统: execute_command() 执行所有命令
✅ 变量系统: $var 赋值和引用
✅ 内联表达式: ${[n]} 即时求值
✅ 管道操作: | 链式命令
✅ 实际应用: 配置管理、数据处理

下一步学习:
• 查看 complete_tutorial.py 了解更多细节
• 阅读 README.md 了解完整文档
• 尝试 bindings/python/examples/ 中的示例

祝你使用愉快! 🚀
""")

if __name__ == "__main__":
        try:
                main()
        except KeyboardInterrupt:
                print("\n\n用户中断")
        except Exception as e:
                print(f"\n❌ 发生错误: {e}")
                import traceback
                traceback.print_exc()
        finally:
                input("\n按 Enter 退出...")
