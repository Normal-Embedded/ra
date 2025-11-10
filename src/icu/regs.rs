#[doc = "DMAC Event Link Setting Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Delsr(pub u16);
impl Delsr {
    #[doc = "Event selection to DMAC Start request"]
    #[must_use]
    #[inline(always)]
    pub const fn dels(&self) -> super::vals::Dels {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Dels::from_bits(val as u8)
    }
    #[doc = "Event selection to DMAC Start request"]
    #[inline(always)]
    pub const fn set_dels(&mut self, val: super::vals::Dels) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
}
impl Default for Delsr {
    #[inline(always)]
    fn default() -> Delsr {
        Delsr(0)
    }
}
impl core::fmt::Debug for Delsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Delsr").field("dels", &self.dels()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Delsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Delsr {{ dels: {:?} }}", self.dels())
    }
}
#[doc = "ICU Event Link Setting Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ielsr(pub u32);
impl Ielsr {
    #[doc = "ICU Event selection to NVIC Set the number for the event signal to be linked ."]
    #[must_use]
    #[inline(always)]
    pub const fn iels(&self) -> super::vals::Iels {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Iels::from_bits(val as u8)
    }
    #[doc = "ICU Event selection to NVIC Set the number for the event signal to be linked ."]
    #[inline(always)]
    pub const fn set_iels(&mut self, val: super::vals::Iels) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ir(&self) -> super::vals::Ir {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ir::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_ir(&mut self, val: super::vals::Ir) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DTC Activation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtce(&self) -> super::vals::Dtce {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dtce::from_bits(val as u8)
    }
    #[doc = "DTC Activation Enable"]
    #[inline(always)]
    pub const fn set_dtce(&mut self, val: super::vals::Dtce) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Ielsr {
    #[inline(always)]
    fn default() -> Ielsr {
        Ielsr(0)
    }
}
impl core::fmt::Debug for Ielsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ielsr")
            .field("iels", &self.iels())
            .field("ir", &self.ir())
            .field("dtce", &self.dtce())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ielsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ielsr {{ iels: {:?}, ir: {:?}, dtce: {:?} }}",
            self.iels(),
            self.ir(),
            self.dtce()
        )
    }
}
#[doc = "IRQ Control Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqcr0(pub u8);
impl Irqcr0 {
    #[doc = "IRQ Detection Sense Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqmd(&self) -> super::vals::Irqcr0irqmd {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Irqcr0irqmd::from_bits(val as u8)
    }
    #[doc = "IRQ Detection Sense Select"]
    #[inline(always)]
    pub const fn set_irqmd(&mut self, val: super::vals::Irqcr0irqmd) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "IRQ Digital Filter Sampling Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn fclksel(&self) -> super::vals::Irqcr0fclksel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Irqcr0fclksel::from_bits(val as u8)
    }
    #[doc = "IRQ Digital Filter Sampling Clock Select"]
    #[inline(always)]
    pub const fn set_fclksel(&mut self, val: super::vals::Irqcr0fclksel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
    }
    #[doc = "IRQ Digital Filter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn flten(&self) -> super::vals::Irqcr0flten {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Irqcr0flten::from_bits(val as u8)
    }
    #[doc = "IRQ Digital Filter Enable"]
    #[inline(always)]
    pub const fn set_flten(&mut self, val: super::vals::Irqcr0flten) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Irqcr0 {
    #[inline(always)]
    fn default() -> Irqcr0 {
        Irqcr0(0)
    }
}
impl core::fmt::Debug for Irqcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqcr0")
            .field("irqmd", &self.irqmd())
            .field("fclksel", &self.fclksel())
            .field("flten", &self.flten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Irqcr0 {{ irqmd: {:?}, fclksel: {:?}, flten: {:?} }}",
            self.irqmd(),
            self.fclksel(),
            self.flten()
        )
    }
}
#[doc = "IRQ Control Register %s"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqcr1(pub u8);
impl Irqcr1 {
    #[doc = "IRQ Detection Sense Select"]
    #[must_use]
    #[inline(always)]
    pub const fn irqmd(&self) -> super::vals::Irqcr1irqmd {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Irqcr1irqmd::from_bits(val as u8)
    }
    #[doc = "IRQ Detection Sense Select"]
    #[inline(always)]
    pub const fn set_irqmd(&mut self, val: super::vals::Irqcr1irqmd) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "IRQ Digital Filter Sampling Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn fclksel(&self) -> super::vals::Irqcr1fclksel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Irqcr1fclksel::from_bits(val as u8)
    }
    #[doc = "IRQ Digital Filter Sampling Clock Select"]
    #[inline(always)]
    pub const fn set_fclksel(&mut self, val: super::vals::Irqcr1fclksel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
    }
    #[doc = "IRQ Digital Filter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn flten(&self) -> super::vals::Irqcr1flten {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Irqcr1flten::from_bits(val as u8)
    }
    #[doc = "IRQ Digital Filter Enable"]
    #[inline(always)]
    pub const fn set_flten(&mut self, val: super::vals::Irqcr1flten) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Irqcr1 {
    #[inline(always)]
    fn default() -> Irqcr1 {
        Irqcr1(0)
    }
}
impl core::fmt::Debug for Irqcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqcr1")
            .field("irqmd", &self.irqmd())
            .field("fclksel", &self.fclksel())
            .field("flten", &self.flten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Irqcr1 {{ irqmd: {:?}, fclksel: {:?}, flten: {:?} }}",
            self.irqmd(),
            self.fclksel(),
            self.flten()
        )
    }
}
#[doc = "Non-Maskable Interrupt Status Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmiclr(pub u16);
impl Nmiclr {
    #[doc = "IWDT Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn iwdtclr(&self) -> super::vals::Iwdtclr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Iwdtclr::from_bits(val as u8)
    }
    #[doc = "IWDT Clear"]
    #[inline(always)]
    pub const fn set_iwdtclr(&mut self, val: super::vals::Iwdtclr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "WDT Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn wdtclr(&self) -> super::vals::Wdtclr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Wdtclr::from_bits(val as u8)
    }
    #[doc = "WDT Clear"]
    #[inline(always)]
    pub const fn set_wdtclr(&mut self, val: super::vals::Wdtclr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "LVD1 Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd1clr(&self) -> super::vals::Lvd1clr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Lvd1clr::from_bits(val as u8)
    }
    #[doc = "LVD1 Clear"]
    #[inline(always)]
    pub const fn set_lvd1clr(&mut self, val: super::vals::Lvd1clr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "LVD2 Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd2clr(&self) -> super::vals::Lvd2clr {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lvd2clr::from_bits(val as u8)
    }
    #[doc = "LVD2 Clear"]
    #[inline(always)]
    pub const fn set_lvd2clr(&mut self, val: super::vals::Lvd2clr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "VBATT Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn vbattclr(&self) -> super::vals::Vbattclr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Vbattclr::from_bits(val as u8)
    }
    #[doc = "VBATT Clear"]
    #[inline(always)]
    pub const fn set_vbattclr(&mut self, val: super::vals::Vbattclr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "OST Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ostclr(&self) -> super::vals::Ostclr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ostclr::from_bits(val as u8)
    }
    #[doc = "OST Clear"]
    #[inline(always)]
    pub const fn set_ostclr(&mut self, val: super::vals::Ostclr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "NMI Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn nmiclr(&self) -> super::vals::Nmiclr {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Nmiclr::from_bits(val as u8)
    }
    #[doc = "NMI Clear"]
    #[inline(always)]
    pub const fn set_nmiclr(&mut self, val: super::vals::Nmiclr) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "SRAM Parity Error Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn rpeclr(&self) -> super::vals::Rpeclr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rpeclr::from_bits(val as u8)
    }
    #[doc = "SRAM Parity Error Clear"]
    #[inline(always)]
    pub const fn set_rpeclr(&mut self, val: super::vals::Rpeclr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "SRAM ECC Error Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn reccclr(&self) -> super::vals::Reccclr {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Reccclr::from_bits(val as u8)
    }
    #[doc = "SRAM ECC Error Clear"]
    #[inline(always)]
    pub const fn set_reccclr(&mut self, val: super::vals::Reccclr) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "Bus Slave Error Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn bussclr(&self) -> super::vals::Bussclr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Bussclr::from_bits(val as u8)
    }
    #[doc = "Bus Slave Error Clear"]
    #[inline(always)]
    pub const fn set_bussclr(&mut self, val: super::vals::Bussclr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "Bus Master Error Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn busmclr(&self) -> super::vals::Busmclr {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Busmclr::from_bits(val as u8)
    }
    #[doc = "Bus Master Error Clear"]
    #[inline(always)]
    pub const fn set_busmclr(&mut self, val: super::vals::Busmclr) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "CPU Stack Pointer Monitor Interrupt Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn speclr(&self) -> super::vals::Speclr {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Speclr::from_bits(val as u8)
    }
    #[doc = "CPU Stack Pointer Monitor Interrupt Clear"]
    #[inline(always)]
    pub const fn set_speclr(&mut self, val: super::vals::Speclr) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
}
impl Default for Nmiclr {
    #[inline(always)]
    fn default() -> Nmiclr {
        Nmiclr(0)
    }
}
impl core::fmt::Debug for Nmiclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nmiclr")
            .field("iwdtclr", &self.iwdtclr())
            .field("wdtclr", &self.wdtclr())
            .field("lvd1clr", &self.lvd1clr())
            .field("lvd2clr", &self.lvd2clr())
            .field("vbattclr", &self.vbattclr())
            .field("ostclr", &self.ostclr())
            .field("nmiclr", &self.nmiclr())
            .field("rpeclr", &self.rpeclr())
            .field("reccclr", &self.reccclr())
            .field("bussclr", &self.bussclr())
            .field("busmclr", &self.busmclr())
            .field("speclr", &self.speclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nmiclr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Nmiclr {{ iwdtclr: {:?}, wdtclr: {:?}, lvd1clr: {:?}, lvd2clr: {:?}, vbattclr: {:?}, ostclr: {:?}, nmiclr: {:?}, rpeclr: {:?}, reccclr: {:?}, bussclr: {:?}, busmclr: {:?}, speclr: {:?} }}" , self . iwdtclr () , self . wdtclr () , self . lvd1clr () , self . lvd2clr () , self . vbattclr () , self . ostclr () , self . nmiclr () , self . rpeclr () , self . reccclr () , self . bussclr () , self . busmclr () , self . speclr ())
    }
}
#[doc = "NMI Pin Interrupt Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmicr(pub u8);
impl Nmicr {
    #[doc = "NMI Detection Set"]
    #[must_use]
    #[inline(always)]
    pub const fn nmimd(&self) -> super::vals::Nmimd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Nmimd::from_bits(val as u8)
    }
    #[doc = "NMI Detection Set"]
    #[inline(always)]
    pub const fn set_nmimd(&mut self, val: super::vals::Nmimd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "NMI Digital Filter Sampling Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn nfclksel(&self) -> super::vals::Nfclksel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Nfclksel::from_bits(val as u8)
    }
    #[doc = "NMI Digital Filter Sampling Clock Select"]
    #[inline(always)]
    pub const fn set_nfclksel(&mut self, val: super::vals::Nfclksel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u8) & 0x03) << 4usize);
    }
    #[doc = "NMI Digital Filter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nflten(&self) -> super::vals::Nflten {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Nflten::from_bits(val as u8)
    }
    #[doc = "NMI Digital Filter Enable"]
    #[inline(always)]
    pub const fn set_nflten(&mut self, val: super::vals::Nflten) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Nmicr {
    #[inline(always)]
    fn default() -> Nmicr {
        Nmicr(0)
    }
}
impl core::fmt::Debug for Nmicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nmicr")
            .field("nmimd", &self.nmimd())
            .field("nfclksel", &self.nfclksel())
            .field("nflten", &self.nflten())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nmicr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nmicr {{ nmimd: {:?}, nfclksel: {:?}, nflten: {:?} }}",
            self.nmimd(),
            self.nfclksel(),
            self.nflten()
        )
    }
}
#[doc = "Non-Maskable Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmier(pub u16);
impl Nmier {
    #[doc = "IWDT Underflow/Refresh Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iwdten(&self) -> super::vals::Iwdten {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Iwdten::from_bits(val as u8)
    }
    #[doc = "IWDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_iwdten(&mut self, val: super::vals::Iwdten) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "WDT Underflow/Refresh Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wdten(&self) -> super::vals::Wdten {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Wdten::from_bits(val as u8)
    }
    #[doc = "WDT Underflow/Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wdten(&mut self, val: super::vals::Wdten) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Voltage-Monitoring 1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd1en(&self) -> super::vals::Lvd1en {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Lvd1en::from_bits(val as u8)
    }
    #[doc = "Voltage-Monitoring 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_lvd1en(&mut self, val: super::vals::Lvd1en) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Voltage-Monitoring 2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd2en(&self) -> super::vals::Lvd2en {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lvd2en::from_bits(val as u8)
    }
    #[doc = "Voltage-Monitoring 2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_lvd2en(&mut self, val: super::vals::Lvd2en) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "VBATT monitor Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbatten(&self) -> super::vals::Vbatten {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Vbatten::from_bits(val as u8)
    }
    #[doc = "VBATT monitor Interrupt Enable"]
    #[inline(always)]
    pub const fn set_vbatten(&mut self, val: super::vals::Vbatten) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Oscillation Stop Detection Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn osten(&self) -> super::vals::Osten {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Osten::from_bits(val as u8)
    }
    #[doc = "Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub const fn set_osten(&mut self, val: super::vals::Osten) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "NMI Pin Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nmien(&self) -> super::vals::Nmien {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Nmien::from_bits(val as u8)
    }
    #[doc = "NMI Pin Interrupt Enable"]
    #[inline(always)]
    pub const fn set_nmien(&mut self, val: super::vals::Nmien) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "RAM Parity Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rpeen(&self) -> super::vals::Rpeen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rpeen::from_bits(val as u8)
    }
    #[doc = "RAM Parity Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rpeen(&mut self, val: super::vals::Rpeen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "RAM ECC Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reccen(&self) -> super::vals::Reccen {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Reccen::from_bits(val as u8)
    }
    #[doc = "RAM ECC Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_reccen(&mut self, val: super::vals::Reccen) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "MPU Bus Slave Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bussen(&self) -> super::vals::Bussen {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Bussen::from_bits(val as u8)
    }
    #[doc = "MPU Bus Slave Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_bussen(&mut self, val: super::vals::Bussen) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "MPU Bus Master Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn busmen(&self) -> super::vals::Busmen {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Busmen::from_bits(val as u8)
    }
    #[doc = "MPU Bus Master Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_busmen(&mut self, val: super::vals::Busmen) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "CPU Stack pointer monitor Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn speen(&self) -> super::vals::Speen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Speen::from_bits(val as u8)
    }
    #[doc = "CPU Stack pointer monitor Interrupt Enable"]
    #[inline(always)]
    pub const fn set_speen(&mut self, val: super::vals::Speen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
}
impl Default for Nmier {
    #[inline(always)]
    fn default() -> Nmier {
        Nmier(0)
    }
}
impl core::fmt::Debug for Nmier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nmier")
            .field("iwdten", &self.iwdten())
            .field("wdten", &self.wdten())
            .field("lvd1en", &self.lvd1en())
            .field("lvd2en", &self.lvd2en())
            .field("vbatten", &self.vbatten())
            .field("osten", &self.osten())
            .field("nmien", &self.nmien())
            .field("rpeen", &self.rpeen())
            .field("reccen", &self.reccen())
            .field("bussen", &self.bussen())
            .field("busmen", &self.busmen())
            .field("speen", &self.speen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nmier {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Nmier {{ iwdten: {:?}, wdten: {:?}, lvd1en: {:?}, lvd2en: {:?}, vbatten: {:?}, osten: {:?}, nmien: {:?}, rpeen: {:?}, reccen: {:?}, bussen: {:?}, busmen: {:?}, speen: {:?} }}" , self . iwdten () , self . wdten () , self . lvd1en () , self . lvd2en () , self . vbatten () , self . osten () , self . nmien () , self . rpeen () , self . reccen () , self . bussen () , self . busmen () , self . speen ())
    }
}
#[doc = "Non-Maskable Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmisr(pub u16);
impl Nmisr {
    #[doc = "IWDT Underflow/Refresh Error Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn iwdtst(&self) -> super::vals::Iwdtst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Iwdtst::from_bits(val as u8)
    }
    #[doc = "IWDT Underflow/Refresh Error Status Flag"]
    #[inline(always)]
    pub const fn set_iwdtst(&mut self, val: super::vals::Iwdtst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "WDT Underflow/Refresh Error Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wdtst(&self) -> super::vals::Wdtst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Wdtst::from_bits(val as u8)
    }
    #[doc = "WDT Underflow/Refresh Error Status Flag"]
    #[inline(always)]
    pub const fn set_wdtst(&mut self, val: super::vals::Wdtst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Voltage-Monitoring 1 Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd1st(&self) -> super::vals::Lvd1st {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Lvd1st::from_bits(val as u8)
    }
    #[doc = "Voltage-Monitoring 1 Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_lvd1st(&mut self, val: super::vals::Lvd1st) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Voltage-Monitoring 2 Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd2st(&self) -> super::vals::Lvd2st {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lvd2st::from_bits(val as u8)
    }
    #[doc = "Voltage-Monitoring 2 Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_lvd2st(&mut self, val: super::vals::Lvd2st) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "VBATT monitor Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn vbattst(&self) -> super::vals::Vbattst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Vbattst::from_bits(val as u8)
    }
    #[doc = "VBATT monitor Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_vbattst(&mut self, val: super::vals::Vbattst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Oscillation Stop Detection Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ostst(&self) -> super::vals::Ostst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ostst::from_bits(val as u8)
    }
    #[doc = "Oscillation Stop Detection Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_ostst(&mut self, val: super::vals::Ostst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "NMI Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn nmist(&self) -> super::vals::Nmist {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Nmist::from_bits(val as u8)
    }
    #[doc = "NMI Status Flag"]
    #[inline(always)]
    pub const fn set_nmist(&mut self, val: super::vals::Nmist) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "RAM Parity Error Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rpest(&self) -> super::vals::Rpest {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rpest::from_bits(val as u8)
    }
    #[doc = "RAM Parity Error Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_rpest(&mut self, val: super::vals::Rpest) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "RAM ECC Error Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn reccst(&self) -> super::vals::Reccst {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Reccst::from_bits(val as u8)
    }
    #[doc = "RAM ECC Error Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_reccst(&mut self, val: super::vals::Reccst) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
    #[doc = "MPU Bus Slave Error Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bussst(&self) -> super::vals::Bussst {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Bussst::from_bits(val as u8)
    }
    #[doc = "MPU Bus Slave Error Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_bussst(&mut self, val: super::vals::Bussst) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u16) & 0x01) << 10usize);
    }
    #[doc = "MPU Bus Master Error Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn busmst(&self) -> super::vals::Busmst {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Busmst::from_bits(val as u8)
    }
    #[doc = "MPU Bus Master Error Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_busmst(&mut self, val: super::vals::Busmst) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u16) & 0x01) << 11usize);
    }
    #[doc = "CPU Stack pointer monitor Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn spest(&self) -> super::vals::Spest {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Spest::from_bits(val as u8)
    }
    #[doc = "CPU Stack pointer monitor Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_spest(&mut self, val: super::vals::Spest) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
}
impl Default for Nmisr {
    #[inline(always)]
    fn default() -> Nmisr {
        Nmisr(0)
    }
}
impl core::fmt::Debug for Nmisr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nmisr")
            .field("iwdtst", &self.iwdtst())
            .field("wdtst", &self.wdtst())
            .field("lvd1st", &self.lvd1st())
            .field("lvd2st", &self.lvd2st())
            .field("vbattst", &self.vbattst())
            .field("ostst", &self.ostst())
            .field("nmist", &self.nmist())
            .field("rpest", &self.rpest())
            .field("reccst", &self.reccst())
            .field("bussst", &self.bussst())
            .field("busmst", &self.busmst())
            .field("spest", &self.spest())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nmisr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Nmisr {{ iwdtst: {:?}, wdtst: {:?}, lvd1st: {:?}, lvd2st: {:?}, vbattst: {:?}, ostst: {:?}, nmist: {:?}, rpest: {:?}, reccst: {:?}, bussst: {:?}, busmst: {:?}, spest: {:?} }}" , self . iwdtst () , self . wdtst () , self . lvd1st () , self . lvd2st () , self . vbattst () , self . ostst () , self . nmist () , self . rpest () , self . reccst () , self . bussst () , self . busmst () , self . spest ())
    }
}
#[doc = "SYS Event Link Setting Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Selsr0(pub u16);
impl Selsr0 {
    #[doc = "SYS Event Link Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sels(&self) -> super::vals::Sels {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Sels::from_bits(val as u8)
    }
    #[doc = "SYS Event Link Select"]
    #[inline(always)]
    pub const fn set_sels(&mut self, val: super::vals::Sels) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
}
impl Default for Selsr0 {
    #[inline(always)]
    fn default() -> Selsr0 {
        Selsr0(0)
    }
}
impl core::fmt::Debug for Selsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Selsr0")
            .field("sels", &self.sels())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Selsr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Selsr0 {{ sels: {:?} }}", self.sels())
    }
}
#[doc = "Wake Up Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wupen(pub u32);
impl Wupen {
    #[doc = "IRQ0 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen0(&self) -> super::vals::Irqwupen0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Irqwupen0::from_bits(val as u8)
    }
    #[doc = "IRQ0 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen0(&mut self, val: super::vals::Irqwupen0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IRQ1 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen1(&self) -> super::vals::Irqwupen1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Irqwupen1::from_bits(val as u8)
    }
    #[doc = "IRQ1 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen1(&mut self, val: super::vals::Irqwupen1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IRQ2 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen2(&self) -> super::vals::Irqwupen2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Irqwupen2::from_bits(val as u8)
    }
    #[doc = "IRQ2 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen2(&mut self, val: super::vals::Irqwupen2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "IRQ3 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen3(&self) -> super::vals::Irqwupen3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Irqwupen3::from_bits(val as u8)
    }
    #[doc = "IRQ3 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen3(&mut self, val: super::vals::Irqwupen3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "IRQ4 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen4(&self) -> super::vals::Irqwupen4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Irqwupen4::from_bits(val as u8)
    }
    #[doc = "IRQ4 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen4(&mut self, val: super::vals::Irqwupen4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IRQ5 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen5(&self) -> super::vals::Irqwupen5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Irqwupen5::from_bits(val as u8)
    }
    #[doc = "IRQ5 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen5(&mut self, val: super::vals::Irqwupen5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IRQ6 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen6(&self) -> super::vals::Irqwupen6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Irqwupen6::from_bits(val as u8)
    }
    #[doc = "IRQ6 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen6(&mut self, val: super::vals::Irqwupen6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "IRQ7 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen7(&self) -> super::vals::Irqwupen7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Irqwupen7::from_bits(val as u8)
    }
    #[doc = "IRQ7 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen7(&mut self, val: super::vals::Irqwupen7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "IRQ8 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen8(&self) -> super::vals::Irqwupen8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Irqwupen8::from_bits(val as u8)
    }
    #[doc = "IRQ8 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen8(&mut self, val: super::vals::Irqwupen8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "IRQ9 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen9(&self) -> super::vals::Irqwupen9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Irqwupen9::from_bits(val as u8)
    }
    #[doc = "IRQ9 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen9(&mut self, val: super::vals::Irqwupen9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "IRQ10 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen10(&self) -> super::vals::Irqwupen10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Irqwupen10::from_bits(val as u8)
    }
    #[doc = "IRQ10 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen10(&mut self, val: super::vals::Irqwupen10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "IRQ11 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen11(&self) -> super::vals::Irqwupen11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Irqwupen11::from_bits(val as u8)
    }
    #[doc = "IRQ11 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen11(&mut self, val: super::vals::Irqwupen11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "IRQ12 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen12(&self) -> super::vals::Irqwupen12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Irqwupen12::from_bits(val as u8)
    }
    #[doc = "IRQ12 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen12(&mut self, val: super::vals::Irqwupen12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "IRQ14 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen14(&self) -> super::vals::Irqwupen14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Irqwupen14::from_bits(val as u8)
    }
    #[doc = "IRQ14 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen14(&mut self, val: super::vals::Irqwupen14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "IRQ15 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irqwupen15(&self) -> super::vals::Irqwupen15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Irqwupen15::from_bits(val as u8)
    }
    #[doc = "IRQ15 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_irqwupen15(&mut self, val: super::vals::Irqwupen15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "IWDT interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iwdtwupen(&self) -> super::vals::Iwdtwupen {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Iwdtwupen::from_bits(val as u8)
    }
    #[doc = "IWDT interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_iwdtwupen(&mut self, val: super::vals::Iwdtwupen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Key interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn keywupen(&self) -> super::vals::Keywupen {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Keywupen::from_bits(val as u8)
    }
    #[doc = "Key interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_keywupen(&mut self, val: super::vals::Keywupen) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "LVD1 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd1wupen(&self) -> super::vals::Lvd1wupen {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Lvd1wupen::from_bits(val as u8)
    }
    #[doc = "LVD1 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_lvd1wupen(&mut self, val: super::vals::Lvd1wupen) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "LVD2 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvd2wupen(&self) -> super::vals::Lvd2wupen {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Lvd2wupen::from_bits(val as u8)
    }
    #[doc = "LVD2 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_lvd2wupen(&mut self, val: super::vals::Lvd2wupen) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "VBATT monitor interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vbattwupen(&self) -> super::vals::Vbattwupen {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Vbattwupen::from_bits(val as u8)
    }
    #[doc = "VBATT monitor interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_vbattwupen(&mut self, val: super::vals::Vbattwupen) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "ACMPLP0 interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn acmplp0wupen(&self) -> super::vals::Acmplp0wupen {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Acmplp0wupen::from_bits(val as u8)
    }
    #[doc = "ACMPLP0 interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_acmplp0wupen(&mut self, val: super::vals::Acmplp0wupen) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "RTC alarm interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rtcalmwupen(&self) -> super::vals::Rtcalmwupen {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Rtcalmwupen::from_bits(val as u8)
    }
    #[doc = "RTC alarm interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_rtcalmwupen(&mut self, val: super::vals::Rtcalmwupen) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "RCT period interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rtcprdwupen(&self) -> super::vals::Rtcprdwupen {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Rtcprdwupen::from_bits(val as u8)
    }
    #[doc = "RCT period interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_rtcprdwupen(&mut self, val: super::vals::Rtcprdwupen) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "USBFS interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usbfswupen(&self) -> super::vals::Usbfswupen {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Usbfswupen::from_bits(val as u8)
    }
    #[doc = "USBFS interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_usbfswupen(&mut self, val: super::vals::Usbfswupen) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "AGT1 underflow interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn agt1udwupen(&self) -> super::vals::Agt1udwupen {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Agt1udwupen::from_bits(val as u8)
    }
    #[doc = "AGT1 underflow interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_agt1udwupen(&mut self, val: super::vals::Agt1udwupen) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "AGT1 compare match A interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn agt1cawupen(&self) -> super::vals::Agt1cawupen {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Agt1cawupen::from_bits(val as u8)
    }
    #[doc = "AGT1 compare match A interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_agt1cawupen(&mut self, val: super::vals::Agt1cawupen) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "AGT1 compare match B interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn agt1cbwupen(&self) -> super::vals::Agt1cbwupen {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Agt1cbwupen::from_bits(val as u8)
    }
    #[doc = "AGT1 compare match B interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_agt1cbwupen(&mut self, val: super::vals::Agt1cbwupen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "IIC0 address match interrupt S/W standby returns enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iic0wupen(&self) -> super::vals::Iic0wupen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Iic0wupen::from_bits(val as u8)
    }
    #[doc = "IIC0 address match interrupt S/W standby returns enable"]
    #[inline(always)]
    pub const fn set_iic0wupen(&mut self, val: super::vals::Iic0wupen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Wupen {
    #[inline(always)]
    fn default() -> Wupen {
        Wupen(0)
    }
}
impl core::fmt::Debug for Wupen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wupen")
            .field("irqwupen0", &self.irqwupen0())
            .field("irqwupen1", &self.irqwupen1())
            .field("irqwupen2", &self.irqwupen2())
            .field("irqwupen3", &self.irqwupen3())
            .field("irqwupen4", &self.irqwupen4())
            .field("irqwupen5", &self.irqwupen5())
            .field("irqwupen6", &self.irqwupen6())
            .field("irqwupen7", &self.irqwupen7())
            .field("irqwupen8", &self.irqwupen8())
            .field("irqwupen9", &self.irqwupen9())
            .field("irqwupen10", &self.irqwupen10())
            .field("irqwupen11", &self.irqwupen11())
            .field("irqwupen12", &self.irqwupen12())
            .field("irqwupen14", &self.irqwupen14())
            .field("irqwupen15", &self.irqwupen15())
            .field("iwdtwupen", &self.iwdtwupen())
            .field("keywupen", &self.keywupen())
            .field("lvd1wupen", &self.lvd1wupen())
            .field("lvd2wupen", &self.lvd2wupen())
            .field("vbattwupen", &self.vbattwupen())
            .field("acmplp0wupen", &self.acmplp0wupen())
            .field("rtcalmwupen", &self.rtcalmwupen())
            .field("rtcprdwupen", &self.rtcprdwupen())
            .field("usbfswupen", &self.usbfswupen())
            .field("agt1udwupen", &self.agt1udwupen())
            .field("agt1cawupen", &self.agt1cawupen())
            .field("agt1cbwupen", &self.agt1cbwupen())
            .field("iic0wupen", &self.iic0wupen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wupen {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wupen {{ irqwupen0: {:?}, irqwupen1: {:?}, irqwupen2: {:?}, irqwupen3: {:?}, irqwupen4: {:?}, irqwupen5: {:?}, irqwupen6: {:?}, irqwupen7: {:?}, irqwupen8: {:?}, irqwupen9: {:?}, irqwupen10: {:?}, irqwupen11: {:?}, irqwupen12: {:?}, irqwupen14: {:?}, irqwupen15: {:?}, iwdtwupen: {:?}, keywupen: {:?}, lvd1wupen: {:?}, lvd2wupen: {:?}, vbattwupen: {:?}, acmplp0wupen: {:?}, rtcalmwupen: {:?}, rtcprdwupen: {:?}, usbfswupen: {:?}, agt1udwupen: {:?}, agt1cawupen: {:?}, agt1cbwupen: {:?}, iic0wupen: {:?} }}" , self . irqwupen0 () , self . irqwupen1 () , self . irqwupen2 () , self . irqwupen3 () , self . irqwupen4 () , self . irqwupen5 () , self . irqwupen6 () , self . irqwupen7 () , self . irqwupen8 () , self . irqwupen9 () , self . irqwupen10 () , self . irqwupen11 () , self . irqwupen12 () , self . irqwupen14 () , self . irqwupen15 () , self . iwdtwupen () , self . keywupen () , self . lvd1wupen () , self . lvd2wupen () , self . vbattwupen () , self . acmplp0wupen () , self . rtcalmwupen () , self . rtcprdwupen () , self . usbfswupen () , self . agt1udwupen () , self . agt1cawupen () , self . agt1cbwupen () , self . iic0wupen ())
    }
}
