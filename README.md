
# SLED

## 🚥 Warning! 🚥

The Neopixel of the devkit can be <font color=lightblue>**ULTRA BRIGHT**</font> without a diffuser.

**DO NOT LOOK DIRECTLY** at the LED. Attach a piece of paper tape or a transparent bumper pad over the light source. Be safe!

## Requirements

<!-- Author's environment
VM with:
- Ubuntu 26.04 (aarch)

`espflash` 4.4.5, remoted:
- https://github.com/finalyards-org/probe-rs-remote
-->

- 1 pc ESP32-C6 devkit (e.g. [M1](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitm-1/user_guide.html))

	The intention is to support most common ESP32-C6 devkits. See `features` and `DEVS/Board support`.

- Working Rust and Cargo

	>Optimization hint. If you work within a VM, and share your source codes with the host, consider moving `target` to a global place, e.g. `~/target`.
	
	<!-- #hidden
	```
	$ cat ~/.cargo/config.toml 
	[build]
	target-dir = "/home/lima/target"
	```
	-->

	- `ldproxy`

		```
		$ cargo install --locked ldproxy
		```
	
		>Needed for using `ESP_IDF_TOOLS_INSTALL_DIR = "global"`.

- OS packages

	```
	$ sudo apt install curl git pkg-config \
	  libssl-dev python3 python3-venv cmake \
	  build-essential clang
	```

- `espflash`
	
	```
	$ espflash --version
	espflash 4.5.0
	```

	>Hint. You can use [probe-rs-remote](https://github.com/finalyards-org/probe-rs-remote) (GitHub) to keep the DUT physically separate from the development computer.


### Disk space

Be ready to have 6..8 GB for the build. 

This can be surprising. The `esp-idf-sys` setup downloads and installs a whole ESP-IDF development toolchain under `~/.espressif`. 

>`cargo clean` does not remove this folder, but you can do it manually, to reclaim the disk space.


## Steps

```
$ cargo run --release
...
```

## Advanced

### Working with an external component (`led_strip`)

They get downloaded to:

```
$ find ~/target/ -name managed_components | grep release
/home/lima/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-452cda0294ec9033/out/managed_components
```

In the `espressif__led_strip` subdirectory, there's the contents of the [source](https://github.com/espressif/idf-extra-components/tree/master/led_strip) (GitHub).

### `esp_idf_...` configurations within your app

As explained in [conditional compilation](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/#conditional-compilation) (`esp_idf_sys`), you can use Rust configs within your code.

- `cfg(esp_idf_comp_led_strip_enabled)`
- `cfg(esp_idf_version="5.5")`
- ...

If you do not have a use for such, the line in `build.rs` (or the whole file) can be let go.


## References

<!-- It's messy... 0.37.2
- [`esp_idf_sys`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/)
-->
