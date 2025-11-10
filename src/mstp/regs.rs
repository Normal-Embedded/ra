#[doc = "Module Stop Control Register B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrb(pub u32);
impl Mstpcrb {
    #[doc = "Controller Area Network Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb2(&self) -> super::vals::Mstpb2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Mstpb2::from_bits(val as u8)
    }
    #[doc = "Controller Area Network Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb2(&mut self, val: super::vals::Mstpb2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "I2C Bus Interface 1 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb8(&self) -> super::vals::Mstpb8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mstpb8::from_bits(val as u8)
    }
    #[doc = "I2C Bus Interface 1 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb8(&mut self, val: super::vals::Mstpb8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "I2C Bus Interface 0 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb9(&self) -> super::vals::Mstpb9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Mstpb9::from_bits(val as u8)
    }
    #[doc = "I2C Bus Interface 0 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb9(&mut self, val: super::vals::Mstpb9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Universal Serial Bus 2.0 FS Interface Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb11(&self) -> super::vals::Mstpb11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mstpb11::from_bits(val as u8)
    }
    #[doc = "Universal Serial Bus 2.0 FS Interface Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb11(&mut self, val: super::vals::Mstpb11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Serial Peripheral Interface 1 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb18(&self) -> super::vals::Mstpb18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Mstpb18::from_bits(val as u8)
    }
    #[doc = "Serial Peripheral Interface 1 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb18(&mut self, val: super::vals::Mstpb18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Serial Peripheral Interface 0 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb19(&self) -> super::vals::Mstpb19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mstpb19::from_bits(val as u8)
    }
    #[doc = "Serial Peripheral Interface 0 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb19(&mut self, val: super::vals::Mstpb19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Serial Communication Interface 9 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb22(&self) -> super::vals::Mstpb22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Mstpb22::from_bits(val as u8)
    }
    #[doc = "Serial Communication Interface 9 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb22(&mut self, val: super::vals::Mstpb22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Serial Communication Interface 2 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb29(&self) -> super::vals::Mstpb29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Mstpb29::from_bits(val as u8)
    }
    #[doc = "Serial Communication Interface 2 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb29(&mut self, val: super::vals::Mstpb29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Serial Communication Interface 1 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb30(&self) -> super::vals::Mstpb30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Mstpb30::from_bits(val as u8)
    }
    #[doc = "Serial Communication Interface 1 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb30(&mut self, val: super::vals::Mstpb30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Serial Communication Interface 0 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpb31(&self) -> super::vals::Mstpb31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mstpb31::from_bits(val as u8)
    }
    #[doc = "Serial Communication Interface 0 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpb31(&mut self, val: super::vals::Mstpb31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mstpcrb {
    #[inline(always)]
    fn default() -> Mstpcrb {
        Mstpcrb(0)
    }
}
impl core::fmt::Debug for Mstpcrb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mstpcrb")
            .field("mstpb2", &self.mstpb2())
            .field("mstpb8", &self.mstpb8())
            .field("mstpb9", &self.mstpb9())
            .field("mstpb11", &self.mstpb11())
            .field("mstpb18", &self.mstpb18())
            .field("mstpb19", &self.mstpb19())
            .field("mstpb22", &self.mstpb22())
            .field("mstpb29", &self.mstpb29())
            .field("mstpb30", &self.mstpb30())
            .field("mstpb31", &self.mstpb31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mstpcrb {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mstpcrb {{ mstpb2: {:?}, mstpb8: {:?}, mstpb9: {:?}, mstpb11: {:?}, mstpb18: {:?}, mstpb19: {:?}, mstpb22: {:?}, mstpb29: {:?}, mstpb30: {:?}, mstpb31: {:?} }}" , self . mstpb2 () , self . mstpb8 () , self . mstpb9 () , self . mstpb11 () , self . mstpb18 () , self . mstpb19 () , self . mstpb22 () , self . mstpb29 () , self . mstpb30 () , self . mstpb31 ())
    }
}
#[doc = "Module Stop Control Register C"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrc(pub u32);
impl Mstpcrc {
    #[doc = "Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc0(&self) -> super::vals::Mstpc0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mstpc0::from_bits(val as u8)
    }
    #[doc = "Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc0(&mut self, val: super::vals::Mstpc0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Cyclic Redundancy Check Calculator Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc1(&self) -> super::vals::Mstpc1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mstpc1::from_bits(val as u8)
    }
    #[doc = "Cyclic Redundancy Check Calculator Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc1(&mut self, val: super::vals::Mstpc1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Capacitive Touch Sensing Unit Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc3(&self) -> super::vals::Mstpc3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mstpc3::from_bits(val as u8)
    }
    #[doc = "Capacitive Touch Sensing Unit Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc3(&mut self, val: super::vals::Mstpc3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Segment LCD Controller Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc4(&self) -> super::vals::Mstpc4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Mstpc4::from_bits(val as u8)
    }
    #[doc = "Segment LCD Controller Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc4(&mut self, val: super::vals::Mstpc4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Synchronous Serial Interface 0 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc8(&self) -> super::vals::Mstpc8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mstpc8::from_bits(val as u8)
    }
    #[doc = "Synchronous Serial Interface 0 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc8(&mut self, val: super::vals::Mstpc8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Data Operation Circuit Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc13(&self) -> super::vals::Mstpc13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Mstpc13::from_bits(val as u8)
    }
    #[doc = "Data Operation Circuit Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc13(&mut self, val: super::vals::Mstpc13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Event Link Controller Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc14(&self) -> super::vals::Mstpc14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Mstpc14::from_bits(val as u8)
    }
    #[doc = "Event Link Controller Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc14(&mut self, val: super::vals::Mstpc14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SCE5 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpc31(&self) -> super::vals::Mstpc31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mstpc31::from_bits(val as u8)
    }
    #[doc = "SCE5 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpc31(&mut self, val: super::vals::Mstpc31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mstpcrc {
    #[inline(always)]
    fn default() -> Mstpcrc {
        Mstpcrc(0)
    }
}
impl core::fmt::Debug for Mstpcrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mstpcrc")
            .field("mstpc0", &self.mstpc0())
            .field("mstpc1", &self.mstpc1())
            .field("mstpc3", &self.mstpc3())
            .field("mstpc4", &self.mstpc4())
            .field("mstpc8", &self.mstpc8())
            .field("mstpc13", &self.mstpc13())
            .field("mstpc14", &self.mstpc14())
            .field("mstpc31", &self.mstpc31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mstpcrc {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mstpcrc {{ mstpc0: {:?}, mstpc1: {:?}, mstpc3: {:?}, mstpc4: {:?}, mstpc8: {:?}, mstpc13: {:?}, mstpc14: {:?}, mstpc31: {:?} }}" , self . mstpc0 () , self . mstpc1 () , self . mstpc3 () , self . mstpc4 () , self . mstpc8 () , self . mstpc13 () , self . mstpc14 () , self . mstpc31 ())
    }
}
#[doc = "Module Stop Control Register D"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcrd(pub u32);
impl Mstpcrd {
    #[doc = "Asynchronous General Purpose Timer 1 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd2(&self) -> super::vals::Mstpd2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Mstpd2::from_bits(val as u8)
    }
    #[doc = "Asynchronous General Purpose Timer 1 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd2(&mut self, val: super::vals::Mstpd2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Asynchronous General Purpose Timer 0 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd3(&self) -> super::vals::Mstpd3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mstpd3::from_bits(val as u8)
    }
    #[doc = "Asynchronous General Purpose Timer 0 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd3(&mut self, val: super::vals::Mstpd3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "General PWM Timer 323 to 320 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd5(&self) -> super::vals::Mstpd5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Mstpd5::from_bits(val as u8)
    }
    #[doc = "General PWM Timer 323 to 320 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd5(&mut self, val: super::vals::Mstpd5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "General PWM Timer 169 to 164 Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd6(&self) -> super::vals::Mstpd6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Mstpd6::from_bits(val as u8)
    }
    #[doc = "General PWM Timer 169 to 164 Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd6(&mut self, val: super::vals::Mstpd6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Port Output Enable for GPT Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd14(&self) -> super::vals::Mstpd14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Mstpd14::from_bits(val as u8)
    }
    #[doc = "Port Output Enable for GPT Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd14(&mut self, val: super::vals::Mstpd14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "14-Bit A/D Converter Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd16(&self) -> super::vals::Mstpd16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Mstpd16::from_bits(val as u8)
    }
    #[doc = "14-Bit A/D Converter Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd16(&mut self, val: super::vals::Mstpd16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "8-bit D/A Converter Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd19(&self) -> super::vals::Mstpd19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mstpd19::from_bits(val as u8)
    }
    #[doc = "8-bit D/A Converter Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd19(&mut self, val: super::vals::Mstpd19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "12-Bit D/A Converter Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd20(&self) -> super::vals::Mstpd20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Mstpd20::from_bits(val as u8)
    }
    #[doc = "12-Bit D/A Converter Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd20(&mut self, val: super::vals::Mstpd20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Low-Power Analog Comparator Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd29(&self) -> super::vals::Mstpd29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Mstpd29::from_bits(val as u8)
    }
    #[doc = "Low-Power Analog Comparator Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd29(&mut self, val: super::vals::Mstpd29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Operational Amplifier Module Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn mstpd31(&self) -> super::vals::Mstpd31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mstpd31::from_bits(val as u8)
    }
    #[doc = "Operational Amplifier Module Stop"]
    #[inline(always)]
    pub const fn set_mstpd31(&mut self, val: super::vals::Mstpd31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mstpcrd {
    #[inline(always)]
    fn default() -> Mstpcrd {
        Mstpcrd(0)
    }
}
impl core::fmt::Debug for Mstpcrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mstpcrd")
            .field("mstpd2", &self.mstpd2())
            .field("mstpd3", &self.mstpd3())
            .field("mstpd5", &self.mstpd5())
            .field("mstpd6", &self.mstpd6())
            .field("mstpd14", &self.mstpd14())
            .field("mstpd16", &self.mstpd16())
            .field("mstpd19", &self.mstpd19())
            .field("mstpd20", &self.mstpd20())
            .field("mstpd29", &self.mstpd29())
            .field("mstpd31", &self.mstpd31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mstpcrd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mstpcrd {{ mstpd2: {:?}, mstpd3: {:?}, mstpd5: {:?}, mstpd6: {:?}, mstpd14: {:?}, mstpd16: {:?}, mstpd19: {:?}, mstpd20: {:?}, mstpd29: {:?}, mstpd31: {:?} }}" , self . mstpd2 () , self . mstpd3 () , self . mstpd5 () , self . mstpd6 () , self . mstpd14 () , self . mstpd16 () , self . mstpd19 () , self . mstpd20 () , self . mstpd29 () , self . mstpd31 ())
    }
}
