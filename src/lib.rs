pub fn vendor_iperf() {
    // Compile iperf using autotools with configuration to output a static library
    let dst =
        autotools::Config::new(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("iperf"))
            .enable_static()
            .config_option("with-ssl", None)
            // .fast_build(true)
            .build();

    // Explicitly search library directory
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Add static library+deps to link list
    println!("cargo:rustc-link-lib=static=iperf");
    println!("cargo:rustc-link-lib=static=sctp");
}
