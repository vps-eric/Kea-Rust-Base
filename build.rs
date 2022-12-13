fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src"); // include path
    let kea_src_1 = std::path::PathBuf::from("/usr/local/include/kea");
    let kea_src_2 = std::path::PathBuf::from("/usr/local/lib");
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path, &kea_src_1, &kea_src_2])
        .build()?;
    b.flag_if_supported("-std=c++14")
     .compile("shim"); // Change your library name here
    println!("cargo:rerun-if-changed=src/lib.rs");

    println!("cargo:rustc-link-search=/usr/local/include/kea");
    println!("cargo:rustc-link-search=/usr/local/lib");

    println!("cargo:rustc-link-lib=kea-hooks");

    Ok(())
}