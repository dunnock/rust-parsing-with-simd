use std::hint::black_box;

use simd_parse::{count_spaces_avx, count_spaces_iter};

const TIMES: usize = 100;

fn main() {
    let buf = std::fs::read(
        std::env::args()
            .nth(1)
            .expect("Expected input file as first argument"),
    )
    .expect("Failed to read input file");

    // Warmup
    println!(
        "Iter warmup completed {}",
        (0..100)
            .map(|_| black_box(count_spaces_iter(&buf)))
            .sum::<i64>()
    );

    // Regular iterator
    let ts = std::time::Instant::now();
    let spaces = (0..TIMES)
        .map(|_| black_box(count_spaces_iter(&buf)))
        .sum::<i64>()
        / TIMES as i64;
    let time = ts.elapsed().as_micros() as i64 / TIMES as i64;
    println!("Iter found {spaces} spaces in \x1b[36;1m{time}us\x1b[30;0m:");

    // SIMD AVX
    let ts = std::time::Instant::now();
    let spaces = (0..TIMES)
        .map(|_| unsafe { black_box(count_spaces_avx(&buf)) })
        .sum::<i64>()
        / TIMES as i64;
    let time = ts.elapsed().as_micros() as i64 / TIMES as i64;
    println!("AVX found {spaces} spaces in \x1b[36;1m{time}us\x1b[30;0m:");
}
