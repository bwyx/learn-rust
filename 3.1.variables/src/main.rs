fn main() {
    // let
    // able to make variable mutable with `mut`
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // const
    // always immutable, can't use `mut` keyword
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}
