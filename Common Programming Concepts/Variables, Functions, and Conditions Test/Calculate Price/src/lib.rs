pub fn calculate_price(apples: u32) -> u32 {
    let price = 2;
    let total = price * apples;
    if apples >= 40 {
        apples * 1
    } else {
        total
    }
}
