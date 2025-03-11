// Copyright (c) 2025 John Millikin <john@john-millikin.com>
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

/// Linux syscall result for the `s390x` architecture.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[must_use]
pub struct Result(u64);

impl Result {
	#[inline]
	pub const fn new(register_r2: u64) -> Self {
		Self(register_r2)
	}

	#[inline]
	pub const fn as_u64_unchecked(self) -> u64 {
		self.0
	}

	#[inline]
	pub const fn as_usize_unchecked(self) -> usize {
		self.0 as usize
	}
}

single_register_result64!(Result);

#[cfg(not(doc))]
#[macro_export]
macro_rules! syscall {
	($nr:expr $(,)?) => {{
		let mut out_r2;
		core::arch::asm!(
			"svc 0",
			in("r1") u32::from(Into::<$crate::Syscall>::into($nr)),
			out("r2") out_r2,
			options(nostack),
		);
		$crate::arch::s390x::Result::new(out_r2)
	}};
	($nr:expr, $a1:expr $(,)?) => {{
		let mut out_r2;
		core::arch::asm!(
			"svc 0",
			in("r1") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("r2") $a1,
			lateout("r2") out_r2,
			options(nostack),
		);
		$crate::arch::s390x::Result::new(out_r2)
	}};
	($nr:expr, $a1:expr, $a2:expr $(,)?) => {{
		let mut out_r2;
		core::arch::asm!(
			"svc 0",
			in("r1") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("r2") $a1,
			in("r3") $a2,
			lateout("r2") out_r2,
			options(nostack),
		);
		$crate::arch::s390x::Result::new(out_r2)
	}};
	($nr:expr, $a1:expr, $a2:expr, $a3:expr $(,)?) => {{
		let mut out_r2;
		core::arch::asm!(
			"svc 0",
			in("r1") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("r2") $a1,
			in("r3") $a2,
			in("r4") $a3,
			lateout("r2") out_r2,
			options(nostack),
		);
		$crate::arch::s390x::Result::new(out_r2)
	}};
	($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr $(,)?) => {{
		let mut out_r2;
		core::arch::asm!(
			"svc 0",
			in("r1") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("r2") $a1,
			in("r3") $a2,
			in("r4") $a3,
			in("r5") $a4,
			lateout("r2") out_r2,
			options(nostack),
		);
		$crate::arch::s390x::Result::new(out_r2)
	}};
	($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr $(,)?) => {{
		let mut out_r2;
		core::arch::asm!(
			"svc 0",
			in("r1") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("r2") $a1,
			in("r3") $a2,
			in("r4") $a3,
			in("r5") $a4,
			in("r6") $a5,
			lateout("r2") out_r2,
			options(nostack),
		);
		$crate::arch::s390x::Result::new(out_r2)
	}};
	($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr $(,)?) => {{
		let mut out_r2;
		core::arch::asm!(
			"svc 0",
			in("r1") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("r2") $a1,
			in("r3") $a2,
			in("r4") $a3,
			in("r5") $a4,
			in("r6") $a5,
			in("r7") $a6,
			lateout("r2") out_r2,
			options(nostack),
		);
		$crate::arch::s390x::Result::new(out_r2)
	}};
}
