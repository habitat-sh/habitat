macro_rules! auth_module (($auth_name:ident,
                           $verify_name:ident,
                           $keybytes:expr,
                           $tagbytes:expr) => (

use libc::c_ulonglong;
use randombytes::randombytes_into;
#[cfg(feature = "default")]
use rustc_serialize;

/// Number of bytes in a `Key`.
pub const KEYBYTES: usize = $keybytes;

/// Number of bytes in a `Tag`.
pub const TAGBYTES: usize = $tagbytes;

new_type! {
    /// Authentication `Key`
    ///
    /// When a `Key` goes out of scope its contents
    /// will be zeroed out
    secret Key(KEYBYTES);
}

new_type! {
    /// Authentication `Tag`
    ///
    /// The tag implements the traits `PartialEq` and `Eq` using constant-time
    /// comparison functions. See `sodiumoxide::utils::memcmp`
    public Tag(TAGBYTES);
}

/// `gen_key()` randomly generates a key for authentication
///
/// THREAD SAFETY: `gen_key()` is thread-safe provided that you have
/// called `sodiumoxide::init()` once before using any other function
/// from sodiumoxide.
pub fn gen_key() -> Key {
    let mut k = [0; KEYBYTES];
    randombytes_into(&mut k);
    Key(k)
}

/// `authenticate()` authenticates a message `m` using a secret key `k`.
/// The function returns an authenticator tag.
pub fn authenticate(m: &[u8],
                    &Key(ref k): &Key) -> Tag {
    unsafe {
        let mut tag = [0; TAGBYTES];
        $auth_name(&mut tag,
                   m.as_ptr(),
                   m.len() as c_ulonglong,
                   k);
        Tag(tag)
    }
}

/// `verify()` returns `true` if `tag` is a correct authenticator of message `m`
/// under a secret key `k`. Otherwise it returns false.
pub fn verify(&Tag(ref tag): &Tag, m: &[u8],
              &Key(ref k): &Key) -> bool {
    unsafe {
        $verify_name(tag,
                     m.as_ptr(),
                     m.len() as c_ulonglong,
                     k) == 0
    }
}

#[cfg(test)]
mod test_m {
    use super::*;

    #[test]
    fn test_auth_verify() {
        use randombytes::randombytes;
        for i in 0..256usize {
            let k = gen_key();
            let m = randombytes(i);
            let tag = authenticate(&m, &k);
            assert!(verify(&tag, &m, &k));
        }
    }

    #[test]
    fn test_auth_verify_tamper() {
        use randombytes::randombytes;
        for i in 0..32usize {
            let k = gen_key();
            let mut m = randombytes(i);
            let Tag(mut tagbuf) = authenticate(&mut m, &k);
            for j in 0..m.len() {
                m[j] ^= 0x20;
                assert!(!verify(&Tag(tagbuf), &mut m, &k));
                m[j] ^= 0x20;
            }
            for j in 0..tagbuf.len() {
                tagbuf[j] ^= 0x20;
                assert!(!verify(&Tag(tagbuf), &mut m, &k));
                tagbuf[j] ^= 0x20;
            }
        }
    }

    #[cfg(feature = "default")]
    #[test]
    fn test_serialisation() {
        use randombytes::randombytes;
        use test_utils::round_trip;
        for i in 0..256usize {
            let k = gen_key();
            let m = randombytes(i);
            let tag = authenticate(&m, &k);
            round_trip(k);
            round_trip(tag);
        }
    }
}

#[cfg(feature = "benchmarks")]
#[cfg(test)]
mod bench_m {
    extern crate test;
    use randombytes::randombytes;
    use super::*;

    const BENCH_SIZES: [usize; 14] = [0, 1, 2, 4, 8, 16, 32, 64,
                                      128, 256, 512, 1024, 2048, 4096];

    #[bench]
    fn bench_auth(b: &mut test::Bencher) {
        let k = gen_key();
        let ms: Vec<Vec<u8>> = BENCH_SIZES.iter().map(|s| {
            randombytes(*s)
        }).collect();
        b.iter(|| {
            for m in ms.iter() {
                authenticate(&m, &k);
            }
        });
    }

    #[bench]
    fn bench_verify(b: &mut test::Bencher) {
        let k = gen_key();
        let ms: Vec<Vec<u8>> = BENCH_SIZES.iter().map(|s| {
            randombytes(*s)
        }).collect();
        let tags: Vec<Tag> = ms.iter().map(|m| {
            authenticate(&m, &k)
        }).collect();
        b.iter(|| {
            for (m, t) in ms.iter().zip(tags.iter()) {
                verify(t, &m, &k);
            }
        });
    }
}

));

/// Macro for defining streaming authenticator tag computation types and functions.
///
/// Parameters:
/// $state_name - The authenticator state type.
///               SAFETY NOTE: This needs to be a type that does not define a `Drop`
///               implementation, otherwise undefined behaviour will occur.
/// $init_name - A function `f(s: *mut $state_name, k: *u8, klen: size_t)` that initializes
///              a state with a key.
/// $update_name - A function `f(s: *mut $state_name, m: *u8, mlen: size_t)` that updates
///                a state with a message chunk.
/// $final_name - A function `f(s: *mut $state_name, t: *u8)` that computes an authenticator                    tag of length $tagbytes from a $state_name.
/// $tagbytes   - The number of bytes in an authenticator tag.
macro_rules! auth_state (($state_name:ident,
                          $init_name:ident,
                          $update_name:ident,
                          $final_name:ident,
                          $tagbytes:expr) => (

use std::mem;
use ffi;

/// Authentication `State`
///
/// State for multi-part (streaming) authenticator tag (HMAC) computation.
///
/// When a `State` goes out of scope its contents will be zeroed out.
///
/// NOTE: the streaming interface takes variable length keys, as opposed to the
/// simple interface which takes a fixed length key. The streaming interface also does not
/// define its own `Key` type, instead using slices for its `init()` method.
/// The caller of the functions is responsible for zeroing out the key after it's been used
/// (in contrast to the simple interface which defines a `Drop` implementation for `Key`).
///
/// NOTE: these functions are specific to `libsodium` and do not exist in `NaCl`.

#[must_use]
pub struct State($state_name);

impl Drop for State {
    fn drop(&mut self) {
        let &mut State(ref mut s) = self;
        unsafe {
            let sp: *mut $state_name = s;
            ffi::sodium_memzero(sp as *mut u8, mem::size_of_val(s));
        }
    }
}

impl State {
    /// `init()` initializes an authentication structure using a secret key 'k'.
    pub fn init(k: &[u8]) -> State {
        unsafe {
            let mut s = mem::uninitialized();
            $init_name(&mut s, k.as_ptr(), k.len());
            State(s)
        }
    }

    /// `update()` can be called more than once in order to compute the authenticator
    /// from sequential chunks of the message.
    pub fn update(&mut self, in_: &[u8]) {
        let &mut State(ref mut state) = self;
        unsafe {
            $update_name(state, in_.as_ptr(), in_.len() as c_ulonglong);
        }
    }

    /// `finalize()` finalizes the authenticator computation and returns a `Tag`.
    pub fn finalize(&mut self) -> Tag {
        unsafe {
            let &mut State(ref mut state) = self;
            let mut tag = [0; $tagbytes as usize];
            $final_name(state, &mut tag);
            Tag(tag)
        }
    }
}

#[cfg(test)]
mod test_s {
    use super::*;

    #[test]
    fn test_auth_eq_auth_state() {
        use randombytes::randombytes;
        for i in 0..256usize {
            let k = gen_key();
            let m = randombytes(i);
            let tag = authenticate(&m, &k);
            let mut state = State::init(&k[..]);
            state.update(&m);
            let tag2 = state.finalize();
            assert_eq!(tag, tag2);
        }
    }

    #[test]
    fn test_auth_eq_auth_state_chunked() {
        use randombytes::randombytes;
        for i in 0..256usize {
            let k = gen_key();
            let m = randombytes(i);
            let tag = authenticate(&m, &k);
            let mut state = State::init(&k[..]);
            for c in m.chunks(1) {
                state.update(c);
            }
            let tag2 = state.finalize();
            assert_eq!(tag, tag2);
        }
    }
}
));
