#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 78. 子集 (Subsets)
// 难度：中等 | 分类：回溯
// https://leetcode.cn/problems/subsets/
//
// 本地验证：cargo t p0078_subsets
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,2,3]
        // [0]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::subsets(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0078_subsets
}
