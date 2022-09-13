/**
 * Main
 * 
 * Entry point for this application.
 */
fn main() {
    /* Welcome banner. */
    welcome_banner();

    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    println!("Pattern : {}", pattern);
    println!("Path    : {}\n", path);

    // let a = 100;
    // let b = 200;
    // println!("\nResult is {}", a + b);
    // assert_eq!((a + b), 300);

    // let arr:[i32;4] = [1,2,3,4];
    // println!("\nArray size is {}", arr.len());

    // for i in 0..3 {
    //     println!("Hi there {}!", i);
    // }

    // let mut sum = 0.0;
    // for i in 0..4 {
    //     sum += i as f64;
    // }
    // println!("\nThe sum is {}", sum);

    // let my_sqr = sqr(4.0);
    // println!("\nSquare is {}", my_sqr);

    // let j = 10;
    // let res1 = by_ref(&j);
    // let res2 = by_ref(&42);
    // println!("\nReferences are [ {} ] and [ {} ]", res1, res2);

    // let mut res3 = 0.0;
    // modifies(&mut res3);
    // println!("\nModified value to {}", res3);
}

/**
 * Squared
 * 
 * Calculate the square of a number.
 */
fn sqr(x: f64) -> f64 {
    x * x
}

fn by_ref(x: &i32) -> i32 {
    println!("\nRef of (x) is {} and {}", x, *x);

    *x + 1
}

fn modifies(y: &mut f64) {
    println!("\nRef of (y) is {} and {}", y, *y);

    *y = 1.337;
}

fn get_version() -> &'static str {
    return "22.9.12 (alpha)";
}

/**
 * Welcome Banner
 * 
 * Prints a welcome banner when the CLI is executed.
 */
fn welcome_banner() {
    println!(r"
    _________    ___.                  __      ________                    
    /   _____/__ _\_ |__   ____   _____/  |_   /  _____/ __ _________ __ __ 
    \_____  \|  |  \ __ \ /    \_/ __ \   __\ /   \  ___|  |  \_  __ \  |  \
    /        \  |  / \_\ \   |  \  ___/|  |   \    \_\  \  |  /|  | \/  |  /
   /_______  /____/|___  /___|  /\___  >__|    \______  /____/ |__|  |____/ 
           \/          \/     \/     \/               \/                    ");

    println!("                                                     v{}\n", get_version());
}