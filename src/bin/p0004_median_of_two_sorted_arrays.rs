#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 4. 寻找两个正序数组的中位数 (Median of Two Sorted Arrays)
// 难度：困难 | 分类：二分查找
// https://leetcode.cn/problems/median-of-two-sorted-arrays/
//
// 本地验证：cargo t p0004_median_of_two_sorted_arrays
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,3]
        // [2]
        // [1,2]
        // [3,4]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::find_median_sorted_arrays(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0004_median_of_two_sorted_arrays
}
