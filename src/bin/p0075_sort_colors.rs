#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 75. 颜色分类 (Sort Colors)
// 难度：中等 | 分类：技巧
// https://leetcode.cn/problems/sort-colors/
//
// 本地验证：cargo t p0075_sort_colors
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [2,0,2,1,1,0]
        // [2,0,1]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::sort_colors(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0075_sort_colors
}
