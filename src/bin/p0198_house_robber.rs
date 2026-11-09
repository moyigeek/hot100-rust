#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 198. 打家劫舍 (House Robber)
// 难度：中等 | 分类：动态规划
// https://leetcode.cn/problems/house-robber/
//
// 本地验证：cargo t p0198_house_robber
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,2,3,1]
        // [2,7,9,3,1]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::rob(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0198_house_robber
}
