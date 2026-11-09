#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 139. 单词拆分 (Word Break)
// 难度：中等 | 分类：动态规划
// https://leetcode.cn/problems/word-break/
//
// 本地验证：cargo t p0139_word_break
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // "leetcode"
        // ["leet","code"]
        // "applepenapple"
        // ["apple","pen"]
        // "catsandog"
        // ["cats","dog","sand","and","cat"]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::word_break(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0139_word_break
}
