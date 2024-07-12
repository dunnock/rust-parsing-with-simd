#[inline(never)]
pub fn count_spaces_memchr(buf: &[u8]) -> i64 {
    // There are also dedicated `memchr2` and `memchr2_iter` functions,
    // but they're not fast at counting, they're fast at finding the first match
    memchr::memchr_iter(b' ', buf).count() as i64 + memchr::memchr_iter(b'\n', buf).count() as i64
}
