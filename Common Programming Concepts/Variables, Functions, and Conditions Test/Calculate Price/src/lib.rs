pub fn calculate_price(num_apples:u32) -> u32 {
    let mut cost = 0;
    if num_apples >= 40 {
        cost = num_apples;
    } else {
        cost = num_apples * 2;
    }
    cost
}
