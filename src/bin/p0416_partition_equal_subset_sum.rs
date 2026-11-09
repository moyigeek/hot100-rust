#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 416. 分割等和子集 (Partition Equal Subset Sum)
// 难度：中等 | 分类：动态规划
// https://leetcode.cn/problems/partition-equal-subset-sum/
//
// 本地验证：cargo t p0416_partition_equal_subset_sum
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,5,11,5]
        // [1,2,3,5]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::can_partition(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0416_partition_equal_subset_sum
}
