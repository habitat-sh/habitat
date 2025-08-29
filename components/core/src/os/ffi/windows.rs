use std::ffi::OsStr;

const INVALID_UTF8: &str = "unexpected invalid UTF-8 code point";

pub trait OsStrExt3 {
    fn from_bytes(b: &[u8]) -> &Self;
    fn as_bytes(&self) -> &[u8];
}

impl OsStrExt3 for OsStr {
    // TODO JB: fix this allow
    #[allow(clippy::transmute_ptr_to_ptr)]
    fn from_bytes(b: &[u8]) -> &Self {
        use std::mem;
        unsafe { mem::transmute(b) }
    }

    fn as_bytes(&self) -> &[u8] { self.to_str().map(str::as_bytes).expect(INVALID_UTF8) }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct OsSplit<'a> {
    sep: u8,
    val: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for OsSplit<'a> {
    type Item = &'a OsStr;

    fn next(&mut self) -> Option<&'a OsStr> {
        if self.pos == self.val.len() {
            return None;
        }
        let start = self.pos;
        for b in &self.val[start..] {
            self.pos += 1;
            if *b == self.sep {
                return Some(OsStr::from_bytes(&self.val[start..self.pos - 1]));
            }
        }
        Some(OsStr::from_bytes(&self.val[start..]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut count = 0;
        for b in &self.val[self.pos..] {
            if *b == self.sep {
                count += 1;
            }
        }
        if count > 0 {
            return (count, Some(count));
        }
        (0, None)
    }
}

impl<'a> DoubleEndedIterator for OsSplit<'a> {
    fn next_back(&mut self) -> Option<&'a OsStr> {
        if self.pos == 0 {
            return None;
        }
        let start = self.pos;
        for b in self.val[..self.pos].iter().rev() {
            self.pos -= 1;
            if *b == self.sep {
                return Some(OsStr::from_bytes(&self.val[self.pos + 1..start]));
            }
        }
        Some(OsStr::from_bytes(&self.val[..start]))
    }
}
