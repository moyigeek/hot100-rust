#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 32. 最长有效括号 (Longest Valid Parentheses)
// 难度：困难 | 分类：动态规划
// https://leetcode.cn/problems/longest-valid-parentheses/
//
// 本地验证：cargo t p0032_longest_valid_parentheses
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // "(()"
        // ")()())"
        // ""
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::longest_valid_parentheses(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0032_longest_valid_parentheses
}
