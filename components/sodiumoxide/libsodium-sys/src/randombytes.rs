// randombytes.h

extern {
    pub fn randombytes_buf(buf: *mut u8,
                           size: size_t);
}
