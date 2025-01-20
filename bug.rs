fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 4; // Modify the first element
    }
    println!("Modified vector: {:?}", v);

    //Dangrous code
    let x = 10;
    let raw_ptr = &x as *const i32;
    unsafe {
        let y = *raw_ptr;
        println!("Value of y: {}", y);
    }
}