#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 215. 数组中的第K个最大元素 (Kth Largest Element in an Array)
// 难度：中等 | 分类：堆
// https://leetcode.cn/problems/kth-largest-element-in-an-array/
//
// 本地验证：cargo t p0215_kth_largest_element_in_an_array
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [3,2,1,5,6,4]
        // 2
        // [3,2,3,1,2,4,5,5,6]
        // 4
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::find_kth_largest(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0215_kth_largest_element_in_an_array
}
