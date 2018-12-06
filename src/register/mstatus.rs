//! mstatus register
// TODO: Virtualization, Memory Privilege and Extension Context Fields

use bit_field::BitField;

/// mstatus register
#[derive(Clone, Copy, Debug)]
pub struct Mstatus {
    bits: usize,
}

/// Machine Previous Privilege Mode
pub enum MPP {
    Machine = 3,
    Supervisor = 1,
    User = 0,
}

/// Supervisor Previous Privilege Mode
pub enum SPP {
    Supervisor = 1,
    User = 0,
}

impl Mstatus {
    /// User Interrupt Enable
    #[inline]
    pub fn uie(&self) -> bool {
        self.bits & (1 << 0) == 1 << 0
    }

    /// Supervisor Interrupt Enable
    #[inline]
    pub fn sie(&self) -> bool {
        self.bits & (1 << 1) == 1 << 1
    }

    /// Machine Interrupt Enable
    #[inline]
    pub fn mie(&self) -> bool {
        self.bits & (1 << 3) == 1 << 3
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn upie(&self) -> bool {
        self.bits & (1 << 4) == 1 << 4
    }

    /// Supervisor Previous Interrupt Enable
    #[inline]
    pub fn spie(&self) -> bool {
        self.bits & (1 << 5) == 1 << 5
    }

    /// User Previous Interrupt Enable
    #[inline]
    pub fn mpie(&self) -> bool {
        self.bits & (1 << 7) == 1 << 7
    }

    /// Supervisor Previous Privilege Mode
    #[inline]
    pub fn spp(&self) -> SPP {
        match self.bits & (1 << 8) == (1 << 8) {
            true => SPP::Supervisor,
            false => SPP::User,
        }
    }

    /// Machine Previous Privilege Mode
    #[inline]
    pub fn mpp(&self) -> MPP {
        match (self.bits & (0b11 << 11)) >> 11 {
            0b00 => MPP::User,
            0b01 => MPP::Supervisor,
            0b11 => MPP::Machine,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn xie(&self) -> bool {
        self.mie()
    }

    #[inline]
    pub fn set_xpie(&mut self, val: bool) {
        self.bits.set_bit(7, val);
    }

    #[inline]
    pub fn set_xie(&mut self, val: bool) {
        self.bits.set_bit(3, val);
    }

    #[inline]
    pub fn set_mpp(&mut self, val: MPP) {
        self.bits.set_bits(11..13, val as usize);
    }
}


/// Reads the CSR
#[inline]
pub fn read() -> Mstatus {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => {
            let r: usize;
            unsafe {
                asm!("csrrs $0, 0x300, x0" : "=r"(r) ::: "volatile");
            }
            Mstatus { bits: r }
        }
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => unimplemented!(),
    }
}

/// Sets the CSR
#[cfg_attr(not(any(target_arch = "riscv32", target_arch = "riscv64")), allow(unused_variables))]
#[inline]
unsafe fn set(bits: usize) {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => asm!("csrrs x0, 0x300, $0" :: "r"(bits) :: "volatile"),
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => unimplemented!(),
    }
}

/// Clears the CSR
#[cfg_attr(not(any(target_arch = "riscv32", target_arch = "riscv64")), allow(unused_variables))]
#[inline]
unsafe fn clear(bits: usize) {
    match () {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        () => asm!("csrrc x0, 0x300, $0" :: "r"(bits) :: "volatile"),
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        () => unimplemented!(),
    }
}

macro_rules! set_csr {
    ($set_field:ident, $e:expr) => {
        #[inline]
        pub unsafe fn $set_field() {
            set($e);
        }
    }
}

macro_rules! clear_csr {
    ($clear_field:ident, $e:expr) => {
        #[inline]
        pub unsafe fn $clear_field() {
            clear($e);
        }
    }
}

macro_rules! set_clear_csr {
    ($set_field:ident, $clear_field:ident, $e:expr) => {
        set_csr!($set_field, $e);
        clear_csr!($clear_field, $e);
    }
}

/// User Interrupt Enable
set_clear_csr!(set_uie, clear_uie, 1 << 0);
/// Supervisor Interrupt Enable
set_clear_csr!(set_sie, clear_sie, 1 << 1);
/// Machine Interrupt Enable
set_clear_csr!(set_mie, clear_mie, 1 << 3);
set_clear_csr!(set_xie, clear_xie, 1 << 3);
/// User Previous Interrupt Enable
set_csr!(set_upie, 1 << 4);
/// Supervisor Previous Interrupt Enable
set_csr!(set_spie, 1 << 5);
/// Machine Previous Interrupt Enable
set_csr!(set_mpie, 1 << 7);
set_csr!(set_xpie, 1 << 7);
/// Supervisor Previous Privilege Mode
#[inline]
pub unsafe fn set_spp(spp: SPP) {
    set((spp as usize) << 8);
}
/// Machine Previous Privilege Mode
#[inline]
pub unsafe fn set_mpp(mpp: MPP) {
    set((mpp as usize) << 11);
}
