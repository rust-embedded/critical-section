#![no_std]
#![doc = include_str!("../README.md")]

pub use bare_metal::CriticalSection;

#[cfg(any(
    all(feature = "token-bool", feature = "token-u8"),
    all(feature = "token-bool", feature = "token-u16"),
    all(feature = "token-bool", feature = "token-u32"),
    all(feature = "token-bool", feature = "token-u64"),
    all(feature = "token-u8", feature = "token-u16"),
    all(feature = "token-u8", feature = "token-u32"),
    all(feature = "token-u8", feature = "token-u64"),
    all(feature = "token-u16", feature = "token-u32"),
    all(feature = "token-u16", feature = "token-u64"),
    all(feature = "token-u32", feature = "token-u64"),
))]
compile_error!("You must set at most one of these Cargo features: token-bool, token-u8, token-u16, token-u32, token-u64");

#[cfg(not(any(
    feature = "token-u8",
    feature = "token-u16",
    feature = "token-u32",
    feature = "token-u64"
)))]
pub type RawToken = bool;

#[cfg(feature = "token-u8")]
pub type RawToken = u8;

#[cfg(feature = "token-u16")]
pub type RawToken = u16;

#[cfg(feature = "token-u32")]
pub type RawToken = u32;

#[cfg(feature = "token-u64")]
pub type RawToken = u64;

pub struct Token(RawToken);

/// Acquire a critical section in the current thread.
///
/// This function is extremely low level. Strongly prefer using [`with`] instead.
///
/// Nesting critical sections is allowed. The inner critical sections
/// are mostly no-ops since they're already protected by the outer one.
///
/// # Safety
///
/// - Each `acquire` call must be paired with exactly one `release` call in the same thread.
/// - `acquire` returns a "restore token" that you must pass to the corresponding `release` call.
/// - `acquire`/`release` pairs must be "properly nested", ie it's not OK to do `a=acquire(); b=acquire(); release(a); release(b);`.
/// - It is UB to call `release` if the critical section is not acquired in the current thread.
/// - It is UB to call `release` with a restore token that does not come from the corresponding `acquire` call.
#[inline]
pub unsafe fn acquire() -> Token {
    extern "Rust" {
        fn _critical_section_acquire() -> RawToken;
    }

    Token(_critical_section_acquire())
}

/// Release the critical section.
///
/// This function is extremely low level. Strongly prefer using [`with`] instead.
///
/// # Safety
///
/// See [`acquire`] for the safety contract description.
#[inline]
pub unsafe fn release(token: Token) {
    extern "Rust" {
        fn _critical_section_release(token: RawToken);
    }
    _critical_section_release(token.0)
}

/// Execute closure `f` in a critical section.
///
/// Nesting critical sections is allowed. The inner critical sections
/// are mostly no-ops since they're already protected by the outer one.
#[inline]
pub fn with<R>(f: impl FnOnce(CriticalSection) -> R) -> R {
    unsafe {
        let token = acquire();
        let r = f(CriticalSection::new());
        release(token);
        r
    }
}

/// Methods required for a custom critical section implementation.
///
/// This trait is not intended to be used except when implementing a custom critical section.
///
/// # Safety
///
/// Implementations must uphold the contract specified in [`crate::acquire`] and [`crate::release`].
pub unsafe trait Impl {
    /// Acquire the critical section.
    unsafe fn acquire() -> RawToken;
    /// Release the critical section.
    unsafe fn release(token: RawToken);
}

/// Set the custom critical section implementation.
///
/// # Example
///
/// ```
/// use critical_section::RawToken;
///
/// struct MyCriticalSection;
/// critical_section::set_impl!(MyCriticalSection);
///
/// unsafe impl critical_section::Impl for MyCriticalSection {
///     unsafe fn acquire() -> RawToken {
///         // ...
///         # return false
///     }
///
///     unsafe fn release(token: RawToken) {
///         // ...
///     }
/// }
///
#[macro_export]
macro_rules! set_impl {
    ($t: ty) => {
        #[no_mangle]
        unsafe fn _critical_section_acquire() -> $crate::RawToken {
            <$t as $crate::Impl>::acquire()
        }
        #[no_mangle]
        unsafe fn _critical_section_release(token: $crate::RawToken) {
            <$t as $crate::Impl>::release(token)
        }
    };
}
