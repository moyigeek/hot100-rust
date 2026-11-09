#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 72. 编辑距离 (Edit Distance)
// 难度：中等 | 分类：多维动态规划
// https://leetcode.cn/problems/edit-distance/
//
// 本地验证：cargo t p0072_edit_distance
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // "horse"
        // "ros"
        // "intention"
        // "execution"
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::min_distance(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0072_edit_distance
}
