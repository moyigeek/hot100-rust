#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 34. 在排序数组中查找元素的第一个和最后一个位置 (Find First and Last Position of Element in Sorted Array)
// 难度：中等 | 分类：二分查找
// https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/
//
// 本地验证：cargo t p0034_find_first_and_last_position_of_element_in_sorted_array
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [5,7,7,8,8,10]
        // 8
        // [5,7,7,8,8,10]
        // 6
        // []
        // 0
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::search_range(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0034_find_first_and_last_position_of_element_in_sorted_array
}
