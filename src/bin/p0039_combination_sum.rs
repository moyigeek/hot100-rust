#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 39. 组合总和 (Combination Sum)
// 难度：中等 | 分类：回溯
// https://leetcode.cn/problems/combination-sum/
//
// 本地验证：cargo t p0039_combination_sum
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [2,3,6,7]
        // 7
        // [2,3,5]
        // 8
        // [2]
        // 1
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::combination_sum(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0039_combination_sum
}
