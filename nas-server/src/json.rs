use std::{fs::File, io::BufReader};

use serde::de::DeserializeOwned;

pub fn read_json<T: DeserializeOwned>(path: &str) -> Result<T, serde_json::Error> {
    let f = File::open(path).expect("failed to open File ");
    let br = BufReader::new(f);
    serde_json::from_reader(br)
}
