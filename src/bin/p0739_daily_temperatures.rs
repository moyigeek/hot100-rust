#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 739. 每日温度 (Daily Temperatures)
// 难度：中等 | 分类：栈
// https://leetcode.cn/problems/daily-temperatures/
//
// 本地验证：cargo t p0739_daily_temperatures
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [73,74,75,71,69,72,76,73]
        // [30,40,50,60]
        // [30,60,90]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::daily_temperatures(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0739_daily_temperatures
}
