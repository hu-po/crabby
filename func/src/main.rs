fn main() {
    let y = {
        let x = 3;
        add_one(x)
    };
    println!("The value of y is: {}", y);
}

fn add_one(x: i32) -> i32 {
    x + 1 // no semi-colon because this is the return value
}
