/// https://leetcode.com/problems/min-cost-climbing-stairs/description/
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    if cost.len() < 2 {
        return cost[0];
    }
    let mut min_cost = vec![0; cost.len() + 1];

    min_cost[0] = 0;
    min_cost[1] = 0;

    for i in 2..cost.len() + 1 {
        min_cost[i] = (min_cost[i - 1] + cost[i - 1]).min(min_cost[i - 2] + cost[i - 2]);
    }

    min_cost[cost.len()]
}

pub fn min_cost_climbing_stairs_v2(cost: Vec<i32>) -> i32 {
    if cost.len() < 2 {
        return cost[0];
    }

    let mut cost0 = 0;
    let mut cost1 = 0;
    let mut result = 0;
    for i in 2..cost.len() + 1 {
        result = (cost1 + cost[i - 1]).min(cost0 + cost[i - 2]);
        cost0 = cost1;
        cost1 = result;
    }

    result
}
