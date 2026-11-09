#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 189. 轮转数组 (Rotate Array)
// 难度：中等 | 分类：普通数组
// https://leetcode.cn/problems/rotate-array/
//
// 本地验证：cargo t p0189_rotate_array
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,2,3,4,5,6,7]
        // 3
        // [-1,-100,3,99]
        // 2
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::rotate(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0189_rotate_array
}
