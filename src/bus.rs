#[doc = "BUS Control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bus {
    ptr: *mut u8,
}
unsafe impl Send for Bus {}
unsafe impl Sync for Bus {}
impl Bus {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Master Bus Control Register %s"]
    #[inline(always)]
    pub const fn busmcnt(self, n: usize) -> crate::common::Reg<regs::Busmcnt, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize + n * 4usize) as _)
        }
    }
    #[doc = "Slave Bus Control Register FLI"]
    #[inline(always)]
    pub const fn busscntfli(self) -> crate::common::Reg<regs::Busscntfli, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[doc = "Slave Bus Control Register %s"]
    #[inline(always)]
    pub const fn busscnt0(self, n: usize) -> crate::common::Reg<regs::Busscnt0, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize + n * 4usize) as _)
        }
    }
    #[doc = "Slave Bus Control Register %s"]
    #[inline(always)]
    pub const fn busscnt1(self, n: usize) -> crate::common::Reg<regs::Busscnt1, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize + n * 4usize) as _)
        }
    }
    #[doc = "Slave Bus Control Register P6B"]
    #[inline(always)]
    pub const fn busscntp6b(self) -> crate::common::Reg<regs::Busscntp6b, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "Slave Bus Control Register FBU"]
    #[inline(always)]
    pub const fn busscntfbu(self) -> crate::common::Reg<regs::Busscntfbu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1130usize) as _) }
    }
    #[doc = "Bus Error Address Register %s"]
    #[inline(always)]
    pub const fn buserradd(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Buserradd, crate::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1800usize + n * 16usize) as _)
        }
    }
    #[doc = "Bus Error Status Register %s"]
    #[inline(always)]
    pub const fn buserrstat(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Buserrstat, crate::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1804usize + n * 16usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
