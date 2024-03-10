use std::hint::black_box;

use simd_parse::count_word_iter;

const TIMES: usize = 100;

fn main() {
    let buf = std::fs::read(
        std::env::args()
            .nth(1)
            .expect("Expected input file as first argument"),
    )
    .expect("Failed to read input file");
    let word = std::env::args()
        .nth(2)
        .expect("Expected word as second argument");

    // Warmup
    println!(
        "Iter warmup completed {}",
        (0..100)
            .map(|_| black_box(count_word_iter(&buf, word.as_bytes())))
            .sum::<i64>()
    );

    // Regular iterator
    let ts = std::time::Instant::now();
    let words = (0..TIMES)
        .map(|_| black_box(count_word_iter(&buf, word.as_bytes())))
        .sum::<i64>()
        / TIMES as i64;
    let time = ts.elapsed().as_micros() as i64 / TIMES as i64;
    println!("Iter found '{word}' {words} times in {time}us");
}
