#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 1. 两数之和 (Two Sum)
// 难度：简单 | 分类：哈希
// https://leetcode.cn/problems/two-sum/
//
// 本地验证：cargo t p0001_two_sum
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [2,7,11,15]
        // 9
        // [3,2,4]
        // 6
        // [3,3]
        // 6
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::two_sum(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0001_two_sum
}
