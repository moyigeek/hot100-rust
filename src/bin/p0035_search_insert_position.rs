#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 35. 搜索插入位置 (Search Insert Position)
// 难度：简单 | 分类：二分查找
// https://leetcode.cn/problems/search-insert-position/
//
// 本地验证：cargo t p0035_search_insert_position
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [1,3,5,6]
        // 5
        // [1,3,5,6]
        // 2
        // [1,3,5,6]
        // 7
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::search_insert(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0035_search_insert_position
}
