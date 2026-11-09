#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 347. 前 K 个高频元素 (Top K Frequent Elements)
// 难度：中等 | 分类：堆
// https://leetcode.cn/problems/top-k-frequent-elements/
//
// 本地验证：cargo t p0347_top_k_frequent_elements
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,1,1,2,2,3]
        // 2
        // [1]
        // 1
        // [1,2,1,2,1,2,3,1,3,2]
        // 2
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::top_k_frequent(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0347_top_k_frequent_elements
}
