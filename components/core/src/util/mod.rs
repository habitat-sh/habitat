#[cfg(not(windows))]
pub mod posix_perm;
pub mod serde_string;
pub mod sys;
#[cfg(windows)]
pub mod win_perm;

use std::{mem,
          time::Duration};

/// Provide a way to convert numeric types safely to i64
pub trait ToI64 {
    fn to_i64(self) -> i64;
}

impl ToI64 for usize {
    fn to_i64(self) -> i64 {
        if mem::size_of::<usize>() >= mem::size_of::<i64>() && self > i64::max_value() as usize {
            if cfg!(debug_assertions) {
                panic!("Tried to convert an out-of-range usize ({}) to i64", self);
            } else {
                error!("Tried to convert an out-of-range usize ({}) to i64; using \
                        i64::max_value()",
                       self);
                i64::max_value()
            }
        } else {
            self as i64
        }
    }
}

impl ToI64 for u64 {
    fn to_i64(self) -> i64 {
        if self > i64::max_value() as u64 {
            if cfg!(debug_assertions) {
                panic!("Tried to convert an out-of-range u64 ({}) to i64", self);
            } else {
                error!("Tried to convert an out-of-range u64 ({}) to i64; using i64::max_value()",
                       self);
                i64::max_value()
            }
        } else {
            self as i64
        }
    }
}

pub fn wait_for(delay: Duration, times: usize) -> impl IntoIterator<Item = Duration> {
    vec![delay].into_iter().cycle().take(times)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_of_usize_to_i64() {
        let just_right: usize = 42;
        let zero: usize = 0;

        assert_eq!(just_right.to_i64(), 42);
        assert_eq!(zero.to_i64(), 0);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn conversion_of_too_big_usize_panics_in_debug_mode() {
        let too_big = usize::max_value();
        too_big.to_i64();
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn conversion_of_too_big_usize_caps_in_release_mode() {
        let too_big = usize::max_value();
        assert_eq!(too_big.to_i64(), i64::max_value());
    }

    #[test]
    fn conversion_of_u64_to_i64() {
        let just_right: u64 = 42;
        let zero: u64 = 0;

        assert_eq!(just_right.to_i64(), 42);
        assert_eq!(zero.to_i64(), 0);
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn conversion_of_too_big_u64_panics_in_debug_mode() {
        let too_big = u64::max_value();
        too_big.to_i64();
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn conversion_of_too_big_u64_caps_in_release_mode() {
        let too_big = u64::max_value();
        assert_eq!(too_big.to_i64(), i64::max_value());
    }
}
