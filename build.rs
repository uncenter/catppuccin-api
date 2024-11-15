use reqwest::blocking::get;
use serde_yaml::Value;
use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("outdir: {}", out_dir);
    let yaml_urls = vec![
        "https://github.com/catppuccin/catppuccin/raw/main/resources/ports.yml",
        "https://github.com/catppuccin/userstyles/raw/main/scripts/userstyles.yml",
    ];
    let json_out_paths = vec![
        Path::new(&out_dir).join("ports.json"),
        Path::new(&out_dir).join("userstyles.json"),
    ];

    for (url, json_path) in yaml_urls.iter().zip(json_out_paths.iter()) {
        let yaml_content = get(*url).unwrap().text().unwrap();
        let yaml_data: Value = serde_yaml::from_str(&yaml_content).unwrap();
        let json_content = serde_json::to_string(&yaml_data).unwrap();
        fs::write(json_path, json_content).unwrap();
    }
}
