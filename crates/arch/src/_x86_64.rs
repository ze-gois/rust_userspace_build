#![allow(unused)]
mod callable;

pub use callable::*;

pub type PointerType = *const u8;

#[derive(Debug, Clone, Copy)]
pub struct Pointer(pub PointerType);

impl Pointer {
    pub fn current() -> Self {
        let p: PointerType;
        unsafe { core::arch::asm!("mov {}, rsp", out(reg) p) };
        Pointer(p)
    }

    pub fn as_ptr(self) -> PointerType {
        self.0
    }
}

impl core::ops::Sub for Pointer {
    type Output = usize;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 as usize - rhs.0 as usize // diferença em bytes
    }
}

impl core::ops::Add<usize> for Pointer {
    type Output = Pointer;
    fn add(self, offset: usize) -> Self::Output {
        Pointer(unsafe { self.0.add(offset) })
    }
}

// #[derive(Debug, Clone, Copy)]
// pub struct Pointer(pub _Pointer);

// impl Pointer {
//     pub fn current() -> Self {
//         let _pointer: _Pointer;
//         unsafe { core::arch::asm!("mov {}, rsp", out(reg) _pointer) };
//         Pointer(_pointer)
//     }
// }

// impl core::ops::Sub for Pointer {
//     type Output = usize;
//     fn sub(self, rhs: Self) -> Self::Output {
//         (self.0 as usize).wrapping_sub(rhs.0 as usize)
//     }
// }

// impl core::ops::Add for Pointer {
//     type Output = Pointer;
//     fn add(self, rhs: Self) -> Self::Output {
//         Pointer((self.0 as usize).wrapping_add(rhs.0 as usize) as _Pointer)
//     }
// }
