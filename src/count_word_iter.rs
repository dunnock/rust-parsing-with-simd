#[inline(never)]
pub fn count_word_iter(buf: &[u8], word: &[u8]) -> i64 {
    buf.windows(word.len())
        .map(|s| (s == word) as i64)
        .sum::<i64>()
}
