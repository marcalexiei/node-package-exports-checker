use serde_json::Value;
use std::fs;

// struct ExportObject {
//     require: String,
//     import: String,
//     node: String,
//     default: String,
//     types: String,
// }

fn main() {
    // todo read from script parameter or use default (./package.json)
    // let file_path: String = "./tests/no-exports/package.json".to_owned();
    // let file_path: String = "./tests/exports-string/package.json".to_owned();
    let file_path: String = "./tests/exports-single-object/package.json".to_owned();

    let package_json_content =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    let package_json_raw: Value = serde_json::from_str(package_json_content.as_str()).expect("Content is not a valid json");

    println!("package.json found. Name: {}", package_json_raw["name"]);

    assert!(package_json_raw["exports"].is_string() || package_json_raw["exports"].is_object(), "\"exports\" property not found");

    println!("Exports found: {}", package_json_raw["exports"]);
}
