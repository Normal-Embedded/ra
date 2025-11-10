#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Elcon {
    #[doc = "ELC function is disabled."]
    _0 = 0x0,
    #[doc = "ELC function is enabled."]
    _1 = 0x01,
}
impl Elcon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Elcon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Elcon {
    #[inline(always)]
    fn from(val: u8) -> Elcon {
        Elcon::from_bits(val)
    }
}
impl From<Elcon> for u8 {
    #[inline(always)]
    fn from(val: Elcon) -> u8 {
        Elcon::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Elsr0els(u8);
impl Elsr0els {
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    pub const _0X00: Self = Self(0x0);
}
impl Elsr0els {
    pub const fn from_bits(val: u8) -> Elsr0els {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Elsr0els {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0X00"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr0els {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0X00"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Elsr0els {
    #[inline(always)]
    fn from(val: u8) -> Elsr0els {
        Elsr0els::from_bits(val)
    }
}
impl From<Elsr0els> for u8 {
    #[inline(always)]
    fn from(val: Elsr0els) -> u8 {
        Elsr0els::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Elsr12els(u8);
impl Elsr12els {
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    pub const _0X00: Self = Self(0x0);
}
impl Elsr12els {
    pub const fn from_bits(val: u8) -> Elsr12els {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Elsr12els {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0X00"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr12els {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0X00"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Elsr12els {
    #[inline(always)]
    fn from(val: u8) -> Elsr12els {
        Elsr12els::from_bits(val)
    }
}
impl From<Elsr12els> for u8 {
    #[inline(always)]
    fn from(val: Elsr12els) -> u8 {
        Elsr12els::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Elsr1els(u8);
impl Elsr1els {
    #[doc = "Event output to the corresponding peripheral module is disabled."]
    pub const _0X00: Self = Self(0x0);
}
impl Elsr1els {
    pub const fn from_bits(val: u8) -> Elsr1els {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Elsr1els {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0X00"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Elsr1els {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0X00"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Elsr1els {
    #[inline(always)]
    fn from(val: u8) -> Elsr1els {
        Elsr1els::from_bits(val)
    }
}
impl From<Elsr1els> for u8 {
    #[inline(always)]
    fn from(val: Elsr1els) -> u8 {
        Elsr1els::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Seg {
    #[doc = "Normal operation"]
    _0 = 0x0,
    #[doc = "Software event is generated."]
    _1 = 0x01,
}
impl Seg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seg {
    #[inline(always)]
    fn from(val: u8) -> Seg {
        Seg::from_bits(val)
    }
}
impl From<Seg> for u8 {
    #[inline(always)]
    fn from(val: Seg) -> u8 {
        Seg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum We {
    #[doc = "Write to SEG bit is disabled."]
    _0 = 0x0,
    #[doc = "Write to SEG bit is enabled."]
    _1 = 0x01,
}
impl We {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> We {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for We {
    #[inline(always)]
    fn from(val: u8) -> We {
        We::from_bits(val)
    }
}
impl From<We> for u8 {
    #[inline(always)]
    fn from(val: We) -> u8 {
        We::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wi {
    #[doc = "Write to ELSEGR register is enabled."]
    _0 = 0x0,
    #[doc = "Write to ELSEGR register is disabled."]
    _1 = 0x01,
}
impl Wi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wi {
    #[inline(always)]
    fn from(val: u8) -> Wi {
        Wi::from_bits(val)
    }
}
impl From<Wi> for u8 {
    #[inline(always)]
    fn from(val: Wi) -> u8 {
        Wi::to_bits(val)
    }
}
