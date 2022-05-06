#![no_std]
#![doc = include_str!("../README.md")]

pub use bare_metal::CriticalSection;

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
/// - `acquire` returns a "restore token" `u8` that you must pass to the corresponding `release` call, and treat opaquely otherwise.
/// - `acquire`/`release` pairs must be "properly nested", ie it's not OK to do `a=acquire(); b=acquire(); release(a); release(b);`.
/// - It is UB to call `release` if the critical section is not acquired in the current thread.
/// - It is UB to call `release` with a restore token that does not come from the corresponding `acquire` call.
#[inline]
pub unsafe fn acquire() -> u8 {
    extern "Rust" {
        fn _critical_section_acquire() -> u8;
    }

    _critical_section_acquire()
}

/// Release the critical section.
///
/// This function is extremely low level. Strongly prefer using [`with`] instead.
///
/// # Safety
///
/// See [`acquire`] for the safety contract description.
#[inline]
pub unsafe fn release(token: u8) {
    extern "Rust" {
        fn _critical_section_release(token: u8);
    }
    _critical_section_release(token)
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
/// Implementations must uphold the contract specified in [`crate::acquire`] and [`crate::release`].
pub unsafe trait Impl {
    /// Acquire the critical section.
    unsafe fn acquire() -> u8;
    /// Release the critical section.
    unsafe fn release(token: u8);
}

/// Set the custom critical section implementation.
///
/// # Example
///
/// ```
/// struct MyCriticalSection;
/// critical_section::custom_impl!(MyCriticalSection);
///
/// unsafe impl critical_section::Impl for MyCriticalSection {
///     unsafe fn acquire() -> u8 {
///         // ...
///         # return 0
///     }
///
///     unsafe fn release(token: u8) {
///         // ...
///     }
/// }
///
#[macro_export]
macro_rules! set_impl {
    ($t: ty) => {
        #[no_mangle]
        unsafe fn _critical_section_acquire() -> u8 {
            <$t as $crate::Impl>::acquire()
        }
        #[no_mangle]
        unsafe fn _critical_section_release(token: u8) {
            <$t as $crate::Impl>::release(token)
        }
    };
}
