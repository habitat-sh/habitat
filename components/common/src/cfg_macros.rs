//! Common config macros
//!
//! There is quite a bit of functionality that is dependent upon the platform/os that we are
//! running on, the following config macros are used to conditionally compile such code. This is
//! inspired by `cfg_*` macros in `tokio`.

// Enable windows specific code
#[macro_export]
macro_rules! cfg_windows {

    ($($item:item)*) => {

        $(
            #[cfg(windows)]
            $item
        )*
    }
}

// Enable unix specific code
#[macro_export]
macro_rules! cfg_unix {

    ($($item:item)*) => {

        $(
            #[cfg(unix)]
            $item
        )*
    }
}
