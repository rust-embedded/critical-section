#![no_std]
#![doc = include_str!("../README.md")]

pub use bare_metal::{CriticalSection, Mutex};

#[cfg(any(
    all(feature = "restore-state-none", feature = "restore-state-bool"),
    all(feature = "restore-state-none", feature = "restore-state-u8"),
    all(feature = "restore-state-none", feature = "restore-state-u16"),
    all(feature = "restore-state-none", feature = "restore-state-u32"),
    all(feature = "restore-state-none", feature = "restore-state-u64"),
    all(feature = "restore-state-bool", feature = "restore-state-u8"),
    all(feature = "restore-state-bool", feature = "restore-state-u16"),
    all(feature = "restore-state-bool", feature = "restore-state-u32"),
    all(feature = "restore-state-bool", feature = "restore-state-u64"),
    all(feature = "restore-state-u8", feature = "restore-state-u16"),
    all(feature = "restore-state-u8", feature = "restore-state-u32"),
    all(feature = "restore-state-u8", feature = "restore-state-u64"),
    all(feature = "restore-state-u16", feature = "restore-state-u32"),
    all(feature = "restore-state-u16", feature = "restore-state-u64"),
    all(feature = "restore-state-u32", feature = "restore-state-u64"),
))]
compile_error!("You must set at most one of these Cargo features: restore-state-none, restore-state-bool, restore-state-u8, restore-state-u16, restore-state-u32, restore-state-u64");

#[cfg(not(any(
    feature = "restore-state-bool",
    feature = "restore-state-u8",
    feature = "restore-state-u16",
    feature = "restore-state-u32",
    feature = "restore-state-u64"
)))]
type RawRestoreStateInner = ();

#[cfg(feature = "restore-state-bool")]
type RawRestoreStateInner = bool;

#[cfg(feature = "restore-state-u8")]
type RawRestoreStateInner = u8;

#[cfg(feature = "restore-state-u16")]
type RawRestoreStateInner = u16;

#[cfg(feature = "restore-state-u32")]
type RawRestoreStateInner = u32;

#[cfg(feature = "restore-state-u64")]
type RawRestoreStateInner = u64;

// We have RawRestoreStateInner and RawRestoreState so that we don't have to copypaste the docs 5 times.
// In the docs this shows as `pub type RawRestoreState = u8` or whatever the selected type is, because
// the "inner" type alias is private.

/// Raw, transparent "restore state".
///
/// This type changes based on which Cargo feature is selected, out of
/// - `restore-state-none` (default, makes the type be `()`)
/// - `restore-state-bool`
/// - `restore-state-u8`
/// - `restore-state-u16`
/// - `restore-state-u32`
/// - `restore-state-u64`
///
/// See [`RestoreState`].
///
/// User code uses [`RestoreState`] opaquely, critical section implementations
/// use [`RawRestoreState`] so that they can use the inner value.
pub type RawRestoreState = RawRestoreStateInner;

/// Opaque "restore state".
///
/// Implementations use this to "carry over" information between acquiring and releasing
/// a critical section. For example, when nesting two critical sections of an
/// implementation that disables interrupts globally, acquiring the inner one won't disable
/// the interrupts since they're already disabled. The impl would use the restore state to "tell"
/// the corresponding release that it does *not* have to reenable interrupts yet, only the
/// outer release should do so.
///
/// User code uses [`RestoreState`] opaquely, critical section implementations
/// use [`RawRestoreState`] so that they can use the inner value.
pub struct RestoreState(RawRestoreState);

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
/// - `acquire` returns a "restore state" that you must pass to the corresponding `release` call.
/// - `acquire`/`release` pairs must be "properly nested", ie it's not OK to do `a=acquire(); b=acquire(); release(a); release(b);`.
/// - It is UB to call `release` if the critical section is not acquired in the current thread.
/// - It is UB to call `release` with a "restore state" that does not come from the corresponding `acquire` call.
#[inline]
pub unsafe fn acquire() -> RestoreState {
    extern "Rust" {
        fn _critical_section_acquire() -> RawRestoreState;
    }

    RestoreState(_critical_section_acquire())
}

/// Release the critical section.
///
/// This function is extremely low level. Strongly prefer using [`with`] instead.
///
/// # Safety
///
/// See [`acquire`] for the safety contract description.
#[inline]
pub unsafe fn release(restore_state: RestoreState) {
    extern "Rust" {
        fn _critical_section_release(restore_state: RawRestoreState);
    }
    _critical_section_release(restore_state.0)
}

/// Execute closure `f` in a critical section.
///
/// Nesting critical sections is allowed. The inner critical sections
/// are mostly no-ops since they're already protected by the outer one.
#[inline]
pub fn with<R>(f: impl FnOnce(CriticalSection) -> R) -> R {
    unsafe {
        let restore_state = acquire();
        let r = f(CriticalSection::new());
        release(restore_state);
        r
    }
}

/// Methods required for a critical section implementation.
///
/// This trait is not intended to be used except when implementing a critical section.
///
/// # Safety
///
/// Implementations must uphold the contract specified in [`crate::acquire`] and [`crate::release`].
pub unsafe trait Impl {
    /// Acquire the critical section.
    unsafe fn acquire() -> RawRestoreState;
    /// Release the critical section.
    unsafe fn release(restore_state: RawRestoreState);
}

/// Set the critical section implementation.
///
/// # Example
///
/// ```
/// use critical_section::RawRestoreState;
///
/// struct MyCriticalSection;
/// critical_section::set_impl!(MyCriticalSection);
///
/// unsafe impl critical_section::Impl for MyCriticalSection {
///     unsafe fn acquire() -> RawRestoreState {
///         // ...
///     }
///
///     unsafe fn release(restore_state: RawRestoreState) {
///         // ...
///     }
/// }
///
#[macro_export]
macro_rules! set_impl {
    ($t: ty) => {
        #[no_mangle]
        unsafe fn _critical_section_acquire() -> $crate::RawRestoreState {
            <$t as $crate::Impl>::acquire()
        }
        #[no_mangle]
        unsafe fn _critical_section_release(restore_state: $crate::RawRestoreState) {
            <$t as $crate::Impl>::release(restore_state)
        }
    };
}
