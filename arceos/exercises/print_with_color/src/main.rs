#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

/// Green + reset so `scripts/test-print.sh` sees `\x1b[` in qemu output.
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("{GREEN}[WithColor]: Hello, Arceos!{RESET}");
}
