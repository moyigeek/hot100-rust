#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 300. 最长递增子序列 (Longest Increasing Subsequence)
// 难度：中等 | 分类：动态规划
// https://leetcode.cn/problems/longest-increasing-subsequence/
//
// 本地验证：cargo t p0300_longest_increasing_subsequence
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [10,9,2,5,3,7,101,18]
        // [0,1,0,3,2,3]
        // [7,7,7,7,7,7,7]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::length_of_lis(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0300_longest_increasing_subsequence
}
