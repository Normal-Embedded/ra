#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr1pdr(u16);
impl Pcntr1pdr {
    #[doc = "Input (functions as an input pin)"]
    pub const _0: Self = Self(0x0);
    #[doc = "Output (functions as an output pin)."]
    pub const _1: Self = Self(0x01);
}
impl Pcntr1pdr {
    pub const fn from_bits(val: u16) -> Pcntr1pdr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr1pdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr1pdr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr1pdr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr1pdr {
        Pcntr1pdr::from_bits(val)
    }
}
impl From<Pcntr1pdr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr1pdr) -> u16 {
        Pcntr1pdr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr1podr(u16);
impl Pcntr1podr {
    #[doc = "Low output"]
    pub const _0: Self = Self(0x0);
    #[doc = "High output."]
    pub const _1: Self = Self(0x01);
}
impl Pcntr1podr {
    pub const fn from_bits(val: u16) -> Pcntr1podr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr1podr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr1podr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr1podr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr1podr {
        Pcntr1podr::from_bits(val)
    }
}
impl From<Pcntr1podr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr1podr) -> u16 {
        Pcntr1podr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr2pidr(u16);
impl Pcntr2pidr {
    #[doc = "Low input"]
    pub const _0: Self = Self(0x0);
    #[doc = "High input."]
    pub const _1: Self = Self(0x01);
}
impl Pcntr2pidr {
    pub const fn from_bits(val: u16) -> Pcntr2pidr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr2pidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr2pidr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr2pidr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr2pidr {
        Pcntr2pidr::from_bits(val)
    }
}
impl From<Pcntr2pidr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr2pidr) -> u16 {
        Pcntr2pidr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr3porr(u16);
impl Pcntr3porr {
    #[doc = "No affect to output"]
    pub const _0: Self = Self(0x0);
    #[doc = "Low output."]
    pub const _1: Self = Self(0x01);
}
impl Pcntr3porr {
    pub const fn from_bits(val: u16) -> Pcntr3porr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr3porr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr3porr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr3porr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr3porr {
        Pcntr3porr::from_bits(val)
    }
}
impl From<Pcntr3porr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr3porr) -> u16 {
        Pcntr3porr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pcntr3posr(u16);
impl Pcntr3posr {
    #[doc = "No affect to output"]
    pub const _0: Self = Self(0x0);
    #[doc = "High output."]
    pub const _1: Self = Self(0x01);
}
impl Pcntr3posr {
    pub const fn from_bits(val: u16) -> Pcntr3posr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pcntr3posr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcntr3posr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pcntr3posr {
    #[inline(always)]
    fn from(val: u16) -> Pcntr3posr {
        Pcntr3posr::from_bits(val)
    }
}
impl From<Pcntr3posr> for u16 {
    #[inline(always)]
    fn from(val: Pcntr3posr) -> u16 {
        Pcntr3posr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PdrPdr(u16);
impl PdrPdr {
    #[doc = "Input (functions as an input pin)"]
    pub const _0: Self = Self(0x0);
    #[doc = "Output (functions as an output pin)."]
    pub const _1: Self = Self(0x01);
}
impl PdrPdr {
    pub const fn from_bits(val: u16) -> PdrPdr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for PdrPdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdrPdr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for PdrPdr {
    #[inline(always)]
    fn from(val: u16) -> PdrPdr {
        PdrPdr::from_bits(val)
    }
}
impl From<PdrPdr> for u16 {
    #[inline(always)]
    fn from(val: PdrPdr) -> u16 {
        PdrPdr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PidrPidr(u16);
impl PidrPidr {
    #[doc = "Low input"]
    pub const _0: Self = Self(0x0);
    #[doc = "High input."]
    pub const _1: Self = Self(0x01);
}
impl PidrPidr {
    pub const fn from_bits(val: u16) -> PidrPidr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for PidrPidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PidrPidr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for PidrPidr {
    #[inline(always)]
    fn from(val: u16) -> PidrPidr {
        PidrPidr::from_bits(val)
    }
}
impl From<PidrPidr> for u16 {
    #[inline(always)]
    fn from(val: PidrPidr) -> u16 {
        PidrPidr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PodrPodr(u16);
impl PodrPodr {
    #[doc = "Low output"]
    pub const _0: Self = Self(0x0);
    #[doc = "High output."]
    pub const _1: Self = Self(0x01);
}
impl PodrPodr {
    pub const fn from_bits(val: u16) -> PodrPodr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for PodrPodr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PodrPodr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for PodrPodr {
    #[inline(always)]
    fn from(val: u16) -> PodrPodr {
        PodrPodr::from_bits(val)
    }
}
impl From<PodrPodr> for u16 {
    #[inline(always)]
    fn from(val: PodrPodr) -> u16 {
        PodrPodr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PorrPorr(u16);
impl PorrPorr {
    #[doc = "No affect to output"]
    pub const _0: Self = Self(0x0);
    #[doc = "Low output."]
    pub const _1: Self = Self(0x01);
}
impl PorrPorr {
    pub const fn from_bits(val: u16) -> PorrPorr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for PorrPorr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PorrPorr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for PorrPorr {
    #[inline(always)]
    fn from(val: u16) -> PorrPorr {
        PorrPorr::from_bits(val)
    }
}
impl From<PorrPorr> for u16 {
    #[inline(always)]
    fn from(val: PorrPorr) -> u16 {
        PorrPorr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PosrPosr(u16);
impl PosrPosr {
    #[doc = "No affect to output"]
    pub const _0: Self = Self(0x0);
    #[doc = "High output."]
    pub const _1: Self = Self(0x01);
}
impl PosrPosr {
    pub const fn from_bits(val: u16) -> PosrPosr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for PosrPosr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("_0"),
            0x01 => f.write_str("_1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PosrPosr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "_0"),
            0x01 => defmt::write!(f, "_1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for PosrPosr {
    #[inline(always)]
    fn from(val: u16) -> PosrPosr {
        PosrPosr::from_bits(val)
    }
}
impl From<PosrPosr> for u16 {
    #[inline(always)]
    fn from(val: PosrPosr) -> u16 {
        PosrPosr::to_bits(val)
    }
}
