#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,

    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


/**
 * Main
 * 
 * Entry point for this application.
 */
fn main() {
    /* Welcome banner. */
    welcome_banner();

    // let pattern = std::env::args().nth(1).expect("Oops! You MUST provide a pattern to search.");
    // let path = std::env::args().nth(2).expect("Oops! You MUST provide filepath to scan.");
    let args = Cli::parse();

    // println!("Pattern : {}", pattern);
    // println!("Path    : {}\n", path);
    println!("  Search pattern is : {}", args.pattern);
    println!("     Search path is : {}\n", args.path.display());

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
    return "22.9.16 (alpha)";
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

    println!("                                                      v{}\n", get_version());
}

/**
 * Welcome Banner (Alternate)
 * 
 * Prints a welcome banner when the CLI is executed.
 */
fn welcome_banner_alt() {
    println!(r"

   ███████╗██╗   ██╗██████╗ ███╗   ██╗███████╗████████╗     ██████╗ ██╗   ██╗██████╗ ██╗   ██╗
   ██╔════╝██║   ██║██╔══██╗████╗  ██║██╔════╝╚══██╔══╝    ██╔════╝ ██║   ██║██╔══██╗██║   ██║
   ███████╗██║   ██║██████╔╝██╔██╗ ██║█████╗     ██║       ██║  ███╗██║   ██║██████╔╝██║   ██║
   ╚════██║██║   ██║██╔══██╗██║╚██╗██║██╔══╝     ██║       ██║   ██║██║   ██║██╔══██╗██║   ██║
   ███████║╚██████╔╝██████╔╝██║ ╚████║███████╗   ██║       ╚██████╔╝╚██████╔╝██║  ██║╚██████╔╝
   ╚══════╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝   ╚═╝        ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ");

    println!("                                                                    v{}\n", get_version());
}

/**
 * Welcome Banner (Alternate #2)
 * 
 * Prints a welcome banner when the CLI is executed.
 */
fn welcome_banner_alt_2() {
    println!(r"
    _____ __ __  ____   ____     ___ ______       ____  __ __  ____  __ __ 
   / ___/|  |  ||    \ |    \   /  _]      |     /    ||  |  ||    \|  |  |
  (   \_ |  |  ||  o  )|  _  | /  [_|      |    |   __||  |  ||  D  )  |  |
   \__  ||  |  ||     ||  |  ||    _]_|  |_|    |  |  ||  |  ||    /|  |  |
   /  \ ||  :  ||  O  ||  |  ||   [_  |  |      |  |_ ||  :  ||    \|  :  |
   \    ||     ||     ||  |  ||     | |  |      |     ||     ||  .  \     |
    \___| \__,_||_____||__|__||_____| |__|      |___,_| \__,_||__|\_|\__,_|");

     println!("                                                     v{}\n", get_version());
}
