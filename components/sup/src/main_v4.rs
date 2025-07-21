#[cfg(feature = "v4")]
pub(crate) fn main_v4() {}

#[cfg(not(feature = "v4"))]
pub(crate) fn main_v4() {
    unreachable! {}
}
