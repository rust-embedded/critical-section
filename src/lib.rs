#![no_std]

pub use bare_metal::CriticalSection;

/// Acquire the critical section.
#[inline]
pub unsafe fn acquire() -> u8 {
    extern "Rust" {
        fn _critical_section_acquire() -> u8;
    }

    _critical_section_acquire()
}

/// Release the critical section.
#[inline]
pub unsafe fn release(token: u8) {
    extern "Rust" {
        fn _critical_section_release(token: u8);
    }
    _critical_section_release(token)
}

/// Execute closure `f` in a critical section.
#[inline]
pub fn with<R>(f: impl FnOnce(CriticalSection) -> R) -> R {
    unsafe {
        let token = acquire();
        let r = f(CriticalSection::new());
        release(token);
        r
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "custom-impl")] {
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
        macro_rules! custom_impl {
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
    } else if #[cfg(cortex_m)] {
        #[no_mangle]
        unsafe fn _critical_section_acquire() -> u8 {
            let primask = cortex_m::register::primask::read();
            cortex_m::interrupt::disable();
            primask.is_active() as _
        }

        #[no_mangle]
        unsafe fn _critical_section_release(token: u8) {
            if token != 0 {
                cortex_m::interrupt::enable()
            }
        }
    } else if #[cfg(target_arch = "riscv32")] {
        #[no_mangle]
        unsafe fn _critical_section_acquire() -> u8 {
            let interrupts_active = riscv::register::mstatus::read().mie();
            riscv::interrupt::disable();
            interrupts_active as _
        }

        #[no_mangle]
        unsafe fn _critical_section_release(token: u8) {
            if token != 0 {
                riscv::interrupt::enable();
            }
        }
    } else if #[cfg(any(unix, windows, wasm, target_arch = "wasm32"))] {
        extern crate std;
        static INIT: std::sync::Once = std::sync::Once::new();
        static mut GLOBAL_LOCK: Option<std::sync::Mutex<()>> = None;
        static mut GLOBAL_GUARD: Option<std::sync::MutexGuard<'static, ()>> = None;

        #[no_mangle]
        unsafe fn _critical_section_acquire() -> u8 {
            INIT.call_once(|| unsafe {
                GLOBAL_LOCK.replace(std::sync::Mutex::new(()));
            });

            let guard = GLOBAL_LOCK.as_ref().unwrap().lock().unwrap();
            GLOBAL_GUARD.replace(guard);
            1
        }

        #[no_mangle]
        unsafe fn _critical_section_release(token: u8) {
            if token == 1 {
                GLOBAL_GUARD.take();
            }
        }
    } else {
        compile_error!("Critical section is not implemented for this target. Make sure you've specified the correct --target. You may need to supply a custom critical section implementation with the `custom-impl` feature");
    }
}
