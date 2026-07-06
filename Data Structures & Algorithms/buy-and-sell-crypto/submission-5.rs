impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // cannot make a transaction with one element in prices
        if prices.len() == 1 {
            return 0;
        }
        let mut current_buy_price = prices[0]; // current buy is the first element in the prices vector
        let mut current_sell_price = 0; // smallest number that prices[i] can be
        // track the max profit so far
        let mut max_profit = 0;
        
        for i in 1..prices.len() {
            println!("{} {} {} {}", i, current_buy_price, current_sell_price, max_profit);
            // if prices[i] < current_buy_price, then that is the new current_buy_price, and then continue
            if prices[i] < current_buy_price {
                current_buy_price = prices[i];
                current_sell_price = 0;
                continue;
            }
            else if prices[i] >= current_sell_price {
                // if price is larger than the current sell price, update and update max profit
                current_sell_price = prices[i];
                max_profit = max_profit.max(current_sell_price-current_buy_price);
            }
        }
        return max_profit;
    }
}
