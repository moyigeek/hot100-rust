#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 153. 寻找旋转排序数组中的最小值 (Find Minimum in Rotated Sorted Array)
// 难度：中等 | 分类：二分查找
// https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array/
//
// 本地验证：cargo t p0153_find_minimum_in_rotated_sorted_array
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [3,4,5,1,2]
        // [4,5,6,7,0,1,2]
        // [11,13,15,17]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::find_min(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0153_find_minimum_in_rotated_sorted_array
}
