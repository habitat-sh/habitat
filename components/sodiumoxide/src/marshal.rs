use libc::c_ulonglong;

#[doc(hidden)]
pub fn marshal<T, F>(buf: &[u8],
                     padbefore: usize,
                     bytestodrop: usize,
                     f: F
                     ) -> (Vec<u8>, T)
    where F: Fn(*mut u8, *const u8, c_ulonglong) -> T {
    let mut dst = Vec::with_capacity(buf.len() + padbefore);
    for _ in 0..padbefore {
        dst.push(0);
    }
    dst.extend(buf.into_iter());
    let pdst = dst.as_mut_ptr();
    let psrc = dst.as_ptr();
    let res = f(pdst, psrc, dst.len() as c_ulonglong);
    (dst.into_iter().skip(bytestodrop).collect(), res)
}
