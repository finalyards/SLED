use bindgen::{
    builder,
    EnumVariation,
};

use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {

    // Detect when IDE is running us, and DO NOT ENGAGE with 'esp-idf-sys' if we're under IDE.
    // i.e. keep this before 'embuild::...' - otherwise '.espressif' or '.embuild' start occurring
    //      on the host disk.
    //
    //  - Rust Rover (host):
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    //  - Rust Rover (remote development):
    //      - REMOTE_DEV_SERVER_IS_NATIVE_LAUNCHER=1
    {
        if env::var("__CFBundleIdentifier").is_ok() ||
            env::var("REMOTE_DEV_SERVER_IS_NATIVE_LAUNCHER").is_ok() {
            panic!("IDE build cut short");   // try to avoid spending a _lot_ of time, building ESP-IDF on the IDE
            //return;  // skip the rest
        }
    }

    // DEBUG
    //
    //      ESP_IDF_TOOLS_INSTALL_DIR=global
    //      ESP_IDF_VERSION=v5.5.5
    //
    #[cfg(false)]
    {
        env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        panic!();
    }

    ensure_no_sdkconfig();

    // Needed for linking of executables to succeed.
    embuild::espidf::sysenv::output();

    let include_paths: Vec<String>;

    // Create the bindings for 'led_strip'
    {
        // 'esp-idf-sys' has brought in the extra components into (e.g.):
        //  <<
        //      /home/lima/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-452cda0294ec9033/out/managed_components
        //  <<
        //
        // Below which:
        //  `-- espressif__led_strip
        //      `-- include
        //          `-- led_strip.h
        //          `-- led_strip_rmt.h
        //          `-- led_strip_spi.h
        //          `-- led_strip_types.h
        //
        // Note that this is NOT our 'OUT_DIR'.
        //
        let sysroot: String = get_sysroot().to_string_lossy().into();

        let mcu = env!("MCU");
        assert!(mcu == "esp32c6", "INTERNAL: MCU {} currently not supported", mcu);

        include_paths = {
            let my_include_path = {
                #[allow(non_snake_case)]
                let DEP_ESP_IDF_ROOT = env::var("DEP_ESP_IDF_ROOT")
                    .expect("🟥 missing 'DEP_ESP_IDF_ROOT' env.var");
                // /home/lima/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-452cda0294ec9033/out

                format!("{}/managed_components/espressif__led_strip/include", DEP_ESP_IDF_ROOT)
            };

            let sdkconfig_h_include: String = {
                #[allow(non_snake_case)]
                let DEP_ESP_IDF_ROOT = env::var("DEP_ESP_IDF_ROOT")
                    .expect("🟥 missing 'DEP_ESP_IDF_ROOT' env.var");
                // /home/lima/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-452cda0294ec9033/out

                format!("{DEP_ESP_IDF_ROOT}/build/config") // /sdkconfig.h
            };

            // Lots of general include paths
            let generic_include_paths: [String;_] = {
                #[allow(non_snake_case)]
                let ESP_IDF_PATH = env::var("DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH")
                    .expect("🟥 missing 'DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH' env.var");
                // /home/lima/.espressif/esp-idf/v5.5.5

                [
                    "components/esp_common/include",
                    "components/hal/include",
                    "components/esp_driver_rmt/include",    // driver/rmt_types.h
                    "components/esp_driver_spi/include",    // driver/spi_master.h
                    format!("components/soc/{mcu}/include").as_str(),       // soc/clk_tree_defs.h
                    format!("components/soc/{mcu}/register").as_str(),      // soc/interrupt_matrix_reg.h
                    "components/esp_hw_support/include",    // spinlock.h
                    "components/freertos/FreeRTOS-Kernel/include",  // freertos/FreeRTOS.h
                    "components/freertos/FreeRTOS-Kernel/portable/riscv/include/freertos",  // portmacro.h
                    "components/freertos/config/include",           // freertos/FreeRTOSConfig.h
                    "components/freertos/config/include/freertos",  // FreeRTOSConfig.h
                    "components/freertos/config/riscv/include",     // freertos/FreeRTOSConfig_arch.h
                    "components/riscv/include",             // riscv/csr.h
                    "components/esp_rom/include",           // esp_rom_sys.h
                    "components/heap/include",              // esp_heap_caps.h
                    "components/esp_system/include",        // esp_system.h
                    "components/newlib/platform_include",   // esp_newlib.h
                ]
                    .map(|x| format!("{}/{}", ESP_IDF_PATH, x))
            };

            // tbd. construct this in a better way
            std::iter::once(my_include_path)
                .chain([sdkconfig_h_include])
                .chain([format!("{sysroot}/riscv32-esp-elf/include")])
                .chain(generic_include_paths)
                .collect()
        };

        for p in &include_paths {
            if !PathBuf::from(&p).exists() {
                println!("cargo:warning=🟥 Did not exist: {}", p);    // carry on..
            }
        }

        let clang_args = {
            let mut v = vec![];

            v.extend(["-D__riscv".into()]);
            v.extend(["-D__riscv_xlen=32".into()]);
            v.extend(["--sysroot".into(), sysroot]);
            v.extend( include_paths.iter().map( |p| format!("-I{p}") ) );
            v
        };

        let wrap_c_path = PathBuf::from("tmp/wrap.c");
        let bindings = builder()
            .header("wrap.h")
            .default_enum_style( EnumVariation::NewType { is_global: false, is_bitfield: false } )
            .use_core()     // prefer 'core' over 'std'
            .fit_macro_constants(true)
            .derive_default(true)
            .generate_inline_functions(true)
            .clang_args(clang_args)
            //
            .wrap_static_fns(true)
            .wrap_static_fns_path(&wrap_c_path)
            .wrap_static_fns_suffix("_ext")
                // Needs some extension to keep the 'wrap.c' function names separate from inline functions (in 'wrap.h').
                // However, this is hidden from the Rust level (only matters in linking); VERY NICE!
                // default is "_extern"
            //
            .allowlist_function("led_strip_.+")
            .allowlist_function("wrap_.+")
            //
            .allowlist_item("soc_periph_rmt_.+")
            .blocklist_item("(spi|.+_spi)_.+")
            //
            .generate()
        .expect("🟥 Unable to generate bindings");

        bindings
            .write_to_file("tmp/bindings_0.rs")
            .expect("🟥 Couldn't write Rust/C bindings");
    }

    // The generated 'tmp/wrap.c' needs to be compiled.
    {
        let wrap_c_path = Path::new("tmp/wrap.c");
        assert!(wrap_c_path.exists());

        let sysroot = get_sysroot();
            // /home/lima/.espressif/tools/riscv32-esp-elf/esp-14.2.0_20260121/riscv32-esp-elf/

        let cc_path = sysroot.join("bin")
            .join("riscv32-esp-elf-gcc");

        cc::Build::new()
            .compiler(cc_path)
            .file(wrap_c_path)
            .includes(include_paths)
            .include(".")   // wrap.h
            .flag(format!("--sysroot={}", sysroot.display()))
            .compile("wrap_static");
    }

    // What files should trigger a new 'build.rs' run (i.e. affect the binding)?
    //
    // Cargo note: 'build.rs' is _not_ executed for every build. It does not need to, if only the
    //      sources change. Here, we list the files that should trigger a rebuild.
    //
    println!("cargo:rerun-if-changed=build.rs");
    // +
    println!("cargo:rerun-if-changed=bindings.rs");
    //
    // This is  *output* but helps _dependent_crates_ to realize if it has been removed, and
    // avoid a failing upstream build.
    println!("cargo:rerun-if-changed=tmp/bindings_0.rs");
}

/// From an env.var 'DEP_ESP_IDF_EMBUILD_ENV_PATH', distill the path for '--sysroot'
fn get_sysroot() -> PathBuf {

    // DEP_ESP_IDF_EMBUILD_ENV_PATH=
    // [...]:/home/lima/.espressif/tools/riscv32-esp-elf/esp-14.2.0_20260121/riscv32-esp-elf/bin:[...]

    let a = env::var("DEP_ESP_IDF_EMBUILD_ENV_PATH")
        .expect("🟥 Missing 'DEP_ESP_IDF_EMBUILD_ENV_PATH' env.var");

    let riscv_bin = a.split(':')
        .find(|p| p.contains("riscv32-esp-elf/bin"))
        .expect("🟥 Failed to figure out sysroot");

    // return without 'bin'
    PathBuf::from(riscv_bin).parent().unwrap().to_path_buf()
}

/**
* Ensure there isn't a 'sdkconfig' file accidentally left lying around. It *could* get picked
* up as the truth (instead of generating one below "out dir"), and confuse builds.
*/
fn ensure_no_sdkconfig() {
    let fn_ = std::path::Path::new("sdkconfig");
    if fn_.exists() {
        panic!(
            "❗️There's an 'sdkconfig' file. It is generally UNWELCOME. Please remove it and try again."
        );
    }
}
