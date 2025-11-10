#[doc = "Slave MPU Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpuctl(pub u16);
impl Smpuctl {
    #[doc = "Master Group enable"]
    #[must_use]
    #[inline(always)]
    pub const fn oad(&self) -> super::vals::Oad {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Oad::from_bits(val as u8)
    }
    #[doc = "Master Group enable"]
    #[inline(always)]
    pub const fn set_oad(&mut self, val: super::vals::Oad) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Protection of register"]
    #[must_use]
    #[inline(always)]
    pub const fn protect(&self) -> super::vals::Protect {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Protect::from_bits(val as u8)
    }
    #[doc = "Protection of register"]
    #[inline(always)]
    pub const fn set_protect(&mut self, val: super::vals::Protect) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit."]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> super::vals::Key {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Key::from_bits(val as u8)
    }
    #[doc = "Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit."]
    #[inline(always)]
    pub const fn set_key(&mut self, val: super::vals::Key) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
    }
}
impl Default for Smpuctl {
    #[inline(always)]
    fn default() -> Smpuctl {
        Smpuctl(0)
    }
}
impl core::fmt::Debug for Smpuctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smpuctl")
            .field("oad", &self.oad())
            .field("protect", &self.protect())
            .field("key", &self.key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smpuctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smpuctl {{ oad: {:?}, protect: {:?}, key: {:?} }}",
            self.oad(),
            self.protect(),
            self.key()
        )
    }
}
#[doc = "Access Control Register for FBIU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpufbiu(pub u16);
impl Smpufbiu {
    #[doc = "CPU Read protection"]
    #[must_use]
    #[inline(always)]
    pub const fn rpcpu(&self) -> super::vals::SmpufbiuRpcpu {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SmpufbiuRpcpu::from_bits(val as u8)
    }
    #[doc = "CPU Read protection"]
    #[inline(always)]
    pub const fn set_rpcpu(&mut self, val: super::vals::SmpufbiuRpcpu) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "CPU Write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn wpcpu(&self) -> super::vals::SmpufbiuWpcpu {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SmpufbiuWpcpu::from_bits(val as u8)
    }
    #[doc = "CPU Write protection"]
    #[inline(always)]
    pub const fn set_wpcpu(&mut self, val: super::vals::SmpufbiuWpcpu) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Master Group A Read protection"]
    #[must_use]
    #[inline(always)]
    pub const fn rpgrpa(&self) -> super::vals::SmpufbiuRpgrpa {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SmpufbiuRpgrpa::from_bits(val as u8)
    }
    #[doc = "Master Group A Read protection"]
    #[inline(always)]
    pub const fn set_rpgrpa(&mut self, val: super::vals::SmpufbiuRpgrpa) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Master Group A Write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn wpgrpa(&self) -> super::vals::SmpufbiuWpgrpa {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SmpufbiuWpgrpa::from_bits(val as u8)
    }
    #[doc = "Master Group A Write protection"]
    #[inline(always)]
    pub const fn set_wpgrpa(&mut self, val: super::vals::SmpufbiuWpgrpa) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
}
impl Default for Smpufbiu {
    #[inline(always)]
    fn default() -> Smpufbiu {
        Smpufbiu(0)
    }
}
impl core::fmt::Debug for Smpufbiu {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smpufbiu")
            .field("rpcpu", &self.rpcpu())
            .field("wpcpu", &self.wpcpu())
            .field("rpgrpa", &self.rpgrpa())
            .field("wpgrpa", &self.wpgrpa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smpufbiu {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smpufbiu {{ rpcpu: {:?}, wpcpu: {:?}, rpgrpa: {:?}, wpgrpa: {:?} }}",
            self.rpcpu(),
            self.wpcpu(),
            self.rpgrpa(),
            self.wpgrpa()
        )
    }
}
#[doc = "Access Control Register for MBIU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpumbiu(pub u16);
impl Smpumbiu {
    #[doc = "Master Group A Read protection"]
    #[must_use]
    #[inline(always)]
    pub const fn rpgrpa(&self) -> super::vals::SmpumbiuRpgrpa {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SmpumbiuRpgrpa::from_bits(val as u8)
    }
    #[doc = "Master Group A Read protection"]
    #[inline(always)]
    pub const fn set_rpgrpa(&mut self, val: super::vals::SmpumbiuRpgrpa) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Master Group A Write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn wpgrpa(&self) -> super::vals::SmpumbiuWpgrpa {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SmpumbiuWpgrpa::from_bits(val as u8)
    }
    #[doc = "Master Group A Write protection"]
    #[inline(always)]
    pub const fn set_wpgrpa(&mut self, val: super::vals::SmpumbiuWpgrpa) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
}
impl Default for Smpumbiu {
    #[inline(always)]
    fn default() -> Smpumbiu {
        Smpumbiu(0)
    }
}
impl core::fmt::Debug for Smpumbiu {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smpumbiu")
            .field("rpgrpa", &self.rpgrpa())
            .field("wpgrpa", &self.wpgrpa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smpumbiu {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smpumbiu {{ rpgrpa: {:?}, wpgrpa: {:?} }}",
            self.rpgrpa(),
            self.wpgrpa()
        )
    }
}
#[doc = "Access Control Register for P%sBIU"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpupbiu(pub u16);
impl Smpupbiu {
    #[doc = "CPU Read protection"]
    #[must_use]
    #[inline(always)]
    pub const fn rpcpu(&self) -> super::vals::SmpupbiuRpcpu {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SmpupbiuRpcpu::from_bits(val as u8)
    }
    #[doc = "CPU Read protection"]
    #[inline(always)]
    pub const fn set_rpcpu(&mut self, val: super::vals::SmpupbiuRpcpu) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "CPU Write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn wpcpu(&self) -> super::vals::SmpupbiuWpcpu {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SmpupbiuWpcpu::from_bits(val as u8)
    }
    #[doc = "CPU Write protection"]
    #[inline(always)]
    pub const fn set_wpcpu(&mut self, val: super::vals::SmpupbiuWpcpu) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Master Group A Read protection"]
    #[must_use]
    #[inline(always)]
    pub const fn rpgrpa(&self) -> super::vals::SmpupbiuRpgrpa {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SmpupbiuRpgrpa::from_bits(val as u8)
    }
    #[doc = "Master Group A Read protection"]
    #[inline(always)]
    pub const fn set_rpgrpa(&mut self, val: super::vals::SmpupbiuRpgrpa) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Master Group A Write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn wpgrpa(&self) -> super::vals::SmpupbiuWpgrpa {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SmpupbiuWpgrpa::from_bits(val as u8)
    }
    #[doc = "Master Group A Write protection"]
    #[inline(always)]
    pub const fn set_wpgrpa(&mut self, val: super::vals::SmpupbiuWpgrpa) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
}
impl Default for Smpupbiu {
    #[inline(always)]
    fn default() -> Smpupbiu {
        Smpupbiu(0)
    }
}
impl core::fmt::Debug for Smpupbiu {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smpupbiu")
            .field("rpcpu", &self.rpcpu())
            .field("wpcpu", &self.wpcpu())
            .field("rpgrpa", &self.rpgrpa())
            .field("wpgrpa", &self.wpgrpa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smpupbiu {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smpupbiu {{ rpcpu: {:?}, wpcpu: {:?}, rpgrpa: {:?}, wpgrpa: {:?} }}",
            self.rpcpu(),
            self.wpcpu(),
            self.rpgrpa(),
            self.wpgrpa()
        )
    }
}
#[doc = "Access Control Register for SRAM0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smpusram0(pub u16);
impl Smpusram0 {
    #[doc = "CPU Read protection"]
    #[must_use]
    #[inline(always)]
    pub const fn rpcpu(&self) -> super::vals::Smpusram0rpcpu {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Smpusram0rpcpu::from_bits(val as u8)
    }
    #[doc = "CPU Read protection"]
    #[inline(always)]
    pub const fn set_rpcpu(&mut self, val: super::vals::Smpusram0rpcpu) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "CPU Write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn wpcpu(&self) -> super::vals::Smpusram0wpcpu {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Smpusram0wpcpu::from_bits(val as u8)
    }
    #[doc = "CPU Write protection"]
    #[inline(always)]
    pub const fn set_wpcpu(&mut self, val: super::vals::Smpusram0wpcpu) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Master Group A Read protection"]
    #[must_use]
    #[inline(always)]
    pub const fn rpgrpa(&self) -> super::vals::Smpusram0rpgrpa {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Smpusram0rpgrpa::from_bits(val as u8)
    }
    #[doc = "Master Group A Read protection"]
    #[inline(always)]
    pub const fn set_rpgrpa(&mut self, val: super::vals::Smpusram0rpgrpa) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Master Group A Write protection"]
    #[must_use]
    #[inline(always)]
    pub const fn wpgrpa(&self) -> super::vals::Smpusram0wpgrpa {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Smpusram0wpgrpa::from_bits(val as u8)
    }
    #[doc = "Master Group A Write protection"]
    #[inline(always)]
    pub const fn set_wpgrpa(&mut self, val: super::vals::Smpusram0wpgrpa) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
}
impl Default for Smpusram0 {
    #[inline(always)]
    fn default() -> Smpusram0 {
        Smpusram0(0)
    }
}
impl core::fmt::Debug for Smpusram0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smpusram0")
            .field("rpcpu", &self.rpcpu())
            .field("wpcpu", &self.wpcpu())
            .field("rpgrpa", &self.rpgrpa())
            .field("wpgrpa", &self.wpgrpa())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smpusram0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smpusram0 {{ rpcpu: {:?}, wpcpu: {:?}, rpgrpa: {:?}, wpgrpa: {:?} }}",
            self.rpcpu(),
            self.wpcpu(),
            self.rpgrpa(),
            self.wpgrpa()
        )
    }
}
