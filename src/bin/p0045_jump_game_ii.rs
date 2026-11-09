#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 45. 跳跃游戏 II (Jump Game II)
// 难度：中等 | 分类：贪心算法
// https://leetcode.cn/problems/jump-game-ii/
//
// 本地验证：cargo t p0045_jump_game_ii
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [2,3,1,1,4]
        // [2,3,0,1,4]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::jump(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0045_jump_game_ii
}
