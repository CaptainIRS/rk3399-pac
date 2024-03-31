#[doc = "Register `GPIO0A_IOMUX` reader"]
pub type R = crate::R<Gpio0aIomuxSpec>;
#[doc = "Register `GPIO0A_IOMUX` writer"]
pub type W = crate::W<Gpio0aIomuxSpec>;
#[doc = "GPIO0A\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0a0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: test_clkout0"]
    B01 = 1,
    #[doc = "2: clk_32k"]
    B10 = 2,
}
impl From<Gpio0a0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0a0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0a0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0A0_SEL` reader - GPIO0A\\[0\\]
iomux select"]
pub type Gpio0a0SelR = crate::FieldReader<Gpio0a0Sel>;
impl Gpio0a0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0a0Sel> {
        match self.bits {
            0 => Some(Gpio0a0Sel::B00),
            1 => Some(Gpio0a0Sel::B01),
            2 => Some(Gpio0a0Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0a0Sel::B00
    }
    #[doc = "test_clkout0"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0a0Sel::B01
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0a0Sel::B10
    }
}
#[doc = "Field `GPIO0A0_SEL` writer - GPIO0A\\[0\\]
iomux select"]
pub type Gpio0a0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0a0Sel>;
impl<'a, REG> Gpio0a0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a0Sel::B00)
    }
    #[doc = "test_clkout0"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a0Sel::B01)
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a0Sel::B10)
    }
}
#[doc = "GPIO0A\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0a1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: ddrio_pwroff"]
    B01 = 1,
    #[doc = "2: tcpd_ccdben"]
    B10 = 2,
}
impl From<Gpio0a1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0a1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0a1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0A1_SEL` reader - GPIO0A\\[1\\]
iomux select"]
pub type Gpio0a1SelR = crate::FieldReader<Gpio0a1Sel>;
impl Gpio0a1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0a1Sel> {
        match self.bits {
            0 => Some(Gpio0a1Sel::B00),
            1 => Some(Gpio0a1Sel::B01),
            2 => Some(Gpio0a1Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0a1Sel::B00
    }
    #[doc = "ddrio_pwroff"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0a1Sel::B01
    }
    #[doc = "tcpd_ccdben"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0a1Sel::B10
    }
}
#[doc = "Field `GPIO0A1_SEL` writer - GPIO0A\\[1\\]
iomux select"]
pub type Gpio0a1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0a1Sel>;
impl<'a, REG> Gpio0a1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a1Sel::B00)
    }
    #[doc = "ddrio_pwroff"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a1Sel::B01)
    }
    #[doc = "tcpd_ccdben"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a1Sel::B10)
    }
}
#[doc = "GPIO0A\\[2\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0a2Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: wifi_26m"]
    B01 = 1,
    #[doc = "2: pmu_debug0"]
    B10 = 2,
}
impl From<Gpio0a2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0a2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0a2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0A2_SEL` reader - GPIO0A\\[2\\]
iomux select"]
pub type Gpio0a2SelR = crate::FieldReader<Gpio0a2Sel>;
impl Gpio0a2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0a2Sel> {
        match self.bits {
            0 => Some(Gpio0a2Sel::B00),
            1 => Some(Gpio0a2Sel::B01),
            2 => Some(Gpio0a2Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0a2Sel::B00
    }
    #[doc = "wifi_26m"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0a2Sel::B01
    }
    #[doc = "pmu_debug0"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0a2Sel::B10
    }
}
#[doc = "Field `GPIO0A2_SEL` writer - GPIO0A\\[2\\]
iomux select"]
pub type Gpio0a2SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0a2Sel>;
impl<'a, REG> Gpio0a2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a2Sel::B00)
    }
    #[doc = "wifi_26m"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a2Sel::B01)
    }
    #[doc = "pmu_debug0"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a2Sel::B10)
    }
}
#[doc = "GPIO0A\\[3\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0a3Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdio_wrprt"]
    B01 = 1,
    #[doc = "2: pmu_debug1"]
    B10 = 2,
}
impl From<Gpio0a3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0a3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0a3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0A3_SEL` reader - GPIO0A\\[3\\]
iomux select"]
pub type Gpio0a3SelR = crate::FieldReader<Gpio0a3Sel>;
impl Gpio0a3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0a3Sel> {
        match self.bits {
            0 => Some(Gpio0a3Sel::B00),
            1 => Some(Gpio0a3Sel::B01),
            2 => Some(Gpio0a3Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0a3Sel::B00
    }
    #[doc = "sdio_wrprt"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0a3Sel::B01
    }
    #[doc = "pmu_debug1"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0a3Sel::B10
    }
}
#[doc = "Field `GPIO0A3_SEL` writer - GPIO0A\\[3\\]
iomux select"]
pub type Gpio0a3SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0a3Sel>;
impl<'a, REG> Gpio0a3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a3Sel::B00)
    }
    #[doc = "sdio_wrprt"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a3Sel::B01)
    }
    #[doc = "pmu_debug1"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a3Sel::B10)
    }
}
#[doc = "GPIO0A\\[4\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0a4Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdio_intn"]
    B01 = 1,
    #[doc = "2: pmu_debug2"]
    B10 = 2,
}
impl From<Gpio0a4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0a4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0a4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0A4_SEL` reader - GPIO0A\\[4\\]
iomux select"]
pub type Gpio0a4SelR = crate::FieldReader<Gpio0a4Sel>;
impl Gpio0a4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0a4Sel> {
        match self.bits {
            0 => Some(Gpio0a4Sel::B00),
            1 => Some(Gpio0a4Sel::B01),
            2 => Some(Gpio0a4Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0a4Sel::B00
    }
    #[doc = "sdio_intn"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0a4Sel::B01
    }
    #[doc = "pmu_debug2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0a4Sel::B10
    }
}
#[doc = "Field `GPIO0A4_SEL` writer - GPIO0A\\[4\\]
iomux select"]
pub type Gpio0a4SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0a4Sel>;
impl<'a, REG> Gpio0a4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a4Sel::B00)
    }
    #[doc = "sdio_intn"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a4Sel::B01)
    }
    #[doc = "pmu_debug2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a4Sel::B10)
    }
}
#[doc = "GPIO0A\\[5\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0a5Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: emmc_pwren"]
    B01 = 1,
    #[doc = "2: pmu_debug3"]
    B10 = 2,
}
impl From<Gpio0a5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0a5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0a5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0A5_SEL` reader - GPIO0A\\[5\\]
iomux select"]
pub type Gpio0a5SelR = crate::FieldReader<Gpio0a5Sel>;
impl Gpio0a5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0a5Sel> {
        match self.bits {
            0 => Some(Gpio0a5Sel::B00),
            1 => Some(Gpio0a5Sel::B01),
            2 => Some(Gpio0a5Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0a5Sel::B00
    }
    #[doc = "emmc_pwren"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0a5Sel::B01
    }
    #[doc = "pmu_debug3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0a5Sel::B10
    }
}
#[doc = "Field `GPIO0A5_SEL` writer - GPIO0A\\[5\\]
iomux select"]
pub type Gpio0a5SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0a5Sel>;
impl<'a, REG> Gpio0a5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a5Sel::B00)
    }
    #[doc = "emmc_pwren"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a5Sel::B01)
    }
    #[doc = "pmu_debug3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a5Sel::B10)
    }
}
#[doc = "GPIO0A\\[6\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0a6Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: pwm_3a"]
    B01 = 1,
    #[doc = "2: pmu_debug4"]
    B10 = 2,
}
impl From<Gpio0a6Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0a6Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0a6Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0A6_SEL` reader - GPIO0A\\[6\\]
iomux select"]
pub type Gpio0a6SelR = crate::FieldReader<Gpio0a6Sel>;
impl Gpio0a6SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0a6Sel> {
        match self.bits {
            0 => Some(Gpio0a6Sel::B00),
            1 => Some(Gpio0a6Sel::B01),
            2 => Some(Gpio0a6Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0a6Sel::B00
    }
    #[doc = "pwm_3a"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0a6Sel::B01
    }
    #[doc = "pmu_debug4"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0a6Sel::B10
    }
}
#[doc = "Field `GPIO0A6_SEL` writer - GPIO0A\\[6\\]
iomux select"]
pub type Gpio0a6SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0a6Sel>;
impl<'a, REG> Gpio0a6SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a6Sel::B00)
    }
    #[doc = "pwm_3a"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a6Sel::B01)
    }
    #[doc = "pmu_debug4"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a6Sel::B10)
    }
}
#[doc = "GPIO0A\\[7\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0a7Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdmmc_dectn"]
    B01 = 1,
    #[doc = "2: pmu_debug5"]
    B10 = 2,
}
impl From<Gpio0a7Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0a7Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0a7Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO0A7_SEL` reader - GPIO0A\\[7\\]
iomux select"]
pub type Gpio0a7SelR = crate::FieldReader<Gpio0a7Sel>;
impl Gpio0a7SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0a7Sel> {
        match self.bits {
            0 => Some(Gpio0a7Sel::B00),
            1 => Some(Gpio0a7Sel::B01),
            2 => Some(Gpio0a7Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0a7Sel::B00
    }
    #[doc = "sdmmc_dectn"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0a7Sel::B01
    }
    #[doc = "pmu_debug5"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0a7Sel::B10
    }
}
#[doc = "Field `GPIO0A7_SEL` writer - GPIO0A\\[7\\]
iomux select"]
pub type Gpio0a7SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0a7Sel>;
impl<'a, REG> Gpio0a7SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a7Sel::B00)
    }
    #[doc = "sdmmc_dectn"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a7Sel::B01)
    }
    #[doc = "pmu_debug5"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0a7Sel::B10)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO0A\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0a0_sel(&self) -> Gpio0a0SelR {
        Gpio0a0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO0A\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0a1_sel(&self) -> Gpio0a1SelR {
        Gpio0a1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO0A\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0a2_sel(&self) -> Gpio0a2SelR {
        Gpio0a2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO0A\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0a3_sel(&self) -> Gpio0a3SelR {
        Gpio0a3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO0A\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0a4_sel(&self) -> Gpio0a4SelR {
        Gpio0a4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO0A\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0a5_sel(&self) -> Gpio0a5SelR {
        Gpio0a5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO0A\\[6\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0a6_sel(&self) -> Gpio0a6SelR {
        Gpio0a6SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO0A\\[7\\]
iomux select"]
    #[inline(always)]
    pub fn gpio0a7_sel(&self) -> Gpio0a7SelR {
        Gpio0a7SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO0A\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a0_sel(&mut self) -> Gpio0a0SelW<Gpio0aIomuxSpec> {
        Gpio0a0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO0A\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a1_sel(&mut self) -> Gpio0a1SelW<Gpio0aIomuxSpec> {
        Gpio0a1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO0A\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a2_sel(&mut self) -> Gpio0a2SelW<Gpio0aIomuxSpec> {
        Gpio0a2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO0A\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a3_sel(&mut self) -> Gpio0a3SelW<Gpio0aIomuxSpec> {
        Gpio0a3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO0A\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a4_sel(&mut self) -> Gpio0a4SelW<Gpio0aIomuxSpec> {
        Gpio0a4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO0A\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a5_sel(&mut self) -> Gpio0a5SelW<Gpio0aIomuxSpec> {
        Gpio0a5SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO0A\\[6\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a6_sel(&mut self) -> Gpio0a6SelW<Gpio0aIomuxSpec> {
        Gpio0a6SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO0A\\[7\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a7_sel(&mut self) -> Gpio0a7SelW<Gpio0aIomuxSpec> {
        Gpio0a7SelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio0aIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO0A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0a_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0a_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0aIomuxSpec;
impl crate::RegisterSpec for Gpio0aIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0a_iomux::R`](R) reader structure"]
impl crate::Readable for Gpio0aIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio0a_iomux::W`](W) writer structure"]
impl crate::Writable for Gpio0aIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO0A_IOMUX to value 0"]
impl crate::Resettable for Gpio0aIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
