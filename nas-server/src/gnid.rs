use std::path::Path;

use blake3::Hasher;
use chrono::Utc;

pub struct GNID {}

pub fn gnid(source: &str) -> String {
    let timestamp = Utc::now().timestamp();
    let input = format!("{}{}", source, timestamp);

    let mut hasher = Hasher::new();
    hasher.update(input.as_bytes());
    let hash = hasher.finalize().to_string()[..16].to_string();
    hash
}

pub fn is_in_udir<P1: AsRef<Path>, P2: AsRef<Path>>(path: P1, fp: P2) -> bool {
    let czp = match path.as_ref().canonicalize() {
        Ok(p) => p,
        Err(_) => return false,
    };
    let edp = match fp.as_ref().canonicalize() {
        Ok(p) => p,
        Err(_) => {
            return false;
        }
    };
    czp.starts_with(&edp)
}

pub fn get_pstr<P: AsRef<Path>>(path: P) -> String {
    path.as_ref().to_string_lossy().replace('\\', "/")
}
