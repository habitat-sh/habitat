//! Libsodium utility functions
use ffi;

/// `memzero()` tries to effectively zero out the data in `x` even if
/// optimizations are being applied to the code.
pub fn memzero(x: &mut [u8]) {
    unsafe {
        ffi::sodium_memzero(x.as_mut_ptr(), x.len());
    }
}

/// `memcmp()` returns true if `x[0]`, `x[1]`, ..., `x[len-1]` are the
/// same as `y[0]`, `y[1]`, ..., `y[len-1]`. Otherwise it returns `false`.
///
/// This function is safe to use for secrets `x[0]`, `x[1]`, ..., `x[len-1]`,
/// `y[0]`, `y[1]`, ..., `y[len-1]`. The time taken by `memcmp` is independent
/// of the contents of `x[0]`, `x[1]`, ..., `x[len-1]`, `y[0]`, `y[1]`, ..., `y[len-1]`.
/// In contrast, the standard C comparison function `memcmp(x,y,len)` takes time
/// that depends on the longest matching prefix of `x` and `y`, often allowing easy
/// timing attacks.
pub fn memcmp(x: &[u8], y: &[u8]) -> bool {
    if x.len() != y.len() {
        return false
    }
    unsafe {
        ffi::sodium_memcmp(x.as_ptr(), y.as_ptr(), x.len()) == 0
    }
}

/// `increment_le()` treats `x` as an unsigned little-endian number and increments it.
///
/// WARNING: this method does not check for arithmetic overflow. When used for incrementing
/// nonces it is the callers responsibility to ensure that any given nonce value
/// is only used once.
/// If the caller does not do that the cryptographic primitives in sodiumoxide
/// will not uphold any security guarantees (i.e. they will break)
pub fn increment_le(x: &mut [u8]) {
    unsafe {
        ffi::sodium_increment(x.as_mut_ptr(), x.len());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_memcmp() {
        use randombytes::randombytes;

        for i in 0usize..256 {
            let x = randombytes(i);
            assert!(memcmp(&x, &x));
            let mut y = x.clone();
            assert!(memcmp(&x, &y));
            y.push(0);
            assert!(!memcmp(&x, &y));
            assert!(!memcmp(&y, &x));

            y = randombytes(i);
            if x == y {
                assert!(memcmp(&x, &y))
            } else {
                assert!(!memcmp(&x, &y))
            }
        }
    }

    #[test]
    fn test_increment_le_zero() {
        for i in 1usize..256 {
            let mut x = vec!(0u8; i);
            increment_le(&mut x);
            assert!(!x.iter().all(|x| { *x == 0 }));
            let mut y = vec!(0u8; i);
            y[0] += 1;
            assert_eq!(x, y);
        }
    }

    #[test]
    fn test_increment_le_vectors() {
        let mut x = [255, 2, 3, 4, 5];
        let y = [0, 3, 3, 4, 5];
        increment_le(&mut x);
        assert!(!x.iter().all(|x| { *x == 0 }));
        assert_eq!(x, y);
        let mut x = [255, 255, 3, 4, 5];
        let y = [0, 0, 4, 4, 5];
        increment_le(&mut x);
        assert!(!x.iter().all(|x| { *x == 0 }));
        assert_eq!(x, y);
        let mut x = [255, 255, 255, 4, 5];
        let y = [0, 0, 0, 5, 5];
        increment_le(&mut x);
        assert!(!x.iter().all(|x| { *x == 0 }));
        assert_eq!(x, y);
        let mut x = [255, 255, 255, 255, 5];
        let y = [0, 0, 0, 0, 6];
        increment_le(&mut x);
        assert!(!x.iter().all(|x| { *x == 0 }));
        assert_eq!(x, y);
        let mut x = [255, 255, 255, 255, 255];
        let y = [0, 0, 0, 0, 0];
        increment_le(&mut x);
        assert!(x.iter().all(|x| { *x == 0 }));
        assert_eq!(x, y);
    }

    #[test]
    fn test_increment_le_overflow() {
        for i in 1usize..256 {
            let mut x = vec!(255u8; i);
            increment_le(&mut x);
            assert!(x.iter().all(|xi| { *xi == 0 }));
        }
    }
}
