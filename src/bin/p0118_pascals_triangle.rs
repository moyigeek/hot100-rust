#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 118. 杨辉三角 (Pascal's Triangle)
// 难度：简单 | 分类：动态规划
// https://leetcode.cn/problems/pascals-triangle/
//
// 本地验证：cargo t p0118_pascals_triangle
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // 5
        // 1
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::generate(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0118_pascals_triangle
}
