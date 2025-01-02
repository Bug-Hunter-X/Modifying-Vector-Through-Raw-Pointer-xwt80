fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 10; // Modify vector through raw pointer
    }
    // This print statement may cause a panic or show incorrect data
    println!("Value at ptr: {}", *ptr);
    v[0] = 20; //Modifying the vector in a safe way.
    println!("Vector v: {:?}", v);
} 