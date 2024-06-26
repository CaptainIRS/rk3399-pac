#[doc = "Register `GPIO1B_IOMUX` reader"]
pub type R = crate::R<Gpio1bIomuxSpec>;
#[doc = "Register `GPIO1B_IOMUX` writer"]
pub type W = crate::W<Gpio1bIomuxSpec>;
#[doc = "GPIO1B\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1b0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: uart4m0_sout"]
    B01 = 1,
    #[doc = "2: spi1ec_txd"]
    B10 = 2,
}
impl From<Gpio1b0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1b0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1b0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1B0_SEL` reader - GPIO1B\\[0\\]
iomux select"]
pub type Gpio1b0SelR = crate::FieldReader<Gpio1b0Sel>;
impl Gpio1b0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1b0Sel> {
        match self.bits {
            0 => Some(Gpio1b0Sel::B00),
            1 => Some(Gpio1b0Sel::B01),
            2 => Some(Gpio1b0Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1b0Sel::B00
    }
    #[doc = "uart4m0_sout"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1b0Sel::B01
    }
    #[doc = "spi1ec_txd"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1b0Sel::B10
    }
}
#[doc = "Field `GPIO1B0_SEL` writer - GPIO1B\\[0\\]
iomux select"]
pub type Gpio1b0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1b0Sel>;
impl<'a, REG> Gpio1b0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b0Sel::B00)
    }
    #[doc = "uart4m0_sout"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b0Sel::B01)
    }
    #[doc = "spi1ec_txd"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b0Sel::B10)
    }
}
#[doc = "GPIO1B\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1b1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: pmum0jtag_tck"]
    B01 = 1,
    #[doc = "2: spi1ec_clk"]
    B10 = 2,
}
impl From<Gpio1b1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1b1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1b1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1B1_SEL` reader - GPIO1B\\[1\\]
iomux select"]
pub type Gpio1b1SelR = crate::FieldReader<Gpio1b1Sel>;
impl Gpio1b1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1b1Sel> {
        match self.bits {
            0 => Some(Gpio1b1Sel::B00),
            1 => Some(Gpio1b1Sel::B01),
            2 => Some(Gpio1b1Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1b1Sel::B00
    }
    #[doc = "pmum0jtag_tck"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1b1Sel::B01
    }
    #[doc = "spi1ec_clk"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1b1Sel::B10
    }
}
#[doc = "Field `GPIO1B1_SEL` writer - GPIO1B\\[1\\]
iomux select"]
pub type Gpio1b1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1b1Sel>;
impl<'a, REG> Gpio1b1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b1Sel::B00)
    }
    #[doc = "pmum0jtag_tck"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b1Sel::B01)
    }
    #[doc = "spi1ec_clk"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b1Sel::B10)
    }
}
#[doc = "GPIO1B\\[2\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1b2Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: pmum0jtag_tms"]
    B01 = 1,
    #[doc = "2: spi1ec_csn0"]
    B10 = 2,
}
impl From<Gpio1b2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1b2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1b2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1B2_SEL` reader - GPIO1B\\[2\\]
iomux select"]
pub type Gpio1b2SelR = crate::FieldReader<Gpio1b2Sel>;
impl Gpio1b2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1b2Sel> {
        match self.bits {
            0 => Some(Gpio1b2Sel::B00),
            1 => Some(Gpio1b2Sel::B01),
            2 => Some(Gpio1b2Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1b2Sel::B00
    }
    #[doc = "pmum0jtag_tms"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1b2Sel::B01
    }
    #[doc = "spi1ec_csn0"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1b2Sel::B10
    }
}
#[doc = "Field `GPIO1B2_SEL` writer - GPIO1B\\[2\\]
iomux select"]
pub type Gpio1b2SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1b2Sel>;
impl<'a, REG> Gpio1b2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b2Sel::B00)
    }
    #[doc = "pmum0jtag_tms"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b2Sel::B01)
    }
    #[doc = "spi1ec_csn0"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b2Sel::B10)
    }
}
#[doc = "GPIO1B\\[3\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1b3Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: i2c4sensor_sda"]
    B01 = 1,
}
impl From<Gpio1b3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1b3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1b3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1B3_SEL` reader - GPIO1B\\[3\\]
iomux select"]
pub type Gpio1b3SelR = crate::FieldReader<Gpio1b3Sel>;
impl Gpio1b3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1b3Sel> {
        match self.bits {
            0 => Some(Gpio1b3Sel::B00),
            1 => Some(Gpio1b3Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1b3Sel::B00
    }
    #[doc = "i2c4sensor_sda"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1b3Sel::B01
    }
}
#[doc = "Field `GPIO1B3_SEL` writer - GPIO1B\\[3\\]
iomux select"]
pub type Gpio1b3SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1b3Sel>;
impl<'a, REG> Gpio1b3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b3Sel::B00)
    }
    #[doc = "i2c4sensor_sda"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b3Sel::B01)
    }
}
#[doc = "GPIO1B\\[4\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1b4Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: i2c4sensor_scl"]
    B01 = 1,
}
impl From<Gpio1b4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1b4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1b4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1B4_SEL` reader - GPIO1B\\[4\\]
iomux select"]
pub type Gpio1b4SelR = crate::FieldReader<Gpio1b4Sel>;
impl Gpio1b4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1b4Sel> {
        match self.bits {
            0 => Some(Gpio1b4Sel::B00),
            1 => Some(Gpio1b4Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1b4Sel::B00
    }
    #[doc = "i2c4sensor_scl"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1b4Sel::B01
    }
}
#[doc = "Field `GPIO1B4_SEL` writer - GPIO1B\\[4\\]
iomux select"]
pub type Gpio1b4SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1b4Sel>;
impl<'a, REG> Gpio1b4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b4Sel::B00)
    }
    #[doc = "i2c4sensor_scl"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b4Sel::B01)
    }
}
#[doc = "GPIO1B\\[5\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1b5Sel {
    #[doc = "0: gpio"]
    B00 = 0,
}
impl From<Gpio1b5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1b5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1b5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1B5_SEL` reader - GPIO1B\\[5\\]
iomux select"]
pub type Gpio1b5SelR = crate::FieldReader<Gpio1b5Sel>;
impl Gpio1b5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1b5Sel> {
        match self.bits {
            0 => Some(Gpio1b5Sel::B00),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1b5Sel::B00
    }
}
#[doc = "Field `GPIO1B5_SEL` writer - GPIO1B\\[5\\]
iomux select"]
pub type Gpio1b5SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1b5Sel>;
impl<'a, REG> Gpio1b5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b5Sel::B00)
    }
}
#[doc = "GPIO1B\\[6\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1b6Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: pwm_3b"]
    B01 = 1,
}
impl From<Gpio1b6Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1b6Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1b6Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1B6_SEL` reader - GPIO1B\\[6\\]
iomux select"]
pub type Gpio1b6SelR = crate::FieldReader<Gpio1b6Sel>;
impl Gpio1b6SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1b6Sel> {
        match self.bits {
            0 => Some(Gpio1b6Sel::B00),
            1 => Some(Gpio1b6Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1b6Sel::B00
    }
    #[doc = "pwm_3b"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1b6Sel::B01
    }
}
#[doc = "Field `GPIO1B6_SEL` writer - GPIO1B\\[6\\]
iomux select"]
pub type Gpio1b6SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1b6Sel>;
impl<'a, REG> Gpio1b6SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b6Sel::B00)
    }
    #[doc = "pwm_3b"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b6Sel::B01)
    }
}
#[doc = "GPIO1B\\[7\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1b7Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: spi3pmu_rxd"]
    B01 = 1,
    #[doc = "2: i2c0pmu_scl"]
    B10 = 2,
}
impl From<Gpio1b7Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1b7Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1b7Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1B7_SEL` reader - GPIO1B\\[7\\]
iomux select"]
pub type Gpio1b7SelR = crate::FieldReader<Gpio1b7Sel>;
impl Gpio1b7SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1b7Sel> {
        match self.bits {
            0 => Some(Gpio1b7Sel::B00),
            1 => Some(Gpio1b7Sel::B01),
            2 => Some(Gpio1b7Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1b7Sel::B00
    }
    #[doc = "spi3pmu_rxd"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1b7Sel::B01
    }
    #[doc = "i2c0pmu_scl"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1b7Sel::B10
    }
}
#[doc = "Field `GPIO1B7_SEL` writer - GPIO1B\\[7\\]
iomux select"]
pub type Gpio1b7SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1b7Sel>;
impl<'a, REG> Gpio1b7SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b7Sel::B00)
    }
    #[doc = "spi3pmu_rxd"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b7Sel::B01)
    }
    #[doc = "i2c0pmu_scl"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1b7Sel::B10)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO1B\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1b0_sel(&self) -> Gpio1b0SelR {
        Gpio1b0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO1B\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1b1_sel(&self) -> Gpio1b1SelR {
        Gpio1b1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO1B\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1b2_sel(&self) -> Gpio1b2SelR {
        Gpio1b2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO1B\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1b3_sel(&self) -> Gpio1b3SelR {
        Gpio1b3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO1B\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1b4_sel(&self) -> Gpio1b4SelR {
        Gpio1b4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO1B\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1b5_sel(&self) -> Gpio1b5SelR {
        Gpio1b5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO1B\\[6\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1b6_sel(&self) -> Gpio1b6SelR {
        Gpio1b6SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO1B\\[7\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1b7_sel(&self) -> Gpio1b7SelR {
        Gpio1b7SelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO1B\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b0_sel(&mut self) -> Gpio1b0SelW<Gpio1bIomuxSpec> {
        Gpio1b0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO1B\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b1_sel(&mut self) -> Gpio1b1SelW<Gpio1bIomuxSpec> {
        Gpio1b1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO1B\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b2_sel(&mut self) -> Gpio1b2SelW<Gpio1bIomuxSpec> {
        Gpio1b2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO1B\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b3_sel(&mut self) -> Gpio1b3SelW<Gpio1bIomuxSpec> {
        Gpio1b3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO1B\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b4_sel(&mut self) -> Gpio1b4SelW<Gpio1bIomuxSpec> {
        Gpio1b4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO1B\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b5_sel(&mut self) -> Gpio1b5SelW<Gpio1bIomuxSpec> {
        Gpio1b5SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO1B\\[6\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b6_sel(&mut self) -> Gpio1b6SelW<Gpio1bIomuxSpec> {
        Gpio1b6SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO1B\\[7\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b7_sel(&mut self) -> Gpio1b7SelW<Gpio1bIomuxSpec> {
        Gpio1b7SelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio1bIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO1B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1b_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1b_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio1bIomuxSpec;
impl crate::RegisterSpec for Gpio1bIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio1b_iomux::R`](R) reader structure"]
impl crate::Readable for Gpio1bIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio1b_iomux::W`](W) writer structure"]
impl crate::Writable for Gpio1bIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO1B_IOMUX to value 0"]
impl crate::Resettable for Gpio1bIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
