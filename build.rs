fn main() {
    // It's necessary to use an absolute path here because the
    // C++ codegen and the macro codegen appears to be run from different
    // working directories.
    let mut b = autocxx_build::build(
        "src/main.rs",
        &["depend/monero/src", "depend/monero/contrib/epee/include", "depend/monero/external/easylogging++", "depend/monero/external"],
        &Vec::<String>::new(),
    )
    .unwrap();

    b.flag_if_supported("-std=c++14")
        .compile("monero-wallet-sys");

    println!("cargo:rerun-if-changed=src/main.rs");
}
