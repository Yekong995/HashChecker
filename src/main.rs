use hash_checker::*;
use clap::Parser;

use std::io::Read;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author = "Yekong", version = "0.1.0", about = "A simple hash checker",
long_about = "Generate hash for a file or do a hash check for a file")]
struct Args {

    /// File to be hashed
    file: String,

    /// Hash algorithm to use (md5, sha1, sha256, sha512)
    #[arg(long, default_value = "sha256")]
    hash: String,

    /// If this is provided, do a hash check
    #[arg(short, long)]
    check: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();
    
    let filename = args.file;

    if args.check.is_some() {
        let check = args.check.clone().unwrap().to_lowercase();

        let mut file = File::open(filename.clone())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut hasher = select_hasher(&args.hash);
        let hash = use_hasher(&mut *hasher, &buffer);

        let strs = hash
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("");
        
        println!("{}::{}", args.hash.to_uppercase(), strs);
        println!("Hash you provided: {}", check);
        
        if strs == check {
            println!("Hash check passed!");
        } else {
            println!("Hash check failed!");
        }

    } else {

        let mut file = File::open(filename.clone())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut hasher = select_hasher(&args.hash);
        let hash = use_hasher(&mut *hasher, &buffer);

        let strs = hash
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("");

        println!("{}::{}", args.hash.to_uppercase(), strs);
    }

    Ok(())
}
