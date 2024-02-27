const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    println!("The value of y (not shadowing) is: {}", y);
    // シャドーイング (もとの y を覆い隠す（上書き）)
    let y = y + 1;
    println!("The value of y (first shadowing) is: {}", y);
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    // スコープの外はシャドーイングの影響を受けない
    // 初めのシャドーイングの影響は受ける
    println!("The value of y (effected first shadowing) is: {}", y);
}
