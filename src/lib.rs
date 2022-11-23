#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]
#![doc = include_str!("../README.md")]

use core::convert::TryInto;
use core::mem;

pub use bare_metal::CriticalSection;

const RESTORE_STATE_SIZE: usize = mem::size_of::<critical_section::RestoreState>();

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
    if RESTORE_STATE_SIZE > 1 {
        // TODO do const assert instead?
        panic!("restore state bigger than u8 is not supported.")
    }

    let state = critical_section::acquire();
    let state: [u8; RESTORE_STATE_SIZE] = mem::transmute(state);
    *state.get(0).unwrap_or(&0)
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
    let state = [token];
    let state: [u8; RESTORE_STATE_SIZE] = state[..RESTORE_STATE_SIZE].try_into().unwrap();
    let state: critical_section::RestoreState = mem::transmute(state);

    critical_section::release(state)
}

/// Execute closure `f` in a critical section.
///
/// Nesting critical sections is allowed. The inner critical sections
/// are mostly no-ops since they're already protected by the outer one.
#[inline]
pub fn with<R>(f: impl FnOnce(CriticalSection) -> R) -> R {
    critical_section::with(|_| f(unsafe { CriticalSection::new() }))
}

#[cfg(feature = "custom-impl")]
pub use critical_section::{set_impl as custom_impl, Impl};
