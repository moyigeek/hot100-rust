#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 62. 不同路径 (Unique Paths)
// 难度：中等 | 分类：多维动态规划
// https://leetcode.cn/problems/unique-paths/
//
// 本地验证：cargo t p0062_unique_paths
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // 3
        // 7
        // 3
        // 2
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::unique_paths(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0062_unique_paths
}
