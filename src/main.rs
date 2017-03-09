#[macro_use]
extern crate clap;
use clap::Arg;

extern crate regex;
use regex::Regex;

use std::io;
use std::io::prelude::*;

fn main() {
    // Parse Command line arguments 
    //   including cargo.toml forces rebuild on changes there 
    include_str!("../Cargo.toml");
    let args = app_from_crate!()
        .arg(Arg::with_name("relax")
             .short("r")
             .long("relax")
             .help("Use relaxed search"))
        .get_matches();
    
    let use_relaxed : bool = args.is_present("relax");

    // Regex Declarations
    let re_strict = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)").unwrap();
    let re_relaxed = Regex::new(r"(http(s)?://.)?(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)").unwrap();

    let regex;
    if use_relaxed {
        regex = re_relaxed;
    } else {
        regex = re_strict;
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        for cap in regex.captures_iter(&line.unwrap()) {
            println!("{}", &cap[0]);
        }
    }
}
