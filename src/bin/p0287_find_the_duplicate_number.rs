#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 287. 寻找重复数 (Find the Duplicate Number)
// 难度：中等 | 分类：技巧
// https://leetcode.cn/problems/find-the-duplicate-number/
//
// 本地验证：cargo t p0287_find_the_duplicate_number
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,3,4,2,2]
        // [3,1,3,4,2]
        // [3,3,3,3,3]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::find_duplicate(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0287_find_the_duplicate_number
}
