fn main() {
    // mixed type tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // access individual elements
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("First element: {}", a);
    println!("Second element: {}", b);
    println!("Third element: {}", c);

    // arrays have fixed length and type
    let a: [u8; 5] = [1, 2, 3, 4, 5];

    // vectors can have variable length
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.push(6);
}
