#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 41. 缺失的第一个正数 (First Missing Positive)
// 难度：困难 | 分类：普通数组
// https://leetcode.cn/problems/first-missing-positive/
//
// 本地验证：cargo t p0041_first_missing_positive
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,2,0]
        // [3,4,-1,1]
        // [7,8,9,11,12]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::first_missing_positive(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0041_first_missing_positive
}
