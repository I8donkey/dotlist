// Node.js 示例
const { JsListData } = require('list_lang');

async function main() {
        // 创建.list数据
        const data = new JsListData('red:[\\::\\],a:b]; blue:[hello:hi,bye,light:www]; [1,2,3,4,5,6,7,8,9];');

        // 读取数据
        console.log(data.toString());
        console.log(data.get([0]));  // red:[\::\],a:b]
        console.log(data.get([2]));  // [1,2,3,4,5,6,7,8,9]
        console.log(data.get([2, 1]));  // 2

        // 修改数据
        console.log(data.append(2n, '10'));  // 追加元素
        console.log(data.insert(2n, 1n, '10'));  // 插入元素
        console.log(data.delete(2n));  // 删除元素
        console.log(data.replace(2n, '[1,2,3]'));  // 替换元素

        // 执行命令
        console.log(data.executeCommand('[0]'));  // 读取索引0
        console.log(data.executeCommand('.+[2]=100'));  // 追加到索引2
}

main().catch(console.error);
