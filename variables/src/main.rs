fn main() {
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // You can change types of shadowed variables
    let num = "42";
    let num: u32 = num.parse().expect("Not a number!");
    println!("The value of num is: {}", num);
}
