#[doc = "D/A Conversion Value Setting Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacs(pub u8);
impl Dacs {
    #[doc = "DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\\[7:0\\] bits for the channel in use is prohibited."]
    #[must_use]
    #[inline(always)]
    pub const fn dacs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DACS D/A conversion store data note: When 8-bit D/A Converter output is selected as the reference input for the ACMPLP in the COMPSEL1 register, and ACMPLP operation is enabled (COMPMDR.CnENB = 1), changing the DACS\\[7:0\\] bits for the channel in use is prohibited."]
    #[inline(always)]
    pub const fn set_dacs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Dacs {
    #[inline(always)]
    fn default() -> Dacs {
        Dacs(0)
    }
}
impl core::fmt::Debug for Dacs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dacs").field("dacs", &self.dacs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dacs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dacs {{ dacs: {=u8:?} }}", self.dacs())
    }
}
#[doc = "D/A Converter Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dam(pub u8);
impl Dam {
    #[doc = "D/A Operation Enable 0"]
    #[must_use]
    #[inline(always)]
    pub const fn dace0(&self) -> super::vals::Dace0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dace0::from_bits(val as u8)
    }
    #[doc = "D/A Operation Enable 0"]
    #[inline(always)]
    pub const fn set_dace0(&mut self, val: super::vals::Dace0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "D/A Operation Enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn dace1(&self) -> super::vals::Dace1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dace1::from_bits(val as u8)
    }
    #[doc = "D/A Operation Enable 1"]
    #[inline(always)]
    pub const fn set_dace1(&mut self, val: super::vals::Dace1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
}
impl Default for Dam {
    #[inline(always)]
    fn default() -> Dam {
        Dam(0)
    }
}
impl core::fmt::Debug for Dam {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dam")
            .field("dace0", &self.dace0())
            .field("dace1", &self.dace1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dam {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dam {{ dace0: {:?}, dace1: {:?} }}",
            self.dace0(),
            self.dace1()
        )
    }
}
