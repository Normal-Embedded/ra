#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key(u8);
impl Key {
    #[doc = "Writing to the PROTECT and OAD bit is valid, when the KEY bits are written 0xA5."]
    pub const _0X_A5: Self = Self(0xa5);
}
impl Key {
    pub const fn from_bits(val: u8) -> Key {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xa5 => f.write_str("_0X_A5"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Key {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xa5 => defmt::write!(f, "_0X_A5"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Key {
    #[inline(always)]
    fn from(val: u8) -> Key {
        Key::from_bits(val)
    }
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(val: Key) -> u8 {
        Key::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oad {
    #[doc = "Non-maskable interrupt."]
    _0 = 0x0,
    #[doc = "Internal reset."]
    _1 = 0x01,
}
impl Oad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oad {
    #[inline(always)]
    fn from(val: u8) -> Oad {
        Oad::from_bits(val)
    }
}
impl From<Oad> for u8 {
    #[inline(always)]
    fn from(val: Oad) -> u8 {
        Oad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Protect {
    #[doc = "All Bus Slave register writing is possible."]
    _0 = 0x0,
    #[doc = "All Bus Slave register writing is protected. Read is possible."]
    _1 = 0x01,
}
impl Protect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Protect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Protect {
    #[inline(always)]
    fn from(val: u8) -> Protect {
        Protect::from_bits(val)
    }
}
impl From<Protect> for u8 {
    #[inline(always)]
    fn from(val: Protect) -> u8 {
        Protect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmpufbiuRpcpu {
    #[doc = "CPU read of memory protection disabled."]
    _0 = 0x0,
    #[doc = "CPU read of memory protection enabled."]
    _1 = 0x01,
}
impl SmpufbiuRpcpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmpufbiuRpcpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmpufbiuRpcpu {
    #[inline(always)]
    fn from(val: u8) -> SmpufbiuRpcpu {
        SmpufbiuRpcpu::from_bits(val)
    }
}
impl From<SmpufbiuRpcpu> for u8 {
    #[inline(always)]
    fn from(val: SmpufbiuRpcpu) -> u8 {
        SmpufbiuRpcpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmpufbiuRpgrpa {
    #[doc = "Master group A read of memory protection disabled."]
    _0 = 0x0,
    #[doc = "Master group A read of memory protection enabled."]
    _1 = 0x01,
}
impl SmpufbiuRpgrpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmpufbiuRpgrpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmpufbiuRpgrpa {
    #[inline(always)]
    fn from(val: u8) -> SmpufbiuRpgrpa {
        SmpufbiuRpgrpa::from_bits(val)
    }
}
impl From<SmpufbiuRpgrpa> for u8 {
    #[inline(always)]
    fn from(val: SmpufbiuRpgrpa) -> u8 {
        SmpufbiuRpgrpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmpufbiuWpcpu {
    #[doc = "CPU write of memory protection disabled."]
    _0 = 0x0,
    #[doc = "CPU write of memory protection enabled."]
    _1 = 0x01,
}
impl SmpufbiuWpcpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmpufbiuWpcpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmpufbiuWpcpu {
    #[inline(always)]
    fn from(val: u8) -> SmpufbiuWpcpu {
        SmpufbiuWpcpu::from_bits(val)
    }
}
impl From<SmpufbiuWpcpu> for u8 {
    #[inline(always)]
    fn from(val: SmpufbiuWpcpu) -> u8 {
        SmpufbiuWpcpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmpufbiuWpgrpa {
    #[doc = "Master group A write of memory protection disabled."]
    _0 = 0x0,
    #[doc = "Master group A write of memory protection enabled."]
    _1 = 0x01,
}
impl SmpufbiuWpgrpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmpufbiuWpgrpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmpufbiuWpgrpa {
    #[inline(always)]
    fn from(val: u8) -> SmpufbiuWpgrpa {
        SmpufbiuWpgrpa::from_bits(val)
    }
}
impl From<SmpufbiuWpgrpa> for u8 {
    #[inline(always)]
    fn from(val: SmpufbiuWpgrpa) -> u8 {
        SmpufbiuWpgrpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmpumbiuRpgrpa {
    #[doc = "Master group A read of memory protection disabled."]
    _0 = 0x0,
    #[doc = "Master group A read of memory protection enabled."]
    _1 = 0x01,
}
impl SmpumbiuRpgrpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmpumbiuRpgrpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmpumbiuRpgrpa {
    #[inline(always)]
    fn from(val: u8) -> SmpumbiuRpgrpa {
        SmpumbiuRpgrpa::from_bits(val)
    }
}
impl From<SmpumbiuRpgrpa> for u8 {
    #[inline(always)]
    fn from(val: SmpumbiuRpgrpa) -> u8 {
        SmpumbiuRpgrpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmpumbiuWpgrpa {
    #[doc = "Master group A write of memory protection disabled."]
    _0 = 0x0,
    #[doc = "Master group A write of memory protection enabled."]
    _1 = 0x01,
}
impl SmpumbiuWpgrpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmpumbiuWpgrpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmpumbiuWpgrpa {
    #[inline(always)]
    fn from(val: u8) -> SmpumbiuWpgrpa {
        SmpumbiuWpgrpa::from_bits(val)
    }
}
impl From<SmpumbiuWpgrpa> for u8 {
    #[inline(always)]
    fn from(val: SmpumbiuWpgrpa) -> u8 {
        SmpumbiuWpgrpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmpupbiuRpcpu {
    #[doc = "CPU read of memory protection disabled."]
    _0 = 0x0,
    #[doc = "CPU read of memory protection enabled."]
    _1 = 0x01,
}
impl SmpupbiuRpcpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmpupbiuRpcpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmpupbiuRpcpu {
    #[inline(always)]
    fn from(val: u8) -> SmpupbiuRpcpu {
        SmpupbiuRpcpu::from_bits(val)
    }
}
impl From<SmpupbiuRpcpu> for u8 {
    #[inline(always)]
    fn from(val: SmpupbiuRpcpu) -> u8 {
        SmpupbiuRpcpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmpupbiuRpgrpa {
    #[doc = "Master group A read of memory protection disabled."]
    _0 = 0x0,
    #[doc = "Master group A read of memory protection enabled."]
    _1 = 0x01,
}
impl SmpupbiuRpgrpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmpupbiuRpgrpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmpupbiuRpgrpa {
    #[inline(always)]
    fn from(val: u8) -> SmpupbiuRpgrpa {
        SmpupbiuRpgrpa::from_bits(val)
    }
}
impl From<SmpupbiuRpgrpa> for u8 {
    #[inline(always)]
    fn from(val: SmpupbiuRpgrpa) -> u8 {
        SmpupbiuRpgrpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmpupbiuWpcpu {
    #[doc = "CPU write of memory protection disabled."]
    _0 = 0x0,
    #[doc = "CPU write of memory protection enabled."]
    _1 = 0x01,
}
impl SmpupbiuWpcpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmpupbiuWpcpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmpupbiuWpcpu {
    #[inline(always)]
    fn from(val: u8) -> SmpupbiuWpcpu {
        SmpupbiuWpcpu::from_bits(val)
    }
}
impl From<SmpupbiuWpcpu> for u8 {
    #[inline(always)]
    fn from(val: SmpupbiuWpcpu) -> u8 {
        SmpupbiuWpcpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmpupbiuWpgrpa {
    #[doc = "Master group A write of memory protection disabled."]
    _0 = 0x0,
    #[doc = "Master group A write of memory protection enabled."]
    _1 = 0x01,
}
impl SmpupbiuWpgrpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmpupbiuWpgrpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmpupbiuWpgrpa {
    #[inline(always)]
    fn from(val: u8) -> SmpupbiuWpgrpa {
        SmpupbiuWpgrpa::from_bits(val)
    }
}
impl From<SmpupbiuWpgrpa> for u8 {
    #[inline(always)]
    fn from(val: SmpupbiuWpgrpa) -> u8 {
        SmpupbiuWpgrpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smpusram0rpcpu {
    #[doc = "CPU read of memory protection disabled."]
    _0 = 0x0,
    #[doc = "CPU read of memory protection enabled."]
    _1 = 0x01,
}
impl Smpusram0rpcpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smpusram0rpcpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smpusram0rpcpu {
    #[inline(always)]
    fn from(val: u8) -> Smpusram0rpcpu {
        Smpusram0rpcpu::from_bits(val)
    }
}
impl From<Smpusram0rpcpu> for u8 {
    #[inline(always)]
    fn from(val: Smpusram0rpcpu) -> u8 {
        Smpusram0rpcpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smpusram0rpgrpa {
    #[doc = "Master group A read of memory protection disabled."]
    _0 = 0x0,
    #[doc = "Master group A read of memory protection enabled."]
    _1 = 0x01,
}
impl Smpusram0rpgrpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smpusram0rpgrpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smpusram0rpgrpa {
    #[inline(always)]
    fn from(val: u8) -> Smpusram0rpgrpa {
        Smpusram0rpgrpa::from_bits(val)
    }
}
impl From<Smpusram0rpgrpa> for u8 {
    #[inline(always)]
    fn from(val: Smpusram0rpgrpa) -> u8 {
        Smpusram0rpgrpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smpusram0wpcpu {
    #[doc = "CPU write of memory protection disabled."]
    _0 = 0x0,
    #[doc = "CPU write of memory protection enabled."]
    _1 = 0x01,
}
impl Smpusram0wpcpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smpusram0wpcpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smpusram0wpcpu {
    #[inline(always)]
    fn from(val: u8) -> Smpusram0wpcpu {
        Smpusram0wpcpu::from_bits(val)
    }
}
impl From<Smpusram0wpcpu> for u8 {
    #[inline(always)]
    fn from(val: Smpusram0wpcpu) -> u8 {
        Smpusram0wpcpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smpusram0wpgrpa {
    #[doc = "Master group A write of memory protection disabled."]
    _0 = 0x0,
    #[doc = "Master group A write of memory protection enabled."]
    _1 = 0x01,
}
impl Smpusram0wpgrpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smpusram0wpgrpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smpusram0wpgrpa {
    #[inline(always)]
    fn from(val: u8) -> Smpusram0wpgrpa {
        Smpusram0wpgrpa::from_bits(val)
    }
}
impl From<Smpusram0wpgrpa> for u8 {
    #[inline(always)]
    fn from(val: Smpusram0wpgrpa) -> u8 {
        Smpusram0wpgrpa::to_bits(val)
    }
}
