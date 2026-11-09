#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 238. 除了自身以外数组的乘积 (Product of Array Except Self)
// 难度：中等 | 分类：普通数组
// https://leetcode.cn/problems/product-of-array-except-self/
//
// 本地验证：cargo t p0238_product_of_array_except_self
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,2,3,4]
        // [-1,1,0,-3,3]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::product_except_self(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0238_product_of_array_except_self
}
