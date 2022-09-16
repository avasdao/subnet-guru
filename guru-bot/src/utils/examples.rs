pub fn by_ref(x: &i32) -> i32 {
    println!("\nRef of (x) is {} and {}", x, *x);

    *x + 1
}

pub fn modifies(y: &mut f64) {
    println!("\nRef of (y) is {} and {}", y, *y);

    *y = 1.337;
}
