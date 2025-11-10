#[doc = "Write-Protect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwpr(pub u8);
impl Pwpr {
    #[doc = "PFS Register Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pfswe(&self) -> super::vals::Pfswe {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pfswe::from_bits(val as u8)
    }
    #[doc = "PFS Register Write Enable"]
    #[inline(always)]
    pub const fn set_pfswe(&mut self, val: super::vals::Pfswe) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "PFSWE Bit Write Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn b0wi(&self) -> super::vals::B0wi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::B0wi::from_bits(val as u8)
    }
    #[doc = "PFSWE Bit Write Disable"]
    #[inline(always)]
    pub const fn set_b0wi(&mut self, val: super::vals::B0wi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Pwpr {
    #[inline(always)]
    fn default() -> Pwpr {
        Pwpr(0)
    }
}
impl core::fmt::Debug for Pwpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwpr")
            .field("pfswe", &self.pfswe())
            .field("b0wi", &self.b0wi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwpr {{ pfswe: {:?}, b0wi: {:?} }}",
            self.pfswe(),
            self.b0wi()
        )
    }
}
