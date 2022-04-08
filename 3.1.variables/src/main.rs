fn main() {
    // let
    // able to make variable mutable with `mut`
    let mut x = 5;
    println!("The value of x is: {}", x);
    x += 1;
    println!("The value of x after assigment is: {}", x);

    // shadowing
    // it's fine in rust to shadow a let variable
    let x = x + 1;
    println!("The value of x after shadowing is: {}", x);

    // const
    // always immutable, can't use `mut` keyword
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}
