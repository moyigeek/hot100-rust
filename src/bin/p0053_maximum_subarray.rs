#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 53. 最大子数组和 (Maximum Subarray)
// 难度：中等 | 分类：普通数组
// https://leetcode.cn/problems/maximum-subarray/
//
// 本地验证：cargo t p0053_maximum_subarray
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [-2,1,-3,4,-1,2,1,-5,4]
        // [1]
        // [5,4,-1,7,8]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::max_sub_array(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0053_maximum_subarray
}
