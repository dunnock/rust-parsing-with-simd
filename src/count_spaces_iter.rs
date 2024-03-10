#[inline(never)]
pub fn count_spaces_iter(buf: &[u8]) -> i64 {
    buf.iter()
        .map(|c| (*c == b' ' || *c == b'\n') as i64)
        .sum::<i64>()
}
