#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 84. 柱状图中最大的矩形 (Largest Rectangle in Histogram)
// 难度：困难 | 分类：栈
// https://leetcode.cn/problems/largest-rectangle-in-histogram/
//
// 本地验证：cargo t p0084_largest_rectangle_in_histogram
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [2,1,5,6,2,3]
        // [2,4]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::largest_rectangle_area(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0084_largest_rectangle_in_histogram
}
