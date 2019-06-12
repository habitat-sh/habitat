//! System architecture representation.
//!
//! This module provides one primary type, [`PackageTarget`], which is the public representation of
//! a package (artifact, installed, or abstract) which has been built to run on a particular system
//! and architecture.
//!
//! A [`PackageTarget`] is similar in nature to [Rust's target triple][rust_triple] which comes
//! from [Clang's target triple][clang_triple], however we only require 2 pieces of information
//! from the triple concept:
//!
//! 1. The target architecture such as `x86_64`, `i386`, `armv7`, etc.
//! 2. The target system which maps to a generic concept of an operating  system, although more
//!    specifically the target kernel such as `linux`, `darwin`, `freebsd`, etc.
//!
//! An optional third piece of information, a variant, may be included in a package target as an
//! informational means of differentiating otherwise identical architecture/system combination
//! values.
//!
//! Internally, a [`PackageTarget`] is represented using the type system resulting in type safety
//! and static certainty while providing only one string representation for each type when
//! serializing and deserializing. So while at the edges of the system we may be dealing with
//! strings such as `x86_64-windows` in web requests, metadata files, etc., a small, safe
//! representation will be stored in memory and passed around.
//!
//! # Determining Package Target
//!
//! At build time, the build system will produce a Habitat artifact which is unconditionally
//! encoded with a specific package target by including a `TARGET` metafile in the root of the
//! package's installed directory. For convenience, the package target string representation is
//! also used in the naming of the so-called "hart" file (or Habitat ARTifact).
//!
//! After a package has been built, there are a few ways to check the [`PackageTarget`]:
//!
//! * Checking an artifact can be done by calling the [`target`][archive_target] method on
//! [`PackageArchive`].
//! * Checking that an installed package's target matches the active target is done automatically
//! when calling the [`load`][install_load] or [`load_at_least`][install_load_at_least] functions
//! on [`PackageInstall`].
//!
//! # A Special Note Concerning Variants
//!
//! The optional variant does **not** correspond to a `<vendor>` or `<abi>` as taken from a
//! traditional target triple (another reason why this concept is called a "package target" in
//! Habitat and not a "target triple"). In particular, the traditional understanding of [ABI]
//! doesn't fully apply where it relates to [C standard library][libc] implementations as multiple
//! libc implementations can live alongside each other within one package target set of packages.
//! For example, in the `x86_64-linux` package target there already exists a mature and default
//! [Glibc] toolchain and a minimal but effective [musl] toolchain which co-exist in the core
//! origin's package set. This fact however doesn't rule out future package targets which may use
//! the variant portion of a [`PackageTarget`] to represent custom or specific CPU/FPU support
//! which is distinct from the default, variant-less package target type.
//!
//! [archive_target]: ../archive/struct.PackageArchive.html#method.target
//! [install_load]: ../install/struct.PackageInstall.html#method.load
//! [install_load_at_least]: ../install/struct.PackageInstall.html#method.load_at_least
//! [`PackageArchive`]: ../archive/struct.PackageArchive.html
//! [`PackageInstall`]: ../install/struct.PackageInstall.html
//! [`PackageTarget`]: struct.PackageTarget.html
//! [ABI]: https://en.wikipedia.org/wiki/Application_binary_interface
//! [clang_triple]: http://clang.llvm.org/docs/CrossCompilation.html#target-triple
//! [Glibc]: https://www.gnu.org/software/libc/
//! [libc]: https://en.wikipedia.org/wiki/C_standard_library
//! [musl]: https://www.musl-libc.org/
//! [rust_triple]: https://github.com/rust-lang/rust/tree/master/src/librustc_back/target

use std::{fmt,
          ops::Deref,
          result,
          str::FromStr};

use regex::Regex;
use serde;

use crate::{error::Error,
            util};

macro_rules! supported_package_targets {
    (
        $(
            $(#[$docs:meta])*
            ($name:expr, $variant:ident, $konst:ident, $target_arch:expr, $target_os:expr);
        )+
    ) => {
        const SUPPORTED_PACKAGE_TARGETS: &'static [PackageTarget] = &[
            $(
                PackageTarget(Type::$variant),
            )+
        ];

        // Generates a public constant for each supported `PackageTarget`.
        $(
            $(#[$docs])*
            pub const $konst: PackageTarget = PackageTarget(Type::$variant);
        )+

        /// An internal representation of a target type, implemented as an `enum` with variants.
        ///
        /// # Representation
        ///
        /// `Type` represents all supported target types using an `enum`, and as such they will not
        /// require an allocation for storage (although the type size is still 1 byte in memory,
        /// used for the `enum` discriminant). The string representation for each variant is
        /// encapsulated in the `as_str` method and therefore can return string slices with a
        /// `static` lifetime (again, incurring no extra allocations). Each string representation
        /// is case sensitive and has only one valid string representation, so any consumer of this
        /// or any surrounding type may have to verify that the correct string case is used.
        #[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
        #[allow(non_camel_case_types)]
        enum Type {
            $(
                $(#[$docs])*
                $variant,
            )+
        }

        impl Type {
            /// Returns a string slice representing the underlying type.
            #[inline]
            fn as_str(&self) -> &'static str {
                match *self {
                    $(
                        Type::$variant => $name,
                    )+
                }
            }
        }

        impl FromStr for Type {
            type Err = Error;

            fn from_str(value: &str) -> result::Result<Self, Self::Err> {
                match value {
                    $(
                        $name => Ok(Type::$variant),
                    )+
                    _ => Err(Error::InvalidPackageTarget(String::from(value))),
                }
            }
        }

        /// Determines and returns the `PackageTarget` that is for the currently running system
        /// architecture.
        fn active_package_target() -> PackageTarget {
            // If a specific package target has been set at build time via an environment variable,
            // then use this value preferentially.
            if let Some(build_target) = option_env!("PLAN_PACKAGE_TARGET") {
                return PackageTarget::from_str(build_target).expect(&format!(
                        "PLAN_PACKAGE_TARGET provided value of {} \
                        could not be parsed as a PackageTarget",
                        build_target));
            }

            // Each supported package target is checked in turn and the resulting target is early
            // returned on first match. This is done with disconnected `if` expressions to make the
            // macro generation easier.
            $(
                if cfg!(target_arch = $target_arch) && cfg!(target_os = $target_os) {
                    return PackageTarget(Type::$variant);
                }
            )+

            // If none of the above conditionals match, then we fail hard
            unreachable!(
                "Current binary is being built for an unknown system architecture. \
                 Current compiletime supported package targets are: [{}]. \
                 If you see a supported package target but still see this message, then \
                 the PackageTarget::active_package_target() function needs to be updated.",
                SUPPORTED_PACKAGE_TARGETS
                    .iter()
                    .map(|t| t.0.as_str())
                    .collect::<Vec<&str>>()
                    .join(", ")
            )
        }

        // Only used in tests to iterate through all possible supported types
        #[cfg(test)]
        const TEST_TYPES: &'static [(Type, &'static str)] = &[
            $(
                (Type::$variant, $name),
            )+
        ];

        #[test]
        fn test_active_package_target_is_supported() {
            $(
                if cfg!(target_arch = $target_arch) && cfg!(target_os = $target_os) {
                    let active = active_package_target();
                    println!("Active package target is: '{}'", &active);
                    assert_eq!(PackageTarget(Type::$variant), active);
                    // Quick return on first matched test arm
                    return;
                }
            )+

            panic!("Active package target is not supported on this system architecture");
        }

        #[test]
        fn test_all_types_as_str() {
            for &(typ, name) in TEST_TYPES {
                assert_eq!(name, typ.as_str());
            }
        }

        #[test]
        fn test_all_types_from_str() {
            for &(typ, name) in TEST_TYPES {
                assert_eq!(typ, Type::from_str(name).unwrap());
            }
        }
    }
}

// Generates an `enum` called `Type` which has a variant for each and every explicitly supported
// package target type. A public constant for each supported target is created containing full
// documentation. An internal constant is also generated containing all supported `PackageTarget`
// types. This constant is exposed via a `PackageTarget::supported_targets` function. Finally, a
// function called `active_package_target` is also generated which determines the package target
// for the compiled version of this code.
//
// Adding a new target entry below will allow new package targets to be supported by any code
// consuming this crate.
//
// The structure for each target entry is as follows:
//
// 1. The string representation of the target type which will be used to derive the `as_str` and
//    `from_str` implementations. The format of this string must follow the structure outlined in
//    this module's documentation.
// 2. The Rust variant identifier which will be used in `Type`. For example, `Type::X86_64_Linux`.
// 3. The Rust constant identifier which will be used to construct a public constant for each
//    supported target.
// 4. The Rust string value to be used in the target's `cfg!(target_arch = "<arch>")` macro. This
//    will be used to conditionally compile the correct and appropriate active package target at
//    compile time.
// 5. The Rust string value to be used in the target's `cfg!(target_os = "<arch>")` macro. This
//    will be used to conditionally compile the correct and appropriate active package target at
//    compile time.
//
// Note that some package targets may overlap on the same underlying `target_arch`/`target_os`
// system. These are still distinct and exclusive package targets and will operate in isolation as
// much as possible if more than one installed package target is present on the same system. Again,
// the third and fourth values are used by the Rust compiler at build time and never exposed in
// code at runtime.
supported_package_targets! {
    /// Represents a [XNU kernel]-based system (more commonly referred to as [Darwin] or [macOS])
    /// running on a [64-bit] version of the [x86][x] [instruction set architecture][isa], commonly
    /// known as [x86_64].
    ///
    /// [XNU kernel]: https://en.wikipedia.org/wiki/XNU
    /// [Darwin]: https://en.wikipedia.org/wiki/Darwin_(operating_system)
    /// [macOS]: https://en.wikipedia.org/wiki/MacOS
    /// [64-bit]: https://en.wikipedia.org/wiki/64-bit_computing
    /// [x]: https://en.wikipedia.org/wiki/X86
    /// [isa]: https://en.wikipedia.org/wiki/Instruction_set_architecture
    /// [x86_64]: https://en.wikipedia.org/wiki/X86-64
    ("x86_64-darwin", X86_64_Darwin, X86_64_DARWIN, "x86_64", "macos");

    /// Represents a [Linux kernel]-based system running on a [64-bit] version of the [x86][x]
    /// [instruction set architecture][isa], commonly known as [x86_64].
    ///
    /// [Linux kernel]: https://en.wikipedia.org/wiki/Linux_kernel
    /// [64-bit]: https://en.wikipedia.org/wiki/64-bit_computing
    /// [x]: https://en.wikipedia.org/wiki/X86
    /// [isa]: https://en.wikipedia.org/wiki/Instruction_set_architecture
    /// [x86_64]: https://en.wikipedia.org/wiki/X86-64
    ("x86_64-linux", X86_64_Linux, X86_64_LINUX, "x86_64", "linux");

    /// Represents an **older** [Linux kernel]-based system from the 2.6.x family running on a
    /// [64-bit] version of the [x86][x] [instruction set architecture][isa], commonly known as
    /// [x86_64].
    ///
    /// This Habitat package target is intended for software with older buildtime or runtime
    /// requirements than those supported by the current `x86_64-linux` package target.
    /// Specifically, software with this package target should run on systems from around the year
    /// 2016, when the Habitat project was publicly launched. This Habitat package target will run
    /// on Linux systems equipped with kernel versions as low as 2.6.32.
    ///
    /// [Linux kernel]: https://en.wikipedia.org/wiki/Linux_kernel
    /// [64-bit]: https://en.wikipedia.org/wiki/64-bit_computing
    /// [x]: https://en.wikipedia.org/wiki/X86
    /// [isa]: https://en.wikipedia.org/wiki/Instruction_set_architecture
    /// [x86_64]: https://en.wikipedia.org/wiki/X86-64
    ("x86_64-linux-kernel2", X86_64_Linux_Kernel2, X86_64_LINUX_KERNEL2, "x86_64", "linux");

    /// Represents a [Windows kernel]-based system running on a [64-bit] version of the [x86][x]
    /// [instruction set architecture][isa], commonly known as [x86_64].
    ///
    /// [Windows kernel]: https://en.wikipedia.org/wiki/Architecture_of_Windows_NT
    /// [64-bit]: https://en.wikipedia.org/wiki/64-bit_computing
    /// [x]: https://en.wikipedia.org/wiki/X86
    /// [isa]: https://en.wikipedia.org/wiki/Instruction_set_architecture
    /// [x86_64]: https://en.wikipedia.org/wiki/X86-64
    ("x86_64-windows", X86_64_Windows, X86_64_WINDOWS, "x86_64", "windows");
}

lazy_static::lazy_static! {
    /// A compiled regular expression that can parse the internal components of a `Type`.
    static ref TYPE_FROM_STR_RE: Regex = Regex::new(
        r"\A(?P<architecture>[a-z0-9_]+)-(?P<system>[a-z0-9_]+)(-(?P<variant>[a-z0-9_]+))?\z"
    ).unwrap();

    /// The `PackageTarget` that is determined at compile time for the currently running system
    /// architecture.
    static ref ACTIVE_PACKAGE_TARGET: PackageTarget = active_package_target();
}

/// Represents a specific system architecture.
///
/// More details about the overall approach can be found in the [module documentation](index.html).
///
/// # Examples
///
/// ```
/// use habitat_core::package::target::{self,
///                                     PackageTarget};
/// use std::str::FromStr;
///
/// // Package target implements the `FromStr` trait and thus can be parsed from strings
/// let target = PackageTarget::from_str("x86_64-linux").unwrap();
///
/// // The `AsRef` trait is implemented to easily return an `&str` representation
/// assert_eq!("x86_64-linux", target.as_ref());
///
/// // The `Deref` trait is also implemented
/// assert_eq!("x86_64-linux", &*target);
///
/// // A module constant also exists for each target
/// assert_eq!(target::X86_64_LINUX, target);
/// ```
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct PackageTarget(Type);

impl PackageTarget {
    /// Produces an iterator over the target's internal components viewed as [`&str`] slices.
    ///
    /// Note that no special interpretation should be taken from the component slices as their
    /// meaning is internal to this struct's implementation.
    ///
    /// [`&str`]: https://doc.rust-lang.org/std/primitive.str.html
    ///
    /// # Examples
    ///
    /// ```
    /// use habitat_core::package::target;
    ///
    /// let mut it = target::X86_64_LINUX.iter();
    ///
    /// assert_eq!(it.next(), Some("x86_64"));
    /// assert_eq!(it.next(), Some("linux"));
    /// assert_eq!(it.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_> {
        Iter { target: self,
               pos:    0, }
    }

    /// Returns the `PackageTarget` that is determined at compile time for the currently running
    /// system architecture.
    ///
    /// This can be used to compare a [`PackageArchive`] or [`PackageInstall`]'s type with the
    /// currently supported version when this code is compiled.
    ///
    /// [`PackageArchive`]: ../archive/struct.PackageArchive.html
    /// [`PackageInstall`]: ../install/struct.PackageInstall.html
    ///
    /// # Examples
    ///
    /// ```
    /// use habitat_core::package::PackageTarget;
    ///
    /// let active = PackageTarget::active_target();
    /// println!("The active target for this system is '{}'", active);
    /// ```
    pub fn active_target() -> Self { *ACTIVE_PACKAGE_TARGET }

    /// Produces an iterator over all supported `PackageTarget`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use habitat_core::package::PackageTarget;
    ///
    /// // The iterator allows the caller to use the result directly in a loop
    /// for target in PackageTarget::supported_targets() {
    ///     println!("Supported target: {}", target);
    /// }
    ///
    /// // Alternatively, the iterator can be chained to perform more sophisticated
    /// // transformations
    /// let targets: Vec<_> = PackageTarget::supported_targets().map(|t| t.as_ref())
    ///                                                         .collect();
    /// println!("All supported targets: [{}]", targets.join(", "));
    /// ```
    pub fn supported_targets() -> ::std::slice::Iter<'static, PackageTarget> {
        SUPPORTED_PACKAGE_TARGETS.iter()
    }
}

impl fmt::Display for PackageTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0.as_str()) }
}

impl FromStr for PackageTarget {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        Ok(PackageTarget(Type::from_str(value)?))
    }
}

impl Deref for PackageTarget {
    type Target = str;

    fn deref(&self) -> &'static str { self.0.as_str() }
}

impl AsRef<str> for PackageTarget {
    fn as_ref(&self) -> &str { self.0.as_str() }
}

impl<'d> serde::Deserialize<'d> for PackageTarget {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: serde::Deserializer<'d>
    {
        util::serde_string::deserialize(deserializer)
    }
}

impl serde::Serialize for PackageTarget {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_str(self.0.as_str())
    }
}

impl Type {
    /// Returns the architecture component of the underlying target type.
    fn architecture(&self) -> &str {
        TYPE_FROM_STR_RE.captures(self.as_str())
                        .unwrap()
                        .name("architecture")
                        .unwrap()
                        .as_str()
    }

    /// Returns the system component of the underlying target type.
    fn system(&self) -> &str {
        TYPE_FROM_STR_RE.captures(self.as_str())
                        .unwrap()
                        .name("system")
                        .unwrap()
                        .as_str()
    }

    /// Returns the variant component of the underlying target type, if one is present.
    fn variant(self) -> Option<&'static str> {
        TYPE_FROM_STR_RE.captures(self.as_str())
                        .unwrap()
                        .name("variant")
                        .and_then(|v| Some(v.as_str()))
    }
}

/// An iterator over the [`&str`] slices of a [`PackageTarget`].
///
/// This `struct` is created by the [`iter`] method on [`PackageTarget`], see its documentation for
/// more.
///
/// [`&str`]: https://doc.rust-lang.org/std/primitive.str.html
/// [`iter`]: struct.PackageTarget.html#method.iter
/// [`PackageTarget`]: struct.PackageTarget.html
///
/// # Examples
///
/// ```
/// use habitat_core::package::target;
/// use std::str::FromStr;
///
/// let target = target::X86_64_LINUX;
///
/// for component in target.iter() {
///     println!("{}", component);
/// }
/// ```
pub struct Iter<'a> {
    target: &'a PackageTarget,
    pos:    usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.pos += 1;
        match self.pos {
            // The first component is the architecture
            1 => Some(self.target.0.architecture()),
            // The second component is the system
            2 => Some(self.target.0.system()),
            // The third component is optional and corresponds to a target variant
            3 => self.target.0.variant(),
            // All components optional and otherwise have been visited
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use serde_derive::{Deserialize,
                       Serialize};
    use toml;

    // This test explicitly runs the function which returns the active `PackageTarget` for the
    // binary on the current running system. If compiletime support has not yet been added for this
    // running system, this test will fail with a message that can be read by adding
    // `-- --nocapture` to `cargo test`.
    #[test]
    fn active_pacakge_target_returns_valid_value() {
        println!("Active package target is: '{}'", active_package_target());
    }

    // The `Type::from_str()` implementation is already tested for every enum variant, so this test
    // only asserts that the `FromStr` implementation is plumbed through to the `PackageTarget`
    // wrapping type's API.
    #[test]
    fn package_target_from_str() {
        assert_eq!(PackageTarget(Type::X86_64_Linux),
                   PackageTarget::from_str("x86_64-linux").unwrap());
    }

    // The `Type::as_str()` implementation is already tested for every enum variant, so this
    // test only asserts that the `Display` implementation is plumbed through to the
    // `PackageTarget` wrapping type's API.
    #[test]
    fn package_target_to_string() {
        let target = PackageTarget(Type::X86_64_Darwin);
        assert_eq!("x86_64-darwin", target.to_string());
    }

    #[test]
    fn package_target_as_ref() {
        let target = PackageTarget(Type::X86_64_Linux);
        assert_eq!("x86_64-linux", target.as_ref());
    }

    #[test]
    fn serialize() {
        #[derive(Serialize)]
        struct Data {
            target: PackageTarget,
        }
        let data = Data { target: PackageTarget(Type::X86_64_Linux), };
        let toml = toml::to_string(&data).unwrap();

        assert!(toml.starts_with(r#"target = "x86_64-linux""#));
    }

    #[test]
    fn deserialize() {
        #[derive(Deserialize)]
        struct Data {
            target: PackageTarget,
        }
        let toml = r#"
            target = "x86_64-windows"
            "#;
        let data: Data = toml::from_str(toml).unwrap();

        assert_eq!(data.target, PackageTarget(Type::X86_64_Windows));
    }

    #[test]
    fn type_architecture() {
        assert_eq!("x86_64", Type::X86_64_Linux.architecture());
    }

    #[test]
    fn type_system() {
        assert_eq!("darwin", Type::X86_64_Darwin.system());
    }

    #[test]
    fn type_variant() {
        assert_eq!(Some("kernel2"), Type::X86_64_Linux_Kernel2.variant());
    }

    #[test]
    fn type_no_variant() {
        assert_eq!(None, Type::X86_64_Windows.variant());
    }

    #[test]
    fn package_target_iter_no_variant() {
        let target = PackageTarget(Type::X86_64_Windows);
        let mut iter = target.iter();

        assert_eq!(Some("x86_64"), iter.next());
        assert_eq!(Some("windows"), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn package_target_iter_with_variant() {
        let target = PackageTarget(Type::X86_64_Linux_Kernel2);
        let mut iter = target.iter();

        assert_eq!(Some("x86_64"), iter.next());
        assert_eq!(Some("linux"), iter.next());
        assert_eq!(Some("kernel2"), iter.next());
        assert_eq!(None, iter.next());
    }
}
