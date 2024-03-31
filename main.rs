use serde_json::Value;
use std::{collections::HashMap, error, fs,  path::Path};

enum PackageJSONExports {
    None,
    String(String),
    Object(serde_json::Map<String, String>),
}

fn check_if_file_exists_in_path(path: &String) -> bool {
    Path::new(path).exists()
}

fn check_exports(value: PackageJSONExports) -> Result<bool, Box<dyn error::Error>> {
    match value {
        PackageJSONExports::None => Err("Unable to find exports".into()),
        PackageJSONExports::String(path) => {
            if check_if_file_exists_in_path(&path) {
                Ok(true)
            } else {
                Err(format!("Unable to find {} file", path).into())
            }
        }
        PackageJSONExports::Object(exports_value) => {
            for (key, value) in exports_value.into_iter() {
                if !check_if_file_exists_in_path(&value) {
                    return Err(format!("Unable to find {} export: {}", key, value).into());
                }
            }

            Ok(true)
        }
    }
}

fn main() {
    // todo read from script parameter or use default (./package.json)
    // let file_path: String = "./tests/no-exports/package.json".to_owned();
    let file_path: String = "./tests/exports-string/package.json".to_owned();
    // let file_path: String = "./tests/exports-single-object/package.json".to_owned();

    let package_json_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let package_json_raw: Value =
        serde_json::from_str(package_json_content.as_str()).expect("Content is not a valid json");

    println!("package.json found. Name: {}", package_json_raw["name"]);

    assert!(
        package_json_raw["exports"].is_string() || package_json_raw["exports"].is_object(),
        "\"exports\" property not found"
    );

    let mut exports_value = PackageJSONExports::None;

    if package_json_raw["exports"].is_string() {
        exports_value = PackageJSONExports::String(package_json_raw["exports"].to_string());
    }

    if package_json_raw["exports"].is_object() {
        exports_value = PackageJSONExports::Object(package_json_raw["exports"].as_object());
    }

    match check_exports(exports_value) {
        Ok(_) => print!("All good"),
        Err(err) => panic!("Exports value not found: {}", err),
    }
}
