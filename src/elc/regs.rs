#[doc = "Event Link Controller Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elcr(pub u8);
impl Elcr {
    #[doc = "All Event Link Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn elcon(&self) -> super::vals::Elcon {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Elcon::from_bits(val as u8)
    }
    #[doc = "All Event Link Enable"]
    #[inline(always)]
    pub const fn set_elcon(&mut self, val: super::vals::Elcon) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Elcr {
    #[inline(always)]
    fn default() -> Elcr {
        Elcr(0)
    }
}
impl core::fmt::Debug for Elcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Elcr")
            .field("elcon", &self.elcon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Elcr {{ elcon: {:?} }}", self.elcon())
    }
}
#[doc = "Event Link Software Event Generation Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsegr(pub u8);
impl Elsegr {
    #[doc = "Software Event Generation"]
    #[must_use]
    #[inline(always)]
    pub const fn seg(&self) -> super::vals::Seg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Seg::from_bits(val as u8)
    }
    #[doc = "Software Event Generation"]
    #[inline(always)]
    pub const fn set_seg(&mut self, val: super::vals::Seg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "SEG Bit Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn we(&self) -> super::vals::We {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::We::from_bits(val as u8)
    }
    #[doc = "SEG Bit Write Enable"]
    #[inline(always)]
    pub const fn set_we(&mut self, val: super::vals::We) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "ELSEGR Register Write Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn wi(&self) -> super::vals::Wi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Wi::from_bits(val as u8)
    }
    #[doc = "ELSEGR Register Write Disable"]
    #[inline(always)]
    pub const fn set_wi(&mut self, val: super::vals::Wi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Elsegr {
    #[inline(always)]
    fn default() -> Elsegr {
        Elsegr(0)
    }
}
impl core::fmt::Debug for Elsegr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Elsegr")
            .field("seg", &self.seg())
            .field("we", &self.we())
            .field("wi", &self.wi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsegr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Elsegr {{ seg: {:?}, we: {:?}, wi: {:?} }}",
            self.seg(),
            self.we(),
            self.wi()
        )
    }
}
#[doc = "Event Link Setting Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsr0(pub u16);
impl Elsr0 {
    #[doc = "Event Link Select"]
    #[must_use]
    #[inline(always)]
    pub const fn els(&self) -> super::vals::Elsr0els {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Elsr0els::from_bits(val as u8)
    }
    #[doc = "Event Link Select"]
    #[inline(always)]
    pub const fn set_els(&mut self, val: super::vals::Elsr0els) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
}
impl Default for Elsr0 {
    #[inline(always)]
    fn default() -> Elsr0 {
        Elsr0(0)
    }
}
impl core::fmt::Debug for Elsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Elsr0").field("els", &self.els()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Elsr0 {{ els: {:?} }}", self.els())
    }
}
#[doc = "Event Link Setting Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsr1(pub u16);
impl Elsr1 {
    #[doc = "Event Link Select"]
    #[must_use]
    #[inline(always)]
    pub const fn els(&self) -> super::vals::Elsr1els {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Elsr1els::from_bits(val as u8)
    }
    #[doc = "Event Link Select"]
    #[inline(always)]
    pub const fn set_els(&mut self, val: super::vals::Elsr1els) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
}
impl Default for Elsr1 {
    #[inline(always)]
    fn default() -> Elsr1 {
        Elsr1(0)
    }
}
impl core::fmt::Debug for Elsr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Elsr1").field("els", &self.els()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Elsr1 {{ els: {:?} }}", self.els())
    }
}
#[doc = "Event Link Setting Register 12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Elsr12(pub u16);
impl Elsr12 {
    #[doc = "Event Link Select"]
    #[must_use]
    #[inline(always)]
    pub const fn els(&self) -> super::vals::Elsr12els {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Elsr12els::from_bits(val as u8)
    }
    #[doc = "Event Link Select"]
    #[inline(always)]
    pub const fn set_els(&mut self, val: super::vals::Elsr12els) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
}
impl Default for Elsr12 {
    #[inline(always)]
    fn default() -> Elsr12 {
        Elsr12(0)
    }
}
impl core::fmt::Debug for Elsr12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Elsr12").field("els", &self.els()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Elsr12 {{ els: {:?} }}", self.els())
    }
}
