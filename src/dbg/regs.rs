#[doc = "Debug Stop Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgstopcr(pub u32);
impl Dbgstopcr {
    #[doc = "Mask bit for IWDT reset/interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgstop_iwdt(&self) -> super::vals::DbgstopIwdt {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DbgstopIwdt::from_bits(val as u8)
    }
    #[doc = "Mask bit for IWDT reset/interrupt"]
    #[inline(always)]
    pub const fn set_dbgstop_iwdt(&mut self, val: super::vals::DbgstopIwdt) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Mask bit for WDT reset/interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgstop_wdt(&self) -> super::vals::DbgstopWdt {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DbgstopWdt::from_bits(val as u8)
    }
    #[doc = "Mask bit for WDT reset/interrupt"]
    #[inline(always)]
    pub const fn set_dbgstop_wdt(&mut self, val: super::vals::DbgstopWdt) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask) b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask) b16: Mask bit for LVD0 reset (0:enable / 1:Mask)"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgstop_lvd(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "b18: Mask bit for LVD2 reset/interrupt (0:enable / 1:Mask) b17: Mask bit for LVD1 reset/interrupt (0:enable / 1:Mask) b16: Mask bit for LVD0 reset (0:enable / 1:Mask)"]
    #[inline(always)]
    pub const fn set_dbgstop_lvd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Mask bit for RAM parity error reset/interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgstop_rper(&self) -> super::vals::DbgstopRper {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::DbgstopRper::from_bits(val as u8)
    }
    #[doc = "Mask bit for RAM parity error reset/interrupt"]
    #[inline(always)]
    pub const fn set_dbgstop_rper(&mut self, val: super::vals::DbgstopRper) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Mask bit for RAM ECC error reset/interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgstop_reccr(&self) -> super::vals::DbgstopReccr {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::DbgstopReccr::from_bits(val as u8)
    }
    #[doc = "Mask bit for RAM ECC error reset/interrupt"]
    #[inline(always)]
    pub const fn set_dbgstop_reccr(&mut self, val: super::vals::DbgstopReccr) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Dbgstopcr {
    #[inline(always)]
    fn default() -> Dbgstopcr {
        Dbgstopcr(0)
    }
}
impl core::fmt::Debug for Dbgstopcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgstopcr")
            .field("dbgstop_iwdt", &self.dbgstop_iwdt())
            .field("dbgstop_wdt", &self.dbgstop_wdt())
            .field("dbgstop_lvd", &self.dbgstop_lvd())
            .field("dbgstop_rper", &self.dbgstop_rper())
            .field("dbgstop_reccr", &self.dbgstop_reccr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgstopcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dbgstopcr {{ dbgstop_iwdt: {:?}, dbgstop_wdt: {:?}, dbgstop_lvd: {=u8:?}, dbgstop_rper: {:?}, dbgstop_reccr: {:?} }}" , self . dbgstop_iwdt () , self . dbgstop_wdt () , self . dbgstop_lvd () , self . dbgstop_rper () , self . dbgstop_reccr ())
    }
}
#[doc = "Debug Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgstr(pub u32);
impl Dbgstr {
    #[doc = "Debug power-up request"]
    #[must_use]
    #[inline(always)]
    pub const fn cdbgpwrupreq(&self) -> super::vals::Cdbgpwrupreq {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Cdbgpwrupreq::from_bits(val as u8)
    }
    #[doc = "Debug power-up request"]
    #[inline(always)]
    pub const fn set_cdbgpwrupreq(&mut self, val: super::vals::Cdbgpwrupreq) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Debug power-up acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn cdbgpwrupack(&self) -> super::vals::Cdbgpwrupack {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Cdbgpwrupack::from_bits(val as u8)
    }
    #[doc = "Debug power-up acknowledge"]
    #[inline(always)]
    pub const fn set_cdbgpwrupack(&mut self, val: super::vals::Cdbgpwrupack) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Dbgstr {
    #[inline(always)]
    fn default() -> Dbgstr {
        Dbgstr(0)
    }
}
impl core::fmt::Debug for Dbgstr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgstr")
            .field("cdbgpwrupreq", &self.cdbgpwrupreq())
            .field("cdbgpwrupack", &self.cdbgpwrupack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgstr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dbgstr {{ cdbgpwrupreq: {:?}, cdbgpwrupack: {:?} }}",
            self.cdbgpwrupreq(),
            self.cdbgpwrupack()
        )
    }
}
#[doc = "Trace Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tracectr(pub u32);
impl Tracectr {
    #[doc = "Enable bit for halt request by ETB full"]
    #[must_use]
    #[inline(always)]
    pub const fn enetbfull(&self) -> super::vals::Enetbfull {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Enetbfull::from_bits(val as u8)
    }
    #[doc = "Enable bit for halt request by ETB full"]
    #[inline(always)]
    pub const fn set_enetbfull(&mut self, val: super::vals::Enetbfull) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Tracectr {
    #[inline(always)]
    fn default() -> Tracectr {
        Tracectr(0)
    }
}
impl core::fmt::Debug for Tracectr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tracectr")
            .field("enetbfull", &self.enetbfull())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tracectr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tracectr {{ enetbfull: {:?} }}", self.enetbfull())
    }
}
