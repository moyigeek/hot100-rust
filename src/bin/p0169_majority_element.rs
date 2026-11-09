#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 169. 多数元素 (Majority Element)
// 难度：简单 | 分类：技巧
// https://leetcode.cn/problems/majority-element/
//
// 本地验证：cargo t p0169_majority_element
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [3,2,3]
        // [2,2,1,1,1,2,2]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::majority_element(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0169_majority_element
}
