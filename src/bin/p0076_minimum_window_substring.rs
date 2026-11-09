#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 76. 最小覆盖子串 (Minimum Window Substring)
// 难度：困难 | 分类：子串
// https://leetcode.cn/problems/minimum-window-substring/
//
// 本地验证：cargo t p0076_minimum_window_substring
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // "ADOBECODEBANC"
        // "ABC"
        // "a"
        // "a"
        // "a"
        // "aa"
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::min_window(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0076_minimum_window_substring
}
