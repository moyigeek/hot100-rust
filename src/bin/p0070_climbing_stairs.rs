#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 70. 爬楼梯 (Climbing Stairs)
// 难度：简单 | 分类：动态规划
// https://leetcode.cn/problems/climbing-stairs/
//
// 本地验证：cargo t p0070_climbing_stairs
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // 2
        // 3
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::climb_stairs(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0070_climbing_stairs
}
