#![allow(unused)]

/* Import modules. */
mod crypto;
mod p2p;
mod utils;
mod welcome;

/* Initailize 3rd-party crates. */
use clap::{Arg, App};
use clap::Parser;
use human_panic::setup_panic;
use log::{info, warn};
use serde_json::json;

// #[derive(Parser)]
// struct Cli {
//     seed_phrase: String,

//     #[clap(parse(from_os_str))]
//     config_path: std::path::PathBuf,
// }
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[clap(short, long, value_parser, default_value = "Satoshi")]
   name: String,

   /// Number of times to greet
   #[clap(short, long, value_parser, default_value_t = 1)]
   count: u8,
}


/**
 * Main
 * 
 * Entry point for this application.
 */
fn main() {
    /* Setup (human) panic. */
    setup_panic!();

    /* Display (Welcome) banner. */
    welcome::display_banner();

    // let args = Cli::parse();
    // println!("  (Private) seed phrase is : {}", args.seed_phrase);
    // println!("     Configuration path is : {}\n", args.config_path.display());

    // let args = Args::parse();

    /* Handle application arguments. */
    let matches = App::new("Subnet Guru")
        .version(guru::get_version().as_str())
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("seed")
            .required(false)
            .takes_value(true)
            .index(1)
            .help("12 or 24 word seed phrase"))
        .arg(Arg::with_name("url")
            .required(false)
            .takes_value(true)
            .index(2)
            .help("url to download"))
        .get_matches();

    /* Handle (master) seed. */
    let seed = matches
        .value_of("seed")
        .unwrap_or("");
    if !seed.is_empty() {
        println!("  Master seed is [ {} ]\n", seed);
    }

    /* Handle URL. */
    let url = matches
        .value_of("url")
        .unwrap_or("");
    if !url.is_empty() {
        println!("  URL is [ {} ]\n", url);
    }

    // for _ in 0..args.count {
    //     println!("  Hi there {}!\n", args.name)
    // }

    // let cmd = clap::Command::new("guru")
    // .bin_name("guru")
    // .subcommand_required(true)
    // .subcommand(
    //     clap::command!("example").arg(
    //         clap::arg!(--"manifest-path" <PATH>)
    //             .value_parser(clap::value_parser!(std::path::PathBuf)),
    //     ),
    // );
    // let matches = cmd.get_matches();
    // let matches = match matches.subcommand() {
    //     Some(("example", matches)) => matches,
    //     _ => unreachable!("clap should ensure we don't get here"),
    // };
    // let manifest_path = matches.get_one::<std::path::PathBuf>("manifest-path");
    // println!("{:?}", manifest_path);
    
    // let result = std::fs::read_to_string(&args.config_path);
    // match result {
    //     Ok(content) => { 
    //         for line in content.lines() {
    //             if line.contains(&args.pattern) {
    //                 println!("  Look! We found a line ---> {}", line);
    //             }
    //         }

    //         println!(); // empty line / spacer
    //     }
        
    //     Err(error) => { 
    //         println!("Oops! Could not read the file you specified.\n\n[ {} ]", error); 
    //     }
    // }

    // let a = 100;
    // let b = 200;
    // println!("  Result is {}\n", a + b);
    // assert_eq!((a + b), 300);

    // let arr:[i32;4] = [1,2,3,4];
    // println!("  Array size is {}\n", arr.len());

    // let my_sqr = crypto::math::sqr(4.0);
    // println!("  Square is {}\n", my_sqr);

    // let j = 10;
    // let res1 = by_ref(&j);
    // let res2 = by_ref(&42);
    // println!("\nReferences are [ {} ] and [ {} ]", res1, res2);

    // let mut res3 = 0.0;
    // modifies(&mut res3);
    // println!("\nModified value to {}", res3);

    // env_logger::init();
    // info!("starting up");
    // warn!("oops, nothing implemented!\n");

    // let node = FederationNode {
    //     id: String::from("190171ee-ac37-4e05-988b-a7e683c1e5d3"),
    //     owner: String::from("Shomari"),
    //     title: String::from("Awesome Node # 1337"),
    //     createdAt: String::from("Tuesday"),
    // }
    // println!("  Node ID is: {}\n", node.get_id);

    println!("  {}\n", json!({
        "type": "message",
        "content": "Hi there!",
    }));

    // panic!("Oops! What happened??");

    p2p::ipfs::make_request();

    // utils::remote::start_download();
    // println!("\n");

    // utils::remote::get_json();
}
