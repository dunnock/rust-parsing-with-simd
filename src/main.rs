pub mod count_spaces_iter;
mod count_word_avx;

use simd_parse::{btoi32, prepare_needle_4_256, simd_contains_4};

fn main() {
    let needle = prepare_needle_4_256(btoi32!(b"abcd"));
    let haystack = b"....abcd...abcd..abcd.abcd......abc";
    let res = simd_contains_4(needle, haystack);

    let rev_haystack = haystack
        .iter()
        .rev()
        .map(|b| *b as char)
        .collect::<String>();
    println!("{rev_haystack}");
    println!("   {res:032b}");
}
