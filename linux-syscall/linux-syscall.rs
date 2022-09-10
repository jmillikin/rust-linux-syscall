// Copyright (c) 2022 John Millikin <john@john-millikin.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
// AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.
//
// SPDX-License-Identifier: 0BSD

//! This library defines syscall numbers and a [`syscall!`] macro for directly
//! invoking Linux system calls.
//!
//! The [`arch`] modules document available syscall numbers for all supported
//! architectures, and the top-level module re-exports syscall numbers for the
//! current target platform.
//!
//! Syscall results may be inspected with the [`Result*` traits](#traits).
//!
//! # Example
//!
//! ```
//! # #[macro_use] extern crate linux_syscall;
//! # use linux_syscall::*;
//! # fn main() -> core::result::Result<(), linux_errno::Error> {
//! let stdout: i32 = 1;
//! let hello = "Hello, world!\n\0";
//! let rc = unsafe {
//! 	syscall!(SYS_write, stdout, hello.as_ptr(), hello.len())
//! };
//! rc.check()?;
//! # Ok(())
//! # }
//! ```
//!
//! # Safety
//!
//! Very unsafe.
//!
//! Linux syscalls are low-level primitives that operate without any notion
//! of borrow checking or type safety. It is the caller's responsibility to
//! ensure parameters have types, values, and lifetimes appropriate to the
//! syscall being invoked.
//!
//! * The kernel cannot distinguish `*const T` and `*mut T`.
//! * Many syscalls accept complex parameters as pointers to a `struct`. The
//!   caller must ensure such parameters have appropriate layout and alignment.
//! * Syscalls vary between architectures. The same syscall name may have
//!   a completely different signature even on similar targets, for example
//!   `SYS_mmap` on `x86` and `x86_64`.

#![no_std]

use linux_errno::Error;

/// An architecture-specific syscall number.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Syscall {
	nr: u32,
}

impl Syscall {
	#[inline]
	pub const fn from_u32(nr: u32) -> Syscall {
		Syscall { nr }
	}
}

impl From<u32> for Syscall {
	#[inline]
	fn from(nr: u32) -> Syscall {
		Syscall { nr }
	}
}

impl From<Syscall> for u32 {
	#[inline]
	fn from(syscall: Syscall) -> u32 {
		syscall.nr
	}
}

/// Check whether a syscall succeeded or failed.
pub trait Result {
	fn check(&self) -> core::result::Result<(), Error>;
}

/// Interpret a syscall result as a 32-bit integer.
pub trait Result32: Result {
	fn try_i32(&self) -> core::result::Result<i32, Error>;

	fn try_u32(&self) -> core::result::Result<u32, Error>;
}

/// Interpret a syscall result as a 64-bit integer.
pub trait Result64: Result {
	fn try_i64(&self) -> core::result::Result<i64, Error>;

	fn try_u64(&self) -> core::result::Result<u64, Error>;
}

/// Interpret a syscall result as a pointer.
pub trait ResultPtr: Result {
	fn try_ptr(&self) -> core::result::Result<*const (), Error>;

	fn try_ptr_mut(&self) -> core::result::Result<*mut (), Error>;
}

/// Interpret a syscall result as a pointer-sized integer.
pub trait ResultSize: Result {
	fn try_isize(&self) -> core::result::Result<isize, Error>;

	fn try_usize(&self) -> core::result::Result<usize, Error>;
}

macro_rules! single_register_result {
	($arch_result:ty) => {
		use linux_errno::Error;

		impl $crate::Result for $arch_result {
			#[inline]
			fn check(&self) -> core::result::Result<(), Error> {
				if self.0 >= self::MAX_ERRNO {
					return Err(new_err(self.0 as u16));
				}
				Ok(())
			}
		}

		#[inline]
		#[cold]
		const fn new_err(truncated_register: u16) -> Error {
			let (err, _) = truncated_register.overflowing_neg();
			unsafe { Error::new_unchecked(err) }
		}

		impl $crate::ResultPtr for $arch_result {
			#[inline]
			fn try_ptr(&self) -> core::result::Result<*const (), Error> {
				$crate::Result::check(self)?;
				Ok(self.0 as *const ())
			}

			#[inline]
			fn try_ptr_mut(&self) -> core::result::Result<*mut (), Error> {
				$crate::Result::check(self)?;
				Ok(self.0 as *mut ())
			}
		}

		impl<T> core::convert::TryFrom<$arch_result> for *const T {
			type Error = Error;

			#[inline]
			fn try_from(rc: $arch_result) -> core::result::Result<Self, Error> {
				$crate::ResultPtr::try_ptr(&rc).map(|p| p.cast())
			}
		}

		impl<T> core::convert::TryFrom<$arch_result> for *mut T {
			type Error = Error;

			#[inline]
			fn try_from(rc: $arch_result) -> core::result::Result<Self, Error> {
				$crate::ResultPtr::try_ptr_mut(&rc).map(|p| p.cast())
			}
		}

		impl $crate::ResultSize for $arch_result {
			#[inline]
			fn try_isize(&self) -> core::result::Result<isize, Error> {
				$crate::Result::check(self)?;
				Ok(self.0 as isize)
			}

			#[inline]
			fn try_usize(&self) -> core::result::Result<usize, Error> {
				$crate::Result::check(self)?;
				Ok(self.0 as usize)
			}
		}

		impl core::convert::TryFrom<$arch_result> for isize {
			type Error = Error;

			#[inline]
			fn try_from(rc: $arch_result) -> core::result::Result<Self, Error> {
				$crate::ResultSize::try_isize(&rc)
			}
		}

		impl core::convert::TryFrom<$arch_result> for usize {
			type Error = Error;

			#[inline]
			fn try_from(rc: $arch_result) -> core::result::Result<Self, Error> {
				$crate::ResultSize::try_usize(&rc)
			}
		}
	};
}

#[cfg(any(target_pointer_width = "32", doc))]
macro_rules! single_register_result32 {
	($arch_result:ty) => {
		single_register_result!($arch_result);

		const MAX_ERRNO: u32 = (-4095i32) as u32;

		impl $crate::Result32 for $arch_result {
			#[inline]
			fn try_i32(&self) -> core::result::Result<i32, Error> {
				$crate::Result::check(self)?;
				Ok(self.0 as i32)
			}

			#[inline]
			fn try_u32(&self) -> core::result::Result<u32, Error> {
				$crate::Result::check(self)?;
				Ok(self.0)
			}
		}

		impl core::convert::TryFrom<$arch_result> for i32 {
			type Error = Error;

			#[inline]
			fn try_from(rc: $arch_result) -> core::result::Result<Self, Error> {
				$crate::Result32::try_i32(&rc)
			}
		}

		impl core::convert::TryFrom<$arch_result> for u32 {
			type Error = Error;

			#[inline]
			fn try_from(rc: $arch_result) -> core::result::Result<Self, Error> {
				$crate::Result32::try_u32(&rc)
			}
		}
	};
}

#[cfg(any(target_pointer_width = "64", doc))]
macro_rules! single_register_result64 {
	($arch_result:ty) => {
		single_register_result!($arch_result);

		const MAX_ERRNO: u64 = (-4095i64) as u64;

		impl $crate::Result64 for $arch_result {
			#[inline]
			fn try_i64(&self) -> core::result::Result<i64, Error> {
				$crate::Result::check(self)?;
				Ok(self.0 as i64)
			}

			#[inline]
			fn try_u64(&self) -> core::result::Result<u64, Error> {
				$crate::Result::check(self)?;
				Ok(self.0)
			}
		}

		impl core::convert::TryFrom<$arch_result> for i64 {
			type Error = Error;

			#[inline]
			fn try_from(rc: $arch_result) -> core::result::Result<Self, Error> {
				$crate::Result64::try_i64(&rc)
			}
		}

		impl core::convert::TryFrom<$arch_result> for u64 {
			type Error = Error;

			#[inline]
			fn try_from(rc: $arch_result) -> core::result::Result<Self, Error> {
				$crate::Result64::try_u64(&rc)
			}
		}
	};
}

macro_rules! syscall_constants {
	( $( $name:ident = $value:literal , )+ ) => {
		use $crate::Syscall;
		$(
			pub const $name: Syscall = Syscall::from_u32($value);
		)*
	};
}

/// Linux syscall numbers for specific target architectures.
pub mod arch {
	/// Linux syscall numbers for the `aarch64` architecture.
	#[cfg(any(target_arch = "aarch64", doc))]
	pub mod aarch64 {
		mod syscall_asm;
		pub use self::syscall_asm::Result;

		pub(crate) mod syscall_tbl;
		pub use self::syscall_tbl::*;
	}

	/// Linux syscall numbers for the `arm` architecture.
	#[cfg(any(target_arch = "arm", doc))]
	pub mod arm {
		mod syscall_asm;
		pub use self::syscall_asm::Result;

		pub(crate) mod syscall_tbl;
		pub use self::syscall_tbl::*;
	}

	/// Linux syscall numbers for the `riscv64` architecture.
	#[cfg(any(target_arch = "riscv64", doc))]
	pub mod riscv64 {
		mod syscall_asm;
		pub use self::syscall_asm::Result;

		pub(crate) mod syscall_tbl;
		pub use self::syscall_tbl::*;
	}

	/// Linux syscall numbers for the `x86` architecture.
	#[cfg(any(target_arch = "x86", doc))]
	pub mod x86 {
		mod syscall_asm;
		pub use self::syscall_asm::Result;

		pub(crate) mod syscall_tbl;
		pub use self::syscall_tbl::*;
	}

	/// Linux syscall numbers for the `x86_64` architecture.
	#[cfg(any(target_arch = "x86_64", doc))]
	pub mod x86_64 {
		mod syscall_asm;
		pub use self::syscall_asm::Result;

		pub(crate) mod syscall_tbl;
		pub use self::syscall_tbl::*;
	}
}

#[cfg(target_arch = "arm")]
pub use crate::arch::arm::syscall_tbl::*;

#[cfg(target_arch = "aarch64")]
pub use crate::arch::aarch64::syscall_tbl::*;

#[cfg(target_arch = "riscv64")]
pub use crate::arch::riscv64::syscall_tbl::*;

#[cfg(target_arch = "x86")]
pub use crate::arch::x86::syscall_tbl::*;

#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::syscall_tbl::*;

/// Invokes a Linux syscall.
///
/// `$syscall` must be a value that implements [`Into<Syscall>`](Syscall).
/// Other arguments must be valid [`asm!`](core::arch::asm!) input operands,
/// such as integers or pointers.
///
/// The returned value is an architecture-specific implementation of [`Result`].
///
/// Additional traits implemented by syscall results vary by architecture.
/// For all architectures currently supported by this library:
///
/// * The [`ResultSize`] and [`ResultPtr`] traits are implemented.
/// * One of the [`Result32`] or [`Result64`] traits is implemented, according
///   to the native word size.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate linux_syscall;
/// # use linux_syscall::*;
/// # fn main() -> core::result::Result<(), linux_errno::Error> {
/// let stdout: i32 = 1;
/// let hello = "Hello, world!\n\0";
/// let rc = unsafe {
/// 	syscall!(SYS_write, stdout, hello.as_ptr(), hello.len())
/// };
/// rc.check()?;
/// # Ok(())
/// # }
/// ```
///
/// # Safety
///
/// Very unsafe. See the [module documentation](self) for details.
#[cfg(doc)]
#[macro_export]
macro_rules! syscall {
	($syscall:expr $(,)?) => {};
	($syscall:expr, $a1:expr $(,)?) => {};
	($syscall:expr, $a1:expr, $a2:expr $(,)?) => {};
	($syscall:expr, $a1:expr, $a2:expr, $a3:expr $(,)?) => {};
	($syscall:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr $(,)?) => {};
	($syscall:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr $(,)?) => {};
	($syscall:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr $(,)?) => {};
}
