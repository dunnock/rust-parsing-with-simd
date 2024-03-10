mod count_spaces_iter;
pub use count_spaces_iter::*;
mod count_spaces_avx;
pub use count_spaces_avx::*;
mod count_word_iter;
pub use count_word_iter::*;
mod count_word_avx;
pub use count_word_avx::*;

use std::arch::x86_64::{__m256i, _mm256_cmpeq_epi32, _mm256_loadu_si256, _mm256_movemask_epi8};

/// Broadcast (tile) 4 bytes to 32
///
/// abcd -> abcdabcd|abcdabcd|abcdabcd|abcdabcd
pub fn prepare_needle_4_256(needle: i32) -> __m256i {
    // see https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm256_set1_epi32&ig_expand=6406
    unsafe { std::arch::x86_64::_mm256_set1_epi32(needle) }
}

/// Search haystack by prepared mask (needle)
/// Notice haystack has size of 35 bytes to allow shifting the mask
/// Resulting i32 will contain bits set on matching haystack positions
///
/// mask: abcd | haystack: .....abcd...... ..
/// result: ..00000111100000b
pub fn simd_contains_4(needle_4_256: __m256i, haystack: &[u8; 35]) -> i32 {
    unsafe {
        let mut res = 0;
        for i in 0..=3 {
            let ptr = haystack.as_ptr().add(i);
            // Load 32 byte slice of haystack into YMM register
            let haystack_256 = _mm256_loadu_si256(ptr as *const __m256i);
            // Match all 32bit words in mask and haystack
            let cmp_mask = _mm256_cmpeq_epi32(haystack_256, needle_4_256);
            // Store resulting mask in the i32
            let mask = _mm256_movemask_epi8(cmp_mask) << i;
            res |= mask;
        }
        res
    }
}

/// Broadcast 4 byte array to i32 in const context
#[macro_export]
macro_rules! btoi32 {
    ($dword:literal) => {
        i32::from_le_bytes(*$dword)
    };
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
