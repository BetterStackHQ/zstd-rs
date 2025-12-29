//! WASM benchmark for measuring decompression performance.
//!
//! This benchmark can be run with or without SIMD to compare performance.
//!
//! ## Build Commands
//!
//! Build with SIMD:
//! ```bash
//! RUSTFLAGS="-Ctarget-feature=+simd128" cargo build -p wasm-bench \
//!     --target wasm32-unknown-unknown --release --features simd
//! ```
//!
//! Build without SIMD:
//! ```bash
//! cargo build -p wasm-bench --target wasm32-unknown-unknown --release
//! ```
//!
//! ## Generate JS bindings
//!
//! ```bash
//! wasm-bindgen target/wasm32-unknown-unknown/release/wasm_bench.wasm \
//!     --out-dir wasm-bench/pkg --target nodejs
//! ```

use ruzstd::decoding::FrameDecoder;
use wasm_bindgen::prelude::*;

// Large test file: 200MB uncompressed, ~1.4MB compressed
// Contains mix of RLE patterns, repeated sequences, and some random data
const LARGE_TEST_FILE: &[u8] = include_bytes!("../test_data_200mb.zst");

/// Returns the size of compressed test data in bytes
#[wasm_bindgen]
pub fn get_compressed_size() -> usize {
    LARGE_TEST_FILE.len()
}

/// Run decompression once and return the decompressed size in bytes
#[wasm_bindgen]
pub fn decompress_once() -> usize {
    let mut decoder = FrameDecoder::new();
    let mut output = vec![0u8; 210 * 1024 * 1024]; // 210MB buffer

    decoder.decode_all(LARGE_TEST_FILE, &mut output).unwrap()
}

/// Run the decompression benchmark for the specified number of iterations.
/// Returns the total bytes decompressed.
#[wasm_bindgen]
pub fn run_benchmark(iterations: u32) -> usize {
    let mut decoder = FrameDecoder::new();
    let mut output = vec![0u8; 210 * 1024 * 1024]; // 210MB buffer
    let mut total_bytes = 0;

    for _ in 0..iterations {
        let bytes = decoder.decode_all(LARGE_TEST_FILE, &mut output).unwrap();
        total_bytes += bytes;
    }

    total_bytes
}

/// Check if SIMD is enabled (compile-time check).
/// Returns true if compiled with WASM SIMD128 support.
#[wasm_bindgen]
pub fn is_simd_enabled() -> bool {
    cfg!(all(target_arch = "wasm32", target_feature = "simd128"))
}
