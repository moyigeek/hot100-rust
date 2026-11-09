#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 208. 实现 Trie (前缀树) (Implement Trie (Prefix Tree))
// 难度：中等 | 分类：图论
// https://leetcode.cn/problems/implement-trie-prefix-tree/
//
// 本地验证：cargo t p0208_implement_trie_prefix_tree
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

struct Trie {}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        todo!()
    }

    fn insert(&self, word: String) {
        todo!()
    }

    fn search(&self, word: String) -> bool {
        todo!()
    }

    fn starts_with(&self, prefix: String) -> bool {
        todo!()
    }
}

/*
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // ["Trie","insert","search","search","startsWith","insert","search"]
        // [[],["apple"],["apple"],["app"],["app"],["app"],["app"]]
        //
        // 注意：官方模板方法接收 &self；需要修改字段时改成 &mut self。
        // let mut obj = Trie::new(/* 参数 */);
    }
}

fn main() {
    // 调试入口：cargo r p0208_implement_trie_prefix_tree
}
