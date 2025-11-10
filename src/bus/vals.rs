#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Accstat {
    #[doc = "Read access"]
    _0 = 0x0,
    #[doc = "Write Access"]
    _1 = 0x01,
}
impl Accstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Accstat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Accstat {
    #[inline(always)]
    fn from(val: u8) -> Accstat {
        Accstat::from_bits(val)
    }
}
impl From<Accstat> for u8 {
    #[inline(always)]
    fn from(val: Accstat) -> u8 {
        Accstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busscnt0arbmet {
    #[doc = "fixed priority"]
    _00 = 0x0,
    #[doc = "round-robin"]
    _01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Busscnt0arbmet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busscnt0arbmet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busscnt0arbmet {
    #[inline(always)]
    fn from(val: u8) -> Busscnt0arbmet {
        Busscnt0arbmet::from_bits(val)
    }
}
impl From<Busscnt0arbmet> for u8 {
    #[inline(always)]
    fn from(val: Busscnt0arbmet) -> u8 {
        Busscnt0arbmet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busscnt1arbmet {
    #[doc = "fixed priority"]
    _00 = 0x0,
    #[doc = "round-robin"]
    _01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Busscnt1arbmet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busscnt1arbmet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busscnt1arbmet {
    #[inline(always)]
    fn from(val: u8) -> Busscnt1arbmet {
        Busscnt1arbmet::from_bits(val)
    }
}
impl From<Busscnt1arbmet> for u8 {
    #[inline(always)]
    fn from(val: Busscnt1arbmet) -> u8 {
        Busscnt1arbmet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusscntfbuArbmet {
    #[doc = "fixed priority"]
    _00 = 0x0,
    #[doc = "round-robin"]
    _01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl BusscntfbuArbmet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusscntfbuArbmet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusscntfbuArbmet {
    #[inline(always)]
    fn from(val: u8) -> BusscntfbuArbmet {
        BusscntfbuArbmet::from_bits(val)
    }
}
impl From<BusscntfbuArbmet> for u8 {
    #[inline(always)]
    fn from(val: BusscntfbuArbmet) -> u8 {
        BusscntfbuArbmet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusscntfliArbmet {
    #[doc = "fixed priority"]
    _00 = 0x0,
    #[doc = "round-robin"]
    _01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl BusscntfliArbmet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusscntfliArbmet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusscntfliArbmet {
    #[inline(always)]
    fn from(val: u8) -> BusscntfliArbmet {
        BusscntfliArbmet::from_bits(val)
    }
}
impl From<BusscntfliArbmet> for u8 {
    #[inline(always)]
    fn from(val: BusscntfliArbmet) -> u8 {
        BusscntfliArbmet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busscntp6bArbmet {
    #[doc = "fixed priority"]
    _00 = 0x0,
    #[doc = "round-robin"]
    _01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Busscntp6bArbmet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busscntp6bArbmet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busscntp6bArbmet {
    #[inline(always)]
    fn from(val: u8) -> Busscntp6bArbmet {
        Busscntp6bArbmet::from_bits(val)
    }
}
impl From<Busscntp6bArbmet> for u8 {
    #[inline(always)]
    fn from(val: Busscntp6bArbmet) -> u8 {
        Busscntp6bArbmet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Errstat {
    #[doc = "No bus error occurred"]
    _0 = 0x0,
    #[doc = "Bus error occurred."]
    _1 = 0x01,
}
impl Errstat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Errstat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Errstat {
    #[inline(always)]
    fn from(val: u8) -> Errstat {
        Errstat::from_bits(val)
    }
}
impl From<Errstat> for u8 {
    #[inline(always)]
    fn from(val: Errstat) -> u8 {
        Errstat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ieres {
    #[doc = "A bus error is reported"]
    _0 = 0x0,
    #[doc = "A bus error is not reported."]
    _1 = 0x01,
}
impl Ieres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ieres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ieres {
    #[inline(always)]
    fn from(val: u8) -> Ieres {
        Ieres::from_bits(val)
    }
}
impl From<Ieres> for u8 {
    #[inline(always)]
    fn from(val: Ieres) -> u8 {
        Ieres::to_bits(val)
    }
}
