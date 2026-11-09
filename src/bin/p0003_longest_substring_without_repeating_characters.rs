#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 3. 无重复字符的最长子串 (Longest Substring Without Repeating Characters)
// 难度：中等 | 分类：滑动窗口
// https://leetcode.cn/problems/longest-substring-without-repeating-characters/
//
// 本地验证：cargo t p0003_longest_substring_without_repeating_characters
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // "abcabcbb"
        // "bbbbb"
        // "pwwkew"
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::length_of_longest_substring(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0003_longest_substring_without_repeating_characters
}
