#![allow(
    unused_variables,
    dead_code,
    unused_imports,
    clippy::items_after_test_module,
    clippy::ptr_arg
)]
// 121. 买卖股票的最佳时机 (Best Time to Buy and Sell Stock)
// 难度：简单 | 分类：贪心算法
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/
//
// 本地验证：cargo t p0121_best_time_to_buy_and_sell_stock
// 提交力扣时只复制 impl Solution（设计类复制 struct + impl），不要复制 use / tests / main。

use leetcode::prelude::*;

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        // 官方示例输入（每行 / 每组对应一个参数）：
        // [7,1,5,3,6,4]
        // [7,6,4,3,1]
        //
        // 补一个断言后再运行 cargo t，例如：
        // assert_eq!(Solution::max_profit(/* 参数 */), /* 期望值 */);
    }
}

fn main() {
    // 调试入口：cargo r p0121_best_time_to_buy_and_sell_stock
}
