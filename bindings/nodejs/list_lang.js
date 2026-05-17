/**
 * .list JavaScript/Node.js 友好接口
 * ====================================
 *
 * 使用示例:
 *   const { List } = require('list-lang');
 *
 *   const l = new List('[apple,banana,cherry]; [1,2,3,4,5];');
 *
 *   // 索引访问
 *   console.log(l.get(0));              // [apple,banana,cherry]
 *   console.log(l.get(0).get(1));       // banana
 *   console.log(l.at([2, 3]));          // 4
 *
 *   // 切片
 *   console.log(l.slice(1, 0, 2));      // [apple, banana]
 *
 *   // 迭代
 *   for (const item of l.get(1)) {
 *       console.log(item);             // 1, 2, 3, 4, 5
 *   }
 *
 *   // 查找
 *   const indices = l.find('banana');   // [1]
 */

class ListItem {
        constructor(data, indices = []) {
                this._data = data;
                this._indices = [...indices];
                this._cachedValue = null;
        }

        get value() {
                if (!this._cachedValue) {
                        this._cachedValue = this._data.get(this._indices);
                }
                return this._cachedValue;
        }

        toString() {
                return this.value;
        }

        [Symbol.iterator]() {
                // 尝试作为数组迭代
                try {
                        const arr = this._data.getArray(this._indices[this._indices.length - 1] || 0);
                        if (arr) {
                                let i = 0;
                                return {
                                        next: () => {
                                                if (i < arr.length) {
                                                        return { done: false, value: new ListItem(this._data, [...this._indices, i++]) };
                                                }
                                                return { done: true };
                                        }
                                };
                        }
                } catch (e) {}

                // 作为字符串迭代
                const str = this.value;
                let i = 0;
                return {
                        next: () => {
                                if (i < str.length) {
                                        return { done: false, value: str[i++] };
                                }
                                return { done: true };
                        }
                };
        }

        get(index) {
                const newIndices = [...this._indices];
                if (index < 0) {
                        // 负数索引处理
                        try {
                                const len = this.length;
                                index = len + index;
                        } catch (e) {
                                throw new RangeError(`负数索引 ${index} 无效`);
                        }
                }
                newIndices.push(index);
                return new ListItem(this._data, newIndices);
        }

        slice(start, end) {
                try {
                        return this._data.getSlice(
                                this._indices[this._indices.length - 1] || 0,
                                start,
                                end
                        ) || [];
                } catch (e) {
                        throw new Error(`切片失败: ${e.message}`);
                }
        }

        set(index, value) {
                const indices = [...this._indices];
                if (index < 0) {
                        try {
                                const len = this.length;
                                index = len + index;
                        } catch (e) {
                                throw new RangeError(`负数索引无效`);
                        }
                }
                indices.push(index);
                this._data.run(`.+${formatIndices(indices)}=${value}`);
                this._cachedValue = null; // 清除缓存
        }

        get length() {
                try {
                        const arr = this._data.getArray(this._indices[this._indices.length - 1] || 0);
                        return arr ? arr.length : this.value.length;
                } catch (e) {
                        return this.value.length;
                }
        }

        toJSON() {
                return this.value;
        }
}

class List {
        constructor(content = '') {
                if (typeof require !== 'undefined') {
                        // Node.js 环境
                        const PyListData = require('list_lang').PyListData;
                        this._data = content ? new PyListData(content) : new PyListData();
                } else {
                        throw new Error('请在 Node.js 环境中使用');
                }
        }

        static fromFile(path) {
                const fs = require('fs');
                const content = fs.readFileSync(path, 'utf-8');
                return new List(content);
        }

        static loadBinary(path) {
                const instance = new List();
                instance._data = require('list_lang').PyListData.loadBinary(path);
                return instance;
        }

        toString() {
                return this._data.toString();
        }

        save(path) {
                const fs = require('fs');
                fs.writeFileSync(path, this.toString(), 'utf-8');
        }

        saveBinary(path) {
                this._data.saveBinary(path);
        }

        exportText(path) {
                this._data.exportText(path);
        }

        get length() {
                return this._data.len();
        }

        get isEmpty() {
                return this._data.isEmpty();
        }

        [Symbol.iterator]() {
                let i = 0;
                const self = this;
                return {
                        next() {
                                if (i < self.length) {
                                        return { done: false, value: self.get(i++) };
                                }
                                return { done: true };
                        }
                };
        }

        get(index) {
                if (index < 0) {
                        index = this.length + index;
                }
                if (index < 0 || index >= this.length) {
                        throw new RangeError(`索引 ${index} 超出范围 (长度 ${this.length})`);
                }
                return new ListItem(this._data, [index]);
        }

        at(indices) {
                // 支持嵌套索引: l.at([0, 1, 2])
                return new ListItem(this._data, indices);
        }

        slice(start, end) {
                start = start || 0;
                end = end === undefined ? this.length : end;

                const result = [];
                for (let i = start; i < end && i < this.length; i++) {
                        result.push(this.get(i));
                }
                return result;
        }

        set(index, value) {
                if (index < 0) {
                        index = this.length + index;
                }
                this._data.run(`.+>[${index}]=${value}`);
        }

        run(command) {
                return this._data.executeCommand(command);
        }

        find(pattern) {
                return this._data.find(pattern);
        }

        findIn(index, pattern) {
                return this._data.findInArray(index, pattern);
        }

        append(index, value) {
                this._data.append(index, value);
        }

        insert(index, position, value) {
                this._data.insert(index, position, value);
        }

        delete(index) {
                this._data.delete(index);
        }

        replace(index, value) {
                this._data.replace(index, value);
        }

        getRaw(indices) {
                return this._data.get(indices);
        }

        toArray() {
                const result = [];
                for (let i = 0; i < this.length; i++) {
                        result.push(this.get(i));
                }
                return result;
        }

        forEach(callback) {
                for (let i = 0; i < this.length; i++) {
                        callback(this.get(i), i, this);
                }
        }

        map(callback) {
                const result = [];
                for (let i = 0; i < this.length; i++) {
                        result.push(callback(this.get(i), i, this));
                }
                return result;
        }

        filter(predicate) {
                const result = [];
                for (let i = 0; i < this.length; i++) {
                        const item = this.get(i);
                        if (predicate(item, i, this)) {
                                result.push(item);
                        }
                }
                return result;
        }

        reduce(callback, initialValue) {
                let accumulator = initialValue;
                for (let i = 0; i < this.length; i++) {
                        accumulator = callback(accumulator, this.get(i), i, this);
                }
                return accumulator;
        }
}

function formatIndices(indices) {
        return indices.map(i => `[${i}]`).join('');
}

// 导出
if (typeof module !== 'undefined' && module.exports) {
        module.exports = { List, ListItem };
}
