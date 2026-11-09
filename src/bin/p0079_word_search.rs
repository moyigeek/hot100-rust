#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 79. 单词搜索 (Word Search)
// 难度：中等 | 分类：回溯
// https://leetcode.cn/problems/word-search/
//
// 本地验证：cargo t p0079_word_search
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]
        // "ABCCED"
        // [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]
        // "SEE"
        // [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]]
        // "ABCB"
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::exist(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0079_word_search
}
