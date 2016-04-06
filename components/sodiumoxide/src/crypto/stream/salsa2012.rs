//! `crypto_stream_salsa2012` (Salsa20/12), a particular cipher specified in
//! [Cryptography in NaCl](http://nacl.cr.yp.to/valid.html), Section 7.  This
//! cipher is conjectured to meet the standard notion of unpredictability.
use ffi::{crypto_stream_salsa2012,
          crypto_stream_salsa2012_xor,
          crypto_stream_salsa2012_KEYBYTES,
          crypto_stream_salsa2012_NONCEBYTES};

stream_module!(crypto_stream_salsa2012,
               crypto_stream_salsa2012_xor,
               crypto_stream_salsa2012_KEYBYTES,
               crypto_stream_salsa2012_NONCEBYTES);
