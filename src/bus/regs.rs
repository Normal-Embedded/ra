#[doc = "Bus Error Address Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserradd(pub u32);
impl Buserradd {
    #[doc = "Bus Error Address When a bus error occurs, It stores an error address."]
    #[must_use]
    #[inline(always)]
    pub const fn berad(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Bus Error Address When a bus error occurs, It stores an error address."]
    #[inline(always)]
    pub const fn set_berad(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Buserradd {
    #[inline(always)]
    fn default() -> Buserradd {
        Buserradd(0)
    }
}
impl core::fmt::Debug for Buserradd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buserradd")
            .field("berad", &self.berad())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buserradd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Buserradd {{ berad: {=u32:?} }}", self.berad())
    }
}
#[doc = "Bus Error Status Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buserrstat(pub u8);
impl Buserrstat {
    #[doc = "Error Access Status The status at the time of the error"]
    #[must_use]
    #[inline(always)]
    pub const fn accstat(&self) -> super::vals::Accstat {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Accstat::from_bits(val as u8)
    }
    #[doc = "Error Access Status The status at the time of the error"]
    #[inline(always)]
    pub const fn set_accstat(&mut self, val: super::vals::Accstat) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Bus Error Status When bus error assert, error flag occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn errstat(&self) -> super::vals::Errstat {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Errstat::from_bits(val as u8)
    }
    #[doc = "Bus Error Status When bus error assert, error flag occurs."]
    #[inline(always)]
    pub const fn set_errstat(&mut self, val: super::vals::Errstat) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Buserrstat {
    #[inline(always)]
    fn default() -> Buserrstat {
        Buserrstat(0)
    }
}
impl core::fmt::Debug for Buserrstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Buserrstat")
            .field("accstat", &self.accstat())
            .field("errstat", &self.errstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buserrstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Buserrstat {{ accstat: {:?}, errstat: {:?} }}",
            self.accstat(),
            self.errstat()
        )
    }
}
#[doc = "Master Bus Control Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busmcnt(pub u16);
impl Busmcnt {
    #[doc = "Ignore Error Responses"]
    #[must_use]
    #[inline(always)]
    pub const fn ieres(&self) -> super::vals::Ieres {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ieres::from_bits(val as u8)
    }
    #[doc = "Ignore Error Responses"]
    #[inline(always)]
    pub const fn set_ieres(&mut self, val: super::vals::Ieres) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
    }
}
impl Default for Busmcnt {
    #[inline(always)]
    fn default() -> Busmcnt {
        Busmcnt(0)
    }
}
impl core::fmt::Debug for Busmcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busmcnt")
            .field("ieres", &self.ieres())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busmcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Busmcnt {{ ieres: {:?} }}", self.ieres())
    }
}
#[doc = "Slave Bus Control Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscnt0(pub u16);
impl Busscnt0 {
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[must_use]
    #[inline(always)]
    pub const fn arbmet(&self) -> super::vals::Busscnt0arbmet {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Busscnt0arbmet::from_bits(val as u8)
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub const fn set_arbmet(&mut self, val: super::vals::Busscnt0arbmet) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
}
impl Default for Busscnt0 {
    #[inline(always)]
    fn default() -> Busscnt0 {
        Busscnt0(0)
    }
}
impl core::fmt::Debug for Busscnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busscnt0")
            .field("arbmet", &self.arbmet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busscnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Busscnt0 {{ arbmet: {:?} }}", self.arbmet())
    }
}
#[doc = "Slave Bus Control Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscnt1(pub u16);
impl Busscnt1 {
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[must_use]
    #[inline(always)]
    pub const fn arbmet(&self) -> super::vals::Busscnt1arbmet {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Busscnt1arbmet::from_bits(val as u8)
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub const fn set_arbmet(&mut self, val: super::vals::Busscnt1arbmet) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
}
impl Default for Busscnt1 {
    #[inline(always)]
    fn default() -> Busscnt1 {
        Busscnt1(0)
    }
}
impl core::fmt::Debug for Busscnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busscnt1")
            .field("arbmet", &self.arbmet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busscnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Busscnt1 {{ arbmet: {:?} }}", self.arbmet())
    }
}
#[doc = "Slave Bus Control Register FBU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntfbu(pub u16);
impl Busscntfbu {
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[must_use]
    #[inline(always)]
    pub const fn arbmet(&self) -> super::vals::BusscntfbuArbmet {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::BusscntfbuArbmet::from_bits(val as u8)
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub const fn set_arbmet(&mut self, val: super::vals::BusscntfbuArbmet) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
}
impl Default for Busscntfbu {
    #[inline(always)]
    fn default() -> Busscntfbu {
        Busscntfbu(0)
    }
}
impl core::fmt::Debug for Busscntfbu {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busscntfbu")
            .field("arbmet", &self.arbmet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busscntfbu {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Busscntfbu {{ arbmet: {:?} }}", self.arbmet())
    }
}
#[doc = "Slave Bus Control Register FLI"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntfli(pub u16);
impl Busscntfli {
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[must_use]
    #[inline(always)]
    pub const fn arbmet(&self) -> super::vals::BusscntfliArbmet {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::BusscntfliArbmet::from_bits(val as u8)
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub const fn set_arbmet(&mut self, val: super::vals::BusscntfliArbmet) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
}
impl Default for Busscntfli {
    #[inline(always)]
    fn default() -> Busscntfli {
        Busscntfli(0)
    }
}
impl core::fmt::Debug for Busscntfli {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busscntfli")
            .field("arbmet", &self.arbmet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busscntfli {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Busscntfli {{ arbmet: {:?} }}", self.arbmet())
    }
}
#[doc = "Slave Bus Control Register P6B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busscntp6b(pub u16);
impl Busscntp6b {
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[must_use]
    #[inline(always)]
    pub const fn arbmet(&self) -> super::vals::Busscntp6bArbmet {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Busscntp6bArbmet::from_bits(val as u8)
    }
    #[doc = "Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub const fn set_arbmet(&mut self, val: super::vals::Busscntp6bArbmet) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
}
impl Default for Busscntp6b {
    #[inline(always)]
    fn default() -> Busscntp6b {
        Busscntp6b(0)
    }
}
impl core::fmt::Debug for Busscntp6b {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busscntp6b")
            .field("arbmet", &self.arbmet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busscntp6b {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Busscntp6b {{ arbmet: {:?} }}", self.arbmet())
    }
}
