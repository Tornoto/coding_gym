/// https://leetcode.com/problems/gas-station/description/
pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let left_gas: Vec<i32> = gas.iter().zip(cost.iter()).map(|(g, c)| g - c).collect();

    if left_gas.iter().sum::<i32>() < 0 {
        return -1;
    }

    let mut remaining = 0;
    let mut start = 0;
    for (index, g) in left_gas.iter().enumerate() {
        remaining += g;
        if remaining < 0 {
            remaining = 0;
            start = index + 1;
        }
    }

    start as i32
}
