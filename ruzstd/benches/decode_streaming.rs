use criterion::{criterion_group, criterion_main, Criterion};
use ruzstd::decoding::FrameDecoder;

fn criterion_benchmark(c: &mut Criterion) {
    let src = include_bytes!("../decodecorpus_files/z000033.zst");

    // Single reusable output buffer for receiving decompressed chunks
    let mut output_buffer = vec![0u8; 256 * 1024];
    let mut leftover = Vec::with_capacity(128 * 1024);

    c.bench_function("decode_streaming_4k_input", |b| {
        b.iter(|| {
            // Fresh decoder for each stream (like ZstdStream::new() does)
            let mut fr = FrameDecoder::new();
            let mut total_decoded = 0usize;
            leftover.clear();

            // Process compressed input in 4K chunks (simulating network arrival)
            for chunk in src.chunks(4096) {
                // Accumulate input like ZstdStream does
                leftover.extend_from_slice(chunk);

                loop {
                    // decode_from_to: feed compressed data, get decompressed output
                    let (bytes_read, bytes_written) =
                        fr.decode_from_to(&leftover, &mut output_buffer).unwrap();

                    total_decoded += bytes_written;

                    // Collect any remaining buffered data
                    if let Some(collected) = fr.collect() {
                        total_decoded += collected.len();
                    }

                    // Remove consumed bytes
                    if bytes_read > 0 {
                        leftover.drain(..bytes_read);
                    }

                    // If no progress, need more input
                    if bytes_read == 0 && bytes_written == 0 {
                        break;
                    }

                    if fr.is_finished() {
                        // Drain any final data
                        while fr.can_collect() > 0 {
                            if let Some(collected) = fr.collect() {
                                total_decoded += collected.len();
                            }
                        }
                        return total_decoded;
                    }
                }
            }

            total_decoded
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
