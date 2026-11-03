# Working with ESP-IDF

If you are new to ESP-IDF, here are some "TL;DR" length notes to be aware of.

## `=n` does not mean "no"

You cannot block settings in a `sdkconfig.defaults` file. You can *request* that they not be included in the build (but other `sdkconfig.defaults` can override that).

## No fails

The ESP-IDF build prefers defaults and warnings in ambiguous situations. This can be very surprising to a Rust developer, since Rust prefers **explicit** and **failing** when something's wrong.

This affects for example:

- misspelled kConfig value pairs (those in the `sdkconfig.defaults`)

	The build churns on. However there *is* a warning about such things in the console output.

To see the console output:

```
$ cargo build --release -vv
```

## An external ESP-IDF component (`led_strip`)

They get downloaded to:

```
$ find ~/target/ -name managed_components | grep release
/home/lima/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-452cda0294ec9033/out/managed_components
```

In the `espressif__led_strip` subdirectory, there's the contents of the [source](https://github.com/espressif/idf-extra-components/tree/master/led_strip) (GitHub).
