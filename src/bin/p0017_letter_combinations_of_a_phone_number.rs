#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 17. 电话号码的字母组合 (Letter Combinations of a Phone Number)
// 难度：中等 | 分类：回溯
// https://leetcode.cn/problems/letter-combinations-of-a-phone-number/
//
// 本地验证：cargo t p0017_letter_combinations_of_a_phone_number
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // "23"
        // "2"
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::letter_combinations(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0017_letter_combinations_of_a_phone_number
}
