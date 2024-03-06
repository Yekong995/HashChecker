extern crate colored;
use colored::*;
use digest::DynDigest;

use std::fs::{File, metadata};
use std::io::{Write, Read};
use std::path::Path;

pub fn use_hasher(hasher: &mut dyn DynDigest, data: &[u8]) -> Box<[u8]> {
    hasher.update(data);
    hasher.finalize_reset()
}

pub fn select_hasher(s: &str) -> Box<dyn DynDigest> {
    match s {
        "md5" => Box::new(md5::Md5::default()),
        "sha256" => Box::new(sha2::Sha256::default()),
        "sha512" => Box::new(sha2::Sha512::default()),
        "sha1" => Box::new(sha1::Sha1::default()),
        _ => unimplemented!("Unknown hasher"),
    }
}

pub fn generate_hash_file(filename: String) -> Result<(), Box<dyn std::error::Error>> {

    let mut file = File::open(filename.clone())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hasher = select_hasher("md5");
    let md5 = use_hasher(&mut *hasher, &buffer);
    let mut hasher = select_hasher("sha1");
    let sha1 = use_hasher(&mut *hasher, &buffer);
    let mut hasher = select_hasher("sha256");
    let sha256 = use_hasher(&mut *hasher, &buffer);
    let mut hasher = select_hasher("sha512");
    let sha512 = use_hasher(&mut *hasher, &buffer);

    let md5_str = md5
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("");
    let sha1_str = sha1
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("");
    let sha256_str = sha256
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("");
    let sha512_str = sha512
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("");

    let mut file = File::create(format!("{}.hash", filename))?;
    file.write_all(format!("MD5::{}\n", md5_str).as_bytes())?;
    file.write_all(format!("SHA1::{}\n", sha1_str).as_bytes())?;
    file.write_all(format!("SHA256::{}\n", sha256_str).as_bytes())?;
    file.write_all(format!("SHA512::{}\n", sha512_str).as_bytes())?;

    Ok(())
}

/// Check if the file exists and is a file
pub fn is_file(filename: String) -> Result<bool, Box<dyn std::error::Error>> {

    let path = Path::new(&filename);
    if path.exists() {
        let metadata = metadata(filename.clone())?;
        if metadata.is_file() {
            return Ok(true)
        }
    }

    Ok(false)
}

/// Check the file with the hash file provide by the user
/// 
/// # Arguments
/// 
/// * `filename` - The hash file
/// * `filetocheck` - The file to check
pub fn check_with_hash_file(filename: String, filetocheck: String) -> Result<(), Box<dyn std::error::Error>> {

    let mut file = File::open(filename.clone())?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let str_vec = buffer.split("\n").collect::<Vec<&str>>();
    for i in str_vec {
        if i == "" {
            continue;
        }
        let data = i.split("::").collect::<Vec<&str>>();
        let hash_type = data[0];
        let hash_data = data[1];
        
        let mut file = File::open(filetocheck.clone())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut hasher = select_hasher(hash_type.to_lowercase().as_str());
        let hash = use_hasher(&mut *hasher, &buffer);

        let strs = hash
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("");
        
        if strs == hash_data {
            println!("{}::{}", hash_type.to_uppercase(), strs);
            println!("Hash you provided: {}", hash_data);
            println!("Hash type {} have passed!", hash_type.to_uppercase().green());
        } else {
            println!("{}::{}", hash_type.to_uppercase(), strs);
            println!("Hash you provided: {}", hash_data);
            println!("Hash type {} have failed!", hash_type.to_uppercase().red());
        }
    }

    Ok(())
}
