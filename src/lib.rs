#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]
#![doc = include_str!("../README.md")]

pub use bare_metal::CriticalSection;

use critical_section_1::RawRestoreState;

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
#[allow(clippy::unit_arg)]
#[inline]
pub unsafe fn acquire() -> u8 {
    extern "Rust" {
        fn _critical_section_1_0_acquire() -> critical_section_1::RawRestoreState;
    }
    <RawRestoreState as ConvertRestoreState>::to_u8(_critical_section_1_0_acquire())
}

/// Release the critical section.
///
/// This function is extremely low level. Strongly prefer using [`with`] instead.
///
/// # Safety
///
/// See [`acquire`] for the safety contract description.
#[allow(clippy::unit_arg)]
#[inline]
pub unsafe fn release(token: u8) {
    extern "Rust" {
        fn _critical_section_1_0_release(restore_state: critical_section_1::RawRestoreState);
    }
    _critical_section_1_0_release(<RawRestoreState as ConvertRestoreState>::from_u8(token));
}

/// Execute closure `f` in a critical section.
///
/// Nesting critical sections is allowed. The inner critical sections
/// are mostly no-ops since they're already protected by the outer one.
#[inline]
pub fn with<R>(f: impl FnOnce(CriticalSection) -> R) -> R {
    critical_section_1::with(|_| f(unsafe { CriticalSection::new() }))
}

// Extension trait which implements conversions between RestoreState and u8, if possible
trait ConvertRestoreState {
    fn to_u8(self) -> u8;
    fn from_u8(state: u8) -> Self;
}

impl ConvertRestoreState for () {
    fn to_u8(self) -> u8 {
        0
    }

    fn from_u8(_state: u8) -> Self {}
}

impl ConvertRestoreState for bool {
    fn to_u8(self) -> u8 {
        self.into()
    }

    fn from_u8(state: u8) -> Self {
        state == 1
    }
}

impl ConvertRestoreState for u8 {
    fn to_u8(self) -> u8 {
        self
    }

    fn from_u8(state: u8) -> Self {
        state
    }
}

#[cfg(feature = "custom-impl")]
pub use critical_section_1::{set_impl as custom_impl, Impl};
