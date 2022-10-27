use config::{Config, File};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn load_from_file(path: &str) -> HashMap<String, String> {
    let config = Config::builder()
        .add_source(File::with_name(path))
        .build()
        .unwrap();

    config.try_deserialize::<HashMap<String, String>>().unwrap()
}