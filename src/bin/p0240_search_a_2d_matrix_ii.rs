#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 240. 搜索二维矩阵 II (Search a 2D Matrix II)
// 难度：中等 | 分类：矩阵
// https://leetcode.cn/problems/search-a-2d-matrix-ii/
//
// 本地验证：cargo t p0240_search_a_2d_matrix_ii
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]]
        // 5
        // [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]]
        // 20
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::search_matrix(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0240_search_a_2d_matrix_ii
}
