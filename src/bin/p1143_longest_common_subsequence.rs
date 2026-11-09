#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 1143. 最长公共子序列 (Longest Common Subsequence)
// 难度：中等 | 分类：多维动态规划
// https://leetcode.cn/problems/longest-common-subsequence/
//
// 本地验证：cargo t p1143_longest_common_subsequence
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // "abcde"
        // "ace"
        // "abc"
        // "abc"
        // "abc"
        // "def"
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::longest_common_subsequence(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p1143_longest_common_subsequence
}
