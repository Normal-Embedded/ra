#[doc = "Port Output Enable Module for GPT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Poeg {
    ptr: *mut u8,
}
unsafe impl Send for Poeg {}
unsafe impl Sync for Poeg {}
impl Poeg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "POEG Group %s Setting Register"]
    #[inline(always)]
    pub const fn poegg(self, n: usize) -> crate::common::Reg<regs::Poegg, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 256usize) as _) }
    }
}
pub mod regs;
pub mod vals;
