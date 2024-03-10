use std::arch::x86_64::{
    __m256i, _mm256_cmpeq_epi32, _mm256_loadu_si256, _mm256_movemask_epi8, _mm256_set1_epi32,
};

/// Search haystack by prepared mask (needle)
/// Notice haystack has size of 35 bytes to allow shifting the mask
/// Resulting i32 will contain bits set on matching haystack positions
///
/// mask: abcd | haystack: .....abcd...... ..
/// result: ..00000111100000b
unsafe fn simd_contains_4(query_i32_256: __m256i, buf: *const u8) -> u32 {
    let mut res = 0;
    for i in 0..=3 {
        // Load 32 byte slice of haystack into YMM register
        let haystack_256 = _mm256_loadu_si256(buf.add(i) as *const __m256i);
        // Match all 32bit words in mask and haystack
        let cmp_mask = _mm256_cmpeq_epi32(haystack_256, query_i32_256);
        // Store resulting mask in the u32
        let mask = _mm256_movemask_epi8(cmp_mask) << i;
        res |= mask;
    }
    std::mem::transmute(res)
}

#[inline(never)]
pub unsafe fn count_word_avx(buf: &[u8], word: &[u8]) -> i64 {
    assert!(
        word.len() > 4,
        "Only words of size 4+ supported in this example for simplification"
    );
    // First we are preparing search query with first four letters for SIMD
    // In this case SIMD will execute 8 comparisons as a single operation
    // text:    reve ta stogne dnipr shiroky
    // mask:    reverevereverevereverevereve
    // result:  1111000000000000000000000000
    let first4b: [u8; 4] = word[..4].try_into().unwrap();
    let query = _mm256_set1_epi32(i32::from_le_bytes(first4b));

    buf.windows(32 + word.len())
        .step_by(32)
        .map(|s| {
            let mut mask = simd_contains_4(query, s.as_ptr());
            let mut count = 0;
            let mut pos: usize = 0;
            while mask != 0 {
                let idx = mask.trailing_zeros();
                pos += idx as usize + 4;
                if s[pos..].starts_with(&word[4..]) {
                    count += 1;
                }
                mask = (mask >> idx) >> 4;
            }
            count
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn findword() {
        let buf = r#"
    reve ta stogne dnipr shyroky,
    serdity viter zavyva,
    dodolu verby gne vysoki,
    goramy hvylyu pidiyma.

    dodolu verby gne vysoki,
    goramy hvylyu pidima.

    I blidy misyats na tu poru
    iz hmary de de vyglyadav,
    nenache choven v synim mori,
    to vyrynav to potopav."#;

        assert_eq!(unsafe { count_word_avx(buf.as_bytes(), b"dodolu") }, 2);
        assert_eq!(unsafe { count_word_avx(buf.as_bytes(), b"blidy") }, 1);
        assert_eq!(unsafe { count_word_avx(buf.as_bytes(), b"zavyva") }, 1);
        assert_eq!(unsafe { count_word_avx(buf.as_bytes(), b"goramy") }, 2);
        assert_eq!(unsafe { count_word_avx(buf.as_bytes(), b"hvylyu") }, 2);
    }
}
