#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 560. 和为 K 的子数组 (Subarray Sum Equals K)
// 难度：中等 | 分类：子串
// https://leetcode.cn/problems/subarray-sum-equals-k/
//
// 本地验证：cargo t p0560_subarray_sum_equals_k
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,1,1]
        // 2
        // [1,2,3]
        // 3
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::subarray_sum(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0560_subarray_sum_equals_k
}
