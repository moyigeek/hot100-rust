#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 128. 最长连续序列 (Longest Consecutive Sequence)
// 难度：中等 | 分类：哈希
// https://leetcode.cn/problems/longest-consecutive-sequence/
//
// 本地验证：cargo t p0128_longest_consecutive_sequence
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [100,4,200,1,3,2]
        // [0,3,7,2,5,8,4,6,0,1]
        // [1,0,1,2]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::longest_consecutive(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0128_longest_consecutive_sequence
}
