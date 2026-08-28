fn main() {
    // So conditional compilation configs would be brought in: (optional)
    //  -> https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/#conditional-compilation
    embuild::espidf::sysenv::output();

    println!("cargo:rerun-if-changed=wrap.h");
}
