fn main() {
    let root_dir = std::path::Path::new(".");
    let xml_dir = root_dir.join("xml").join("src");
    let dtd_dir = root_dir.join("dtd").join("src");
    let common_dir = root_dir.join("common");

    let mut config = cc::Build::new();
    config.include(&xml_dir);
    config
        .flag_if_supported("-std=c11")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-value");

    for path in &[
        xml_dir.join("parser.c"),
        xml_dir.join("scanner.c"),
        dtd_dir.join("parser.c"),
        dtd_dir.join("scanner.c"),
    ] {
        config.file(path);
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    println!(
        "cargo:rerun-if-changed={}",
        common_dir.join("scanner.h").to_str().unwrap()
    );

    config.compile("tree-sitter-xml");
}
