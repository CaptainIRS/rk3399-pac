#[doc = "Register `GPIO1A_IOMUX` reader"]
pub type R = crate::R<Gpio1aIomuxSpec>;
#[doc = "Register `GPIO1A_IOMUX` writer"]
pub type W = crate::W<Gpio1aIomuxSpec>;
#[doc = "GPIO1A\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1a0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: isp0_shutteren"]
    B01 = 1,
    #[doc = "2: isp1_shutteren"]
    B10 = 2,
    #[doc = "3: tcpd_vbussink"]
    B11 = 3,
}
impl From<Gpio1a0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1a0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1a0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1A0_SEL` reader - GPIO1A\\[0\\]
iomux select"]
pub type Gpio1a0SelR = crate::FieldReader<Gpio1a0Sel>;
impl Gpio1a0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1a0Sel {
        match self.bits {
            0 => Gpio1a0Sel::B00,
            1 => Gpio1a0Sel::B01,
            2 => Gpio1a0Sel::B10,
            3 => Gpio1a0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1a0Sel::B00
    }
    #[doc = "isp0_shutteren"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1a0Sel::B01
    }
    #[doc = "isp1_shutteren"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1a0Sel::B10
    }
    #[doc = "tcpd_vbussink"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1a0Sel::B11
    }
}
#[doc = "Field `GPIO1A0_SEL` writer - GPIO1A\\[0\\]
iomux select"]
pub type Gpio1a0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio1a0Sel>;
impl<'a, REG> Gpio1a0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a0Sel::B00)
    }
    #[doc = "isp0_shutteren"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a0Sel::B01)
    }
    #[doc = "isp1_shutteren"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a0Sel::B10)
    }
    #[doc = "tcpd_vbussink"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a0Sel::B11)
    }
}
#[doc = "GPIO1A\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1a1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: isp0_shuttertrig"]
    B01 = 1,
    #[doc = "2: isp1_shuttertrig"]
    B10 = 2,
    #[doc = "3: tcpd_cc0vconn"]
    B11 = 3,
}
impl From<Gpio1a1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1a1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1a1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1A1_SEL` reader - GPIO1A\\[1\\]
iomux select"]
pub type Gpio1a1SelR = crate::FieldReader<Gpio1a1Sel>;
impl Gpio1a1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1a1Sel {
        match self.bits {
            0 => Gpio1a1Sel::B00,
            1 => Gpio1a1Sel::B01,
            2 => Gpio1a1Sel::B10,
            3 => Gpio1a1Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1a1Sel::B00
    }
    #[doc = "isp0_shuttertrig"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1a1Sel::B01
    }
    #[doc = "isp1_shuttertrig"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1a1Sel::B10
    }
    #[doc = "tcpd_cc0vconn"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1a1Sel::B11
    }
}
#[doc = "Field `GPIO1A1_SEL` writer - GPIO1A\\[1\\]
iomux select"]
pub type Gpio1a1SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio1a1Sel>;
impl<'a, REG> Gpio1a1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a1Sel::B00)
    }
    #[doc = "isp0_shuttertrig"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a1Sel::B01)
    }
    #[doc = "isp1_shuttertrig"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a1Sel::B10)
    }
    #[doc = "tcpd_cc0vconn"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a1Sel::B11)
    }
}
#[doc = "GPIO1A\\[2\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1a2Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: isp0_flashtrigin"]
    B01 = 1,
    #[doc = "2: isp1_flashtrigin"]
    B10 = 2,
    #[doc = "3: tcpd_cc1vconn"]
    B11 = 3,
}
impl From<Gpio1a2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1a2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1a2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1A2_SEL` reader - GPIO1A\\[2\\]
iomux select"]
pub type Gpio1a2SelR = crate::FieldReader<Gpio1a2Sel>;
impl Gpio1a2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1a2Sel {
        match self.bits {
            0 => Gpio1a2Sel::B00,
            1 => Gpio1a2Sel::B01,
            2 => Gpio1a2Sel::B10,
            3 => Gpio1a2Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1a2Sel::B00
    }
    #[doc = "isp0_flashtrigin"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1a2Sel::B01
    }
    #[doc = "isp1_flashtrigin"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1a2Sel::B10
    }
    #[doc = "tcpd_cc1vconn"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1a2Sel::B11
    }
}
#[doc = "Field `GPIO1A2_SEL` writer - GPIO1A\\[2\\]
iomux select"]
pub type Gpio1a2SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio1a2Sel>;
impl<'a, REG> Gpio1a2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a2Sel::B00)
    }
    #[doc = "isp0_flashtrigin"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a2Sel::B01)
    }
    #[doc = "isp1_flashtrigin"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a2Sel::B10)
    }
    #[doc = "tcpd_cc1vconn"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a2Sel::B11)
    }
}
#[doc = "GPIO1A\\[3\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1a3Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: isp0_flashtrigout"]
    B01 = 1,
    #[doc = "2: isp1_flashtrigout"]
    B10 = 2,
}
impl From<Gpio1a3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1a3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1a3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1A3_SEL` reader - GPIO1A\\[3\\]
iomux select"]
pub type Gpio1a3SelR = crate::FieldReader<Gpio1a3Sel>;
impl Gpio1a3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1a3Sel> {
        match self.bits {
            0 => Some(Gpio1a3Sel::B00),
            1 => Some(Gpio1a3Sel::B01),
            2 => Some(Gpio1a3Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1a3Sel::B00
    }
    #[doc = "isp0_flashtrigout"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1a3Sel::B01
    }
    #[doc = "isp1_flashtrigout"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1a3Sel::B10
    }
}
#[doc = "Field `GPIO1A3_SEL` writer - GPIO1A\\[3\\]
iomux select"]
pub type Gpio1a3SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1a3Sel>;
impl<'a, REG> Gpio1a3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a3Sel::B00)
    }
    #[doc = "isp0_flashtrigout"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a3Sel::B01)
    }
    #[doc = "isp1_flashtrigout"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a3Sel::B10)
    }
}
#[doc = "GPIO1A\\[4\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1a4Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: isp0_prelighttrig"]
    B01 = 1,
    #[doc = "2: isp1_prelighttrig"]
    B10 = 2,
}
impl From<Gpio1a4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1a4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1a4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1A4_SEL` reader - GPIO1A\\[4\\]
iomux select"]
pub type Gpio1a4SelR = crate::FieldReader<Gpio1a4Sel>;
impl Gpio1a4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1a4Sel> {
        match self.bits {
            0 => Some(Gpio1a4Sel::B00),
            1 => Some(Gpio1a4Sel::B01),
            2 => Some(Gpio1a4Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1a4Sel::B00
    }
    #[doc = "isp0_prelighttrig"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1a4Sel::B01
    }
    #[doc = "isp1_prelighttrig"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1a4Sel::B10
    }
}
#[doc = "Field `GPIO1A4_SEL` writer - GPIO1A\\[4\\]
iomux select"]
pub type Gpio1a4SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1a4Sel>;
impl<'a, REG> Gpio1a4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a4Sel::B00)
    }
    #[doc = "isp0_prelighttrig"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a4Sel::B01)
    }
    #[doc = "isp1_prelighttrig"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a4Sel::B10)
    }
}
#[doc = "GPIO1A\\[5\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1a5Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: ap_pwroff"]
    B01 = 1,
}
impl From<Gpio1a5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1a5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1a5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1A5_SEL` reader - GPIO1A\\[5\\]
iomux select"]
pub type Gpio1a5SelR = crate::FieldReader<Gpio1a5Sel>;
impl Gpio1a5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1a5Sel> {
        match self.bits {
            0 => Some(Gpio1a5Sel::B00),
            1 => Some(Gpio1a5Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1a5Sel::B00
    }
    #[doc = "ap_pwroff"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1a5Sel::B01
    }
}
#[doc = "Field `GPIO1A5_SEL` writer - GPIO1A\\[5\\]
iomux select"]
pub type Gpio1a5SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1a5Sel>;
impl<'a, REG> Gpio1a5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a5Sel::B00)
    }
    #[doc = "ap_pwroff"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a5Sel::B01)
    }
}
#[doc = "GPIO1A\\[6\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1a6Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: tsadc_int"]
    B01 = 1,
}
impl From<Gpio1a6Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1a6Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1a6Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1A6_SEL` reader - GPIO1A\\[6\\]
iomux select"]
pub type Gpio1a6SelR = crate::FieldReader<Gpio1a6Sel>;
impl Gpio1a6SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1a6Sel> {
        match self.bits {
            0 => Some(Gpio1a6Sel::B00),
            1 => Some(Gpio1a6Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1a6Sel::B00
    }
    #[doc = "tsadc_int"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1a6Sel::B01
    }
}
#[doc = "Field `GPIO1A6_SEL` writer - GPIO1A\\[6\\]
iomux select"]
pub type Gpio1a6SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1a6Sel>;
impl<'a, REG> Gpio1a6SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a6Sel::B00)
    }
    #[doc = "tsadc_int"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a6Sel::B01)
    }
}
#[doc = "GPIO1A\\[7\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1a7Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: uart4m0_sin"]
    B01 = 1,
    #[doc = "2: spi1ec_rxd"]
    B10 = 2,
}
impl From<Gpio1a7Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1a7Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1a7Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1A7_SEL` reader - GPIO1A\\[7\\]
iomux select"]
pub type Gpio1a7SelR = crate::FieldReader<Gpio1a7Sel>;
impl Gpio1a7SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1a7Sel> {
        match self.bits {
            0 => Some(Gpio1a7Sel::B00),
            1 => Some(Gpio1a7Sel::B01),
            2 => Some(Gpio1a7Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1a7Sel::B00
    }
    #[doc = "uart4m0_sin"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1a7Sel::B01
    }
    #[doc = "spi1ec_rxd"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1a7Sel::B10
    }
}
#[doc = "Field `GPIO1A7_SEL` writer - GPIO1A\\[7\\]
iomux select"]
pub type Gpio1a7SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1a7Sel>;
impl<'a, REG> Gpio1a7SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a7Sel::B00)
    }
    #[doc = "uart4m0_sin"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a7Sel::B01)
    }
    #[doc = "spi1ec_rxd"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1a7Sel::B10)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO1A\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1a0_sel(&self) -> Gpio1a0SelR {
        Gpio1a0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO1A\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1a1_sel(&self) -> Gpio1a1SelR {
        Gpio1a1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO1A\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1a2_sel(&self) -> Gpio1a2SelR {
        Gpio1a2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO1A\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1a3_sel(&self) -> Gpio1a3SelR {
        Gpio1a3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO1A\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1a4_sel(&self) -> Gpio1a4SelR {
        Gpio1a4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO1A\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1a5_sel(&self) -> Gpio1a5SelR {
        Gpio1a5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO1A\\[6\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1a6_sel(&self) -> Gpio1a6SelR {
        Gpio1a6SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO1A\\[7\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1a7_sel(&self) -> Gpio1a7SelR {
        Gpio1a7SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO1A\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a0_sel(&mut self) -> Gpio1a0SelW<Gpio1aIomuxSpec> {
        Gpio1a0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO1A\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a1_sel(&mut self) -> Gpio1a1SelW<Gpio1aIomuxSpec> {
        Gpio1a1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO1A\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a2_sel(&mut self) -> Gpio1a2SelW<Gpio1aIomuxSpec> {
        Gpio1a2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO1A\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a3_sel(&mut self) -> Gpio1a3SelW<Gpio1aIomuxSpec> {
        Gpio1a3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO1A\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a4_sel(&mut self) -> Gpio1a4SelW<Gpio1aIomuxSpec> {
        Gpio1a4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO1A\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a5_sel(&mut self) -> Gpio1a5SelW<Gpio1aIomuxSpec> {
        Gpio1a5SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO1A\\[6\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a6_sel(&mut self) -> Gpio1a6SelW<Gpio1aIomuxSpec> {
        Gpio1a6SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO1A\\[7\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a7_sel(&mut self) -> Gpio1a7SelW<Gpio1aIomuxSpec> {
        Gpio1a7SelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio1aIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO1A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1a_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1a_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio1aIomuxSpec;
impl crate::RegisterSpec for Gpio1aIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio1a_iomux::R`](R) reader structure"]
impl crate::Readable for Gpio1aIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio1a_iomux::W`](W) writer structure"]
impl crate::Writable for Gpio1aIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO1A_IOMUX to value 0"]
impl crate::Resettable for Gpio1aIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
