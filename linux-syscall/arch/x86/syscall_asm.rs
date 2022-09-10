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

/// Linux syscall result for the `x86` architecture.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[must_use]
pub struct Result(u32);

impl Result {
	#[inline]
	pub const fn new(register_eax: u32) -> Self {
		Self(register_eax)
	}

	#[inline]
	pub const fn as_u32_unchecked(self) -> u32 {
		self.0
	}

	#[inline]
	pub const fn as_usize_unchecked(self) -> usize {
		self.0 as usize
	}
}

single_register_result32!(Result);

#[cfg(not(doc))]
#[macro_export]
macro_rules! syscall {
	($nr:expr $(,)?) => {{
		let mut out_eax;
		core::arch::asm!(
			"int $0x80",
			in("eax") u32::from(Into::<$crate::Syscall>::into($nr)),
			lateout("eax") out_eax,
			options(nostack),
		);
		$crate::arch::x86::Result::new(out_eax)
	}};
	($nr:expr, $a1:expr $(,)?) => {{
		let mut out_eax;
		core::arch::asm!(
			"int $0x80",
			in("eax") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("ebx") $a1,
			lateout("eax") out_eax,
			options(nostack),
		);
		$crate::arch::x86::Result::new(out_eax)
	}};
	($nr:expr, $a1:expr, $a2:expr $(,)?) => {{
		let mut out_eax;
		core::arch::asm!(
			"int $0x80",
			in("eax") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("ebx") $a1,
			in("ecx") $a2,
			lateout("eax") out_eax,
			options(nostack),
		);
		$crate::arch::x86::Result::new(out_eax)
	}};
	($nr:expr, $a1:expr, $a2:expr, $a3:expr $(,)?) => {{
		let mut out_eax;
		core::arch::asm!(
			"int $0x80",
			in("eax") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("ebx") $a1,
			in("ecx") $a2,
			in("edx") $a3,
			lateout("eax") out_eax,
			options(nostack),
		);
		$crate::arch::x86::Result::new(out_eax)
	}};
	($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr $(,)?) => {{
		let in_esi = $a4;
		let mut out_eax;
		core::arch::asm!(
			"push esi",
			"mov esi, {0}",
			"int $0x80",
			"pop esi",
			in(reg) in_esi,
			in("eax") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("ebx") $a1,
			in("ecx") $a2,
			in("edx") $a3,
			lateout("eax") out_eax,
		);
		$crate::arch::x86::Result::new(out_eax)
	}};
	($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr $(,)?) => {{
		let in_esi = $a4;
		let mut out_eax;
		core::arch::asm!(
			"push esi",
			"mov esi, {0}",
			"int $0x80",
			"pop esi",
			in(reg) in_esi,
			in("eax") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("ebx") $a1,
			in("ecx") $a2,
			in("edx") $a3,
			in("edi") $a5,
			lateout("eax") out_eax,
		);
		$crate::arch::x86::Result::new(out_eax)
	}};
	($nr:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr $(,)?) => {{
		let in_esi = $a4;
		let in_ebp = $a6;
		let mut out_eax;
		core::arch::asm!(
			"push esi",
			"push ebp",
			"mov esi, {0}",
			"mov ebp, {1}",
			"int $0x80",
			"pop ebp",
			"pop esi",
			in(reg) in_esi,
			in(reg) in_ebp,
			in("eax") u32::from(Into::<$crate::Syscall>::into($nr)),
			in("ebx") $a1,
			in("ecx") $a2,
			in("edx") $a3,
			in("edi") $a5,
			lateout("eax") out_eax,
		);
		$crate::arch::x86::Result::new(out_eax)
	}};
}
