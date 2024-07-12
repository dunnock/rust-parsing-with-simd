#[inline(never)]
pub fn count_word_memchr(buf: &[u8], word: &[u8]) -> i64 {
    memchr::memmem::find_iter(buf, word).count() as i64
}
