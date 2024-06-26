#[doc = "Register `GPIO0B_IOMUX` reader"]
pub type R = crate::R<Gpio0bIomuxSpec>;
#[doc = "Register `GPIO0B_IOMUX` writer"]
pub type W = crate::W<Gpio0bIomuxSpec>;
#[doc = "GPIO0B\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0b0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdmmc_wrprt"]
    B01 = 1,
    #[doc = "2: pmum0_wfi"]
    B10 = 2,
    #[doc = "3: test_clkout2"]
    B11 = 3,
}
impl From<Gpio0b0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0b0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0b0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0B0_SEL` reader - GPIO0B\\[0\\]
iomux select"]
pub type Gpio0b0SelR = crate::FieldReader<Gpio0b0Sel>;
impl Gpio0b0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0b0Sel {
        match self.bits {
            0 => Gpio0b0Sel::B00,
            1 => Gpio0b0Sel::B01,
            2 => Gpio0b0Sel::B10,
            3 => Gpio0b0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0b0Sel::B00
    }
    #[doc = "sdmmc_wrprt"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0b0Sel::B01
    }
    #[doc = "pmum0_wfi"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0b0Sel::B10
    }
    #[doc = "test_clkout2"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio0b0Sel::B11
    }
}
#[doc = "Field `GPIO0B0_SEL` writer - GPIO0B\\[0\\]
iomux select"]
pub type Gpio0b0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio0b0Sel>;
impl<'a, REG> Gpio0b0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b0Sel::B00)
    }
    #[doc = "sdmmc_wrprt"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b0Sel::B01)
    }
    #[doc = "pmum0_wfi"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b0Sel::B10)
    }
    #[doc = "test_clkout2"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b0Sel::B11)
    }
}
#[doc = "GPIO0B\\[1\\]
iomux select\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0b1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: pmu1830_volsel"]
    B01 = 1,
}
impl From<Gpio0b1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0b1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0b1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0B1_SEL` reader - GPIO0B\\[1\\]
iomux select"]
pub type Gpio0b1SelR = crate::FieldReader<Gpio0b1Sel>;
impl Gpio0b1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0b1Sel> {
        match self.bits {
            0 => Some(Gpio0b1Sel::B00),
            1 => Some(Gpio0b1Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0b1Sel::B00
    }
    #[doc = "pmu1830_volsel"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0b1Sel::B01
    }
}
#[doc = "Field `GPIO0B1_SEL` writer - GPIO0B\\[1\\]
iomux select"]
pub type Gpio0b1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0b1Sel>;
impl<'a, REG> Gpio0b1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b1Sel::B00)
    }
    #[doc = "pmu1830_volsel"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b1Sel::B01)
    }
}
#[doc = "GPIO0B\\[2\\]
iomux select\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0b2Sel {
    #[doc = "0: gpio"]
    B00 = 0,
}
impl From<Gpio0b2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0b2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0b2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0B2_SEL` reader - GPIO0B\\[2\\]
iomux select"]
pub type Gpio0b2SelR = crate::FieldReader<Gpio0b2Sel>;
impl Gpio0b2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0b2Sel> {
        match self.bits {
            0 => Some(Gpio0b2Sel::B00),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0b2Sel::B00
    }
}
#[doc = "Field `GPIO0B2_SEL` writer - GPIO0B\\[2\\]
iomux select"]
pub type Gpio0b2SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0b2Sel>;
impl<'a, REG> Gpio0b2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b2Sel::B00)
    }
}
#[doc = "GPIO0B\\[3\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0b3Sel {
    #[doc = "0: gpio"]
    B00 = 0,
}
impl From<Gpio0b3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0b3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0b3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0B3_SEL` reader - GPIO0B\\[3\\]
iomux select"]
pub type Gpio0b3SelR = crate::FieldReader<Gpio0b3Sel>;
impl Gpio0b3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0b3Sel> {
        match self.bits {
            0 => Some(Gpio0b3Sel::B00),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0b3Sel::B00
    }
}
#[doc = "Field `GPIO0B3_SEL` writer - GPIO0B\\[3\\]
iomux select"]
pub type Gpio0b3SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0b3Sel>;
impl<'a, REG> Gpio0b3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b3Sel::B00)
    }
}
#[doc = "GPIO0B4\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0b4Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: tcpd_vbusbdis"]
    B01 = 1,
}
impl From<Gpio0b4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0b4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0b4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0B4_SEL` reader - GPIO0B4\\]
iomux select"]
pub type Gpio0b4SelR = crate::FieldReader<Gpio0b4Sel>;
impl Gpio0b4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0b4Sel> {
        match self.bits {
            0 => Some(Gpio0b4Sel::B00),
            1 => Some(Gpio0b4Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0b4Sel::B00
    }
    #[doc = "tcpd_vbusbdis"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0b4Sel::B01
    }
}
#[doc = "Field `GPIO0B4_SEL` writer - GPIO0B4\\]
iomux select"]
pub type Gpio0b4SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0b4Sel>;
impl<'a, REG> Gpio0b4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b4Sel::B00)
    }
    #[doc = "tcpd_vbusbdis"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b4Sel::B01)
    }
}
#[doc = "GPIO0B\\[5\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0b5Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: tcpd_vbusfdis"]
    B01 = 1,
    #[doc = "2: tcpdusb2_vbussource3"]
    B10 = 2,
}
impl From<Gpio0b5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0b5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0b5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0B5_SEL` reader - GPIO0B\\[5\\]
iomux select"]
pub type Gpio0b5SelR = crate::FieldReader<Gpio0b5Sel>;
impl Gpio0b5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0b5Sel> {
        match self.bits {
            0 => Some(Gpio0b5Sel::B00),
            1 => Some(Gpio0b5Sel::B01),
            2 => Some(Gpio0b5Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0b5Sel::B00
    }
    #[doc = "tcpd_vbusfdis"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0b5Sel::B01
    }
    #[doc = "tcpdusb2_vbussource3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0b5Sel::B10
    }
}
#[doc = "Field `GPIO0B5_SEL` writer - GPIO0B\\[5\\]
iomux select"]
pub type Gpio0b5SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0b5Sel>;
impl<'a, REG> Gpio0b5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b5Sel::B00)
    }
    #[doc = "tcpd_vbusfdis"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b5Sel::B01)
    }
    #[doc = "tcpdusb2_vbussource3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0b5Sel::B10)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO0B\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0b0_sel(&self) -> Gpio0b0SelR {
        Gpio0b0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO0B\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0b1_sel(&self) -> Gpio0b1SelR {
        Gpio0b1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO0B\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0b2_sel(&self) -> Gpio0b2SelR {
        Gpio0b2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO0B\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0b3_sel(&self) -> Gpio0b3SelR {
        Gpio0b3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO0B4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0b4_sel(&self) -> Gpio0b4SelR {
        Gpio0b4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO0B\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0b5_sel(&self) -> Gpio0b5SelR {
        Gpio0b5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO0B\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b0_sel(&mut self) -> Gpio0b0SelW<Gpio0bIomuxSpec> {
        Gpio0b0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO0B\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b1_sel(&mut self) -> Gpio0b1SelW<Gpio0bIomuxSpec> {
        Gpio0b1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO0B\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b2_sel(&mut self) -> Gpio0b2SelW<Gpio0bIomuxSpec> {
        Gpio0b2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO0B\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b3_sel(&mut self) -> Gpio0b3SelW<Gpio0bIomuxSpec> {
        Gpio0b3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO0B4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b4_sel(&mut self) -> Gpio0b4SelW<Gpio0bIomuxSpec> {
        Gpio0b4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO0B\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b5_sel(&mut self) -> Gpio0b5SelW<Gpio0bIomuxSpec> {
        Gpio0b5SelW::new(self, 10)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio0bIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO0B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0b_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0b_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0bIomuxSpec;
impl crate::RegisterSpec for Gpio0bIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0b_iomux::R`](R) reader structure"]
impl crate::Readable for Gpio0bIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio0b_iomux::W`](W) writer structure"]
impl crate::Writable for Gpio0bIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO0B_IOMUX to value 0x14"]
impl crate::Resettable for Gpio0bIomuxSpec {
    const RESET_VALUE: u32 = 0x14;
}
