#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 239. 滑动窗口最大值 (Sliding Window Maximum)
// 难度：困难 | 分类：子串
// https://leetcode.cn/problems/sliding-window-maximum/
//
// 本地验证：cargo t p0239_sliding_window_maximum
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,3,-1,-3,5,3,6,7]
        // 3
        // [1]
        // 1
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::max_sliding_window(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0239_sliding_window_maximum
}
