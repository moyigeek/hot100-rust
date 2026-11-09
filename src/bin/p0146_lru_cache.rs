#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 146. LRU 缓存 (LRU Cache)
// 难度：中等 | 分类：链表
// https://leetcode.cn/problems/lru-cache/
//
// 本地验证：cargo t p0146_lru_cache
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

struct LRUCache {}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        todo!()
    }

    fn get(&self, key: i32) -> i32 {
        todo!()
    }

    fn put(&self, key: i32, value: i32) {
        todo!()
    }
}

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // ["LRUCache","put","put","get","put","get","put","get","get","get"]
        // [[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]
        //
        // 注意：官方模板方法接收 &self；需要修改字段时改成 &mut self。
        // let mut obj = LRUCache::new(/* 参数 */);
    }
}

fn main() {
    // 调试入口：cargo r p0146_lru_cache
}
