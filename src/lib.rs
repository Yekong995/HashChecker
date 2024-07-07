use digest::DynDigest;

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
