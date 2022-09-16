#![allow(unused)]

/* Import modules. */
mod utils;
mod welcome;

/* Initailize 3rd-party crates. */
use clap::Parser;
use log::{info, warn};

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
    /* Display (Welcome) banner. */
    welcome::display_banner();

    // let pattern = std::env::args().nth(1).expect("Oops! You MUST provide a pattern to search.");
    // let path = std::env::args().nth(2).expect("Oops! You MUST provide filepath to scan.");
    let args = Cli::parse();

    // println!("Pattern : {}", pattern);
    // println!("Path    : {}\n", path);
    println!("  Search pattern is : {}", args.pattern);
    println!("     Search path is : {}\n", args.path.display());

    // let content = std::fs::read_to_string(&args.path);
    //     .expect("Oops! Could not read the file you specified.");
    let result = std::fs::read_to_string(&args.path);

    match result {
        Ok(content) => { 
            for line in content.lines() {
                if line.contains(&args.pattern) {
                    println!("  Look! We found a line ---> {}", line);
                }
            }

            println!(); // empty line / spacer
        }
        
        Err(error) => { 
            println!("Oops! Could not read the file you specified.\n\n[ {} ]", error); 
        }
    }

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

    let my_sqr = utils::sqr(4.0);
    println!("  Square is {}\n", my_sqr);

    // let j = 10;
    // let res1 = by_ref(&j);
    // let res2 = by_ref(&42);
    // println!("\nReferences are [ {} ] and [ {} ]", res1, res2);

    // let mut res3 = 0.0;
    // modifies(&mut res3);
    // println!("\nModified value to {}", res3);

    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!\n");

    utils::start_download();
    println!("\n");

}
