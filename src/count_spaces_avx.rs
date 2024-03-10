use std::arch::x86_64::{__m256i, _mm256_cmpeq_epi8, _mm256_movemask_epi8, _mm256_or_si256};

#[inline(never)]
pub unsafe fn count_spaces_avx(buf: &[u8]) -> i64 {
    // First we are preparing search queries for SIMD
    // In this case SIMD will execute 32 comparisons as a single operation
    // text:    reve ta stogne dnipr shiroky
    // mask:    ____________________________
    // result:  0000100100000010000010000000
    let queries = [
        unsafe { std::arch::x86_64::_mm256_set1_epi8(' ' as i8) },
        unsafe { std::arch::x86_64::_mm256_set1_epi8('\n' as i8) },
    ];
    // In this task we sacrifice up to 32bytes of input string to get
    // slices of 256bit aligned memory
    let (_, search, _) = unsafe { buf.align_to::<__m256i>() };
    search
        .into_iter()
        .map(|slice| {
            // Match all 8bit words in mask and 256bit slice, this instruction takes 7 cycles
            let m1 = _mm256_cmpeq_epi8(*slice, queries[0]);
            // m1 is 32 bytes, where each byte is set to 0xFF where slice[i] == query[i]
            let m2 = _mm256_cmpeq_epi8(*slice, queries[1]);
            let m = _mm256_or_si256(m1, m2);
            // Following operation will extract highest bit from each byte returning u32
            // and then we count how many matches we got from 32 bytes in total
            _mm256_movemask_epi8(m).count_ones() as i64
        })
        .sum::<i64>()
}
