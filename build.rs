use rust_embed::RustEmbed;
use std::env;
use std::fs;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "alphabets"]
struct Asset;
fn main() {
    let files: Vec<String> = Asset::iter().map(|a| a.to_string()).collect();
    let mut alphabets: Vec<(String, String)> = files
        .iter()
        .map(|file_path| {
            let file = Asset::get(file_path).unwrap();
            let header = &String::from_utf8_lossy(&file.data)[2..]; // Strip "# " from the header
            (
                file_path.to_string(),
                header.split('\n').next().unwrap().to_string(),
            )
        })
        .collect();
    alphabets.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut contents = String::new();
    contents.push_str(
        "use strum_macros::{Display, EnumString};\n\
        #[derive(Debug, Display, EnumString, Clone)]\n\
        #[allow(non_camel_case_types)]\n\
        pub enum Alphabet {\n",
    );

    for (language, header) in alphabets {
        contents.push_str(&format!("    /// {}\n", header));
        contents.push_str(&format!("    {},\n", language));
    }
    contents.push_str("}\n");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    dbg!(&out_dir);
    let dest_path = Path::new(&out_dir).join("alphabet_kinds.rs");
    fs::write(dest_path, contents).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
