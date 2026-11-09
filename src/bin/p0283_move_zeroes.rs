#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 283. 移动零 (Move Zeroes)
// 难度：简单 | 分类：双指针
// https://leetcode.cn/problems/move-zeroes/
//
// 本地验证：cargo t p0283_move_zeroes
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [0,1,0,3,12]
        // [0]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::move_zeroes(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0283_move_zeroes
}
