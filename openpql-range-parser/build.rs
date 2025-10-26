fn main() {
    let source = "src/range.lalrpop";
    println!("cargo:rerun-if-changed={source}");
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .emit_report(true)
        .process_file(source)
        .unwrap();
}
