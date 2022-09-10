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

/// Linux syscall result for the `x86_64` architecture.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[must_use]
pub struct Result(u64);

impl Result {
	#[inline]
	pub const fn new(register_rax: u64) -> Self {
		Self(register_rax)
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
		let mut out_rax;
		core::arch::asm!(
			"syscall",
			in("rax") u32::from(Into::<$crate::Syscall>::into($nr)),
			out("rcx") _,
			out("r11") _,
			lateout("rax") out_rax,
			options(nostack),
		);
		$crate::arch::x86_64::Result::new(out_rax)
	}};
	($nr:expr, $a1:expr $(,)?) => {{
		let mut out_rax;
		core::arch::asm!(
			"syscall",
			in("rax") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("rdi") $a1,
			out("rcx") _,
			out("r11") _,
			lateout("rax") out_rax,
			options(nostack),
		);
		$crate::arch::x86_64::Result::new(out_rax)
	}};
	($nr:expr, $a1:expr, $a2:expr $(,)?) => {{
		let mut out_rax;
		core::arch::asm!(
			"syscall",
			in("rax") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("rdi") $a1,
			in("rsi") $a2,
			out("rcx") _,
			out("r11") _,
			lateout("rax") out_rax,
			options(nostack),
		);
		$crate::arch::x86_64::Result::new(out_rax)
	}};
	($nr:expr, $a1:expr, $a2:expr, $a3:expr $(,)?) => {{
		let mut out_rax;
		core::arch::asm!(
			"syscall",
			in("rax") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("rdi") $a1,
			in("rsi") $a2,
			in("rdx") $a3,
			out("rcx") _,
			out("r11") _,
			lateout("rax") out_rax,
			options(nostack),
		);
		$crate::arch::x86_64::Result::new(out_rax)
	}};
	($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr $(,)?) => {{
		let mut out_rax;
		core::arch::asm!(
			"syscall",
			in("rax") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("rdi") $a1,
			in("rsi") $a2,
			in("rdx") $a3,
			in("r10") $a4,
			out("rcx") _,
			out("r11") _,
			lateout("rax") out_rax,
			options(nostack),
		);
		$crate::arch::x86_64::Result::new(out_rax)
	}};
	($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr $(,)?) => {{
		let mut out_rax;
		core::arch::asm!(
			"syscall",
			in("rax") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("rdi") $a1,
			in("rsi") $a2,
			in("rdx") $a3,
			in("r10") $a4,
			in("r8")  $a5,
			out("rcx") _,
			out("r11") _,
			lateout("rax") out_rax,
			options(nostack),
		);
		$crate::arch::x86_64::Result::new(out_rax)
	}};
	($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr $(,)?) => {{
		let mut out_rax;
		core::arch::asm!(
			"syscall",
			in("rax") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("rdi") $a1,
			in("rsi") $a2,
			in("rdx") $a3,
			in("r10") $a4,
			in("r8")  $a5,
			in("r9")  $a6,
			out("rcx") _,
			out("r11") _,
			lateout("rax") out_rax,
			options(nostack),
		);
		$crate::arch::x86_64::Result::new(out_rax)
	}};
}
