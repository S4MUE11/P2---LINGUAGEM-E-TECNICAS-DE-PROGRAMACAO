
#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let capacity = max_weight as usize;
    let mut dp = vec![0; capacity + 1];

    for item in items {
        let weight = item.weight as usize;

        if weight <= capacity {
            for current_weight in (weight..=capacity).rev() {
                dp[current_weight] = dp[current_weight]
                    .max(dp[current_weight - weight] + item.value);
            }
        }
    }

    dp[capacity]
}
