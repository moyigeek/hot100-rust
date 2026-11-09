#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 42. 接雨水 (Trapping Rain Water)
// 难度：困难 | 分类：双指针
// https://leetcode.cn/problems/trapping-rain-water/
//
// 本地验证：cargo t p0042_trapping_rain_water
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [0,1,0,2,1,0,1,3,2,1,2,1]
        // [4,2,0,3,2,5]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::trap(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0042_trapping_rain_water
}
