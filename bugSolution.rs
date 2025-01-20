fn main() {
    let mut v = vec![1, 2, 3];
    //Safe method
    v[0] = 4; // Modify the first element using safe indexing
    println!("Modified vector: {:?}", v);

    // Safe alternative to raw pointers
    let x = 10;
    let y = x;
    println!("Value of y: {}", y);
} 