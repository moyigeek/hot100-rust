#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 56. 合并区间 (Merge Intervals)
// 难度：中等 | 分类：普通数组
// https://leetcode.cn/problems/merge-intervals/
//
// 本地验证：cargo t p0056_merge_intervals
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [[1,3],[2,6],[8,10],[15,18]]
        // [[1,4],[4,5]]
        // [[4,7],[1,4]]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::merge(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0056_merge_intervals
}
