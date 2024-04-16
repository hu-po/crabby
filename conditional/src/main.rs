fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    
    // label loops to break out of nested looops
    let mut counter = 0;
    'outerloop: loop {
        counter += 3;
        println!("counter: {}", counter);
        loop {
            counter += 1;
            if counter == 10 {
                break 'outerloop;
            }
        }
    }

    // element-wise loop
    let mut a = [10, 20, 30, 40, 50];
    for element in a {
        println!("element: {}", element);
    }
    println!("a: {:?}", a);

}
