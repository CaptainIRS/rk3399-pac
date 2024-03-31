#[doc = "Register `GPIO3B_IOMUX` reader"]
pub type R = crate::R<Gpio3bIomuxSpec>;
#[doc = "Register `GPIO3B_IOMUX` writer"]
pub type W = crate::W<Gpio3bIomuxSpec>;
#[doc = "GPIO3B\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: mac_mdc"]
    B01 = 1,
    #[doc = "2: spi0norcodec_csn1"]
    B10 = 2,
}
impl From<Gpio3b0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3B0_SEL` reader - GPIO3B\\[0\\]
iomux select"]
pub type Gpio3b0SelR = crate::FieldReader<Gpio3b0Sel>;
impl Gpio3b0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3b0Sel> {
        match self.bits {
            0 => Some(Gpio3b0Sel::B00),
            1 => Some(Gpio3b0Sel::B01),
            2 => Some(Gpio3b0Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b0Sel::B00
    }
    #[doc = "mac_mdc"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b0Sel::B01
    }
    #[doc = "spi0norcodec_csn1"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b0Sel::B10
    }
}
#[doc = "Field `GPIO3B0_SEL` writer - GPIO3B\\[0\\]
iomux select"]
pub type Gpio3b0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio3b0Sel>;
impl<'a, REG> Gpio3b0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b0Sel::B00)
    }
    #[doc = "mac_mdc"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b0Sel::B01)
    }
    #[doc = "spi0norcodec_csn1"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b0Sel::B10)
    }
}
#[doc = "GPIO3B\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: mac_rxdv"]
    B01 = 1,
}
impl From<Gpio3b1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3B1_SEL` reader - GPIO3B\\[1\\]
iomux select"]
pub type Gpio3b1SelR = crate::FieldReader<Gpio3b1Sel>;
impl Gpio3b1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3b1Sel> {
        match self.bits {
            0 => Some(Gpio3b1Sel::B00),
            1 => Some(Gpio3b1Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b1Sel::B00
    }
    #[doc = "mac_rxdv"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b1Sel::B01
    }
}
#[doc = "Field `GPIO3B1_SEL` writer - GPIO3B\\[1\\]
iomux select"]
pub type Gpio3b1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio3b1Sel>;
impl<'a, REG> Gpio3b1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b1Sel::B00)
    }
    #[doc = "mac_rxdv"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b1Sel::B01)
    }
}
#[doc = "GPIO3B\\[2\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b2Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: mac_rxer"]
    B01 = 1,
    #[doc = "2: i2c5trackpad_sda"]
    B10 = 2,
}
impl From<Gpio3b2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3B2_SEL` reader - GPIO3B\\[2\\]
iomux select"]
pub type Gpio3b2SelR = crate::FieldReader<Gpio3b2Sel>;
impl Gpio3b2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3b2Sel> {
        match self.bits {
            0 => Some(Gpio3b2Sel::B00),
            1 => Some(Gpio3b2Sel::B01),
            2 => Some(Gpio3b2Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b2Sel::B00
    }
    #[doc = "mac_rxer"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b2Sel::B01
    }
    #[doc = "i2c5trackpad_sda"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b2Sel::B10
    }
}
#[doc = "Field `GPIO3B2_SEL` writer - GPIO3B\\[2\\]
iomux select"]
pub type Gpio3b2SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio3b2Sel>;
impl<'a, REG> Gpio3b2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b2Sel::B00)
    }
    #[doc = "mac_rxer"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b2Sel::B01)
    }
    #[doc = "i2c5trackpad_sda"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b2Sel::B10)
    }
}
#[doc = "GPIO3B\\[3\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b3Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: mac_clk"]
    B01 = 1,
    #[doc = "2: i2c5trackpad_scl"]
    B10 = 2,
}
impl From<Gpio3b3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3B3_SEL` reader - GPIO3B\\[3\\]
iomux select"]
pub type Gpio3b3SelR = crate::FieldReader<Gpio3b3Sel>;
impl Gpio3b3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3b3Sel> {
        match self.bits {
            0 => Some(Gpio3b3Sel::B00),
            1 => Some(Gpio3b3Sel::B01),
            2 => Some(Gpio3b3Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b3Sel::B00
    }
    #[doc = "mac_clk"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b3Sel::B01
    }
    #[doc = "i2c5trackpad_scl"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b3Sel::B10
    }
}
#[doc = "Field `GPIO3B3_SEL` writer - GPIO3B\\[3\\]
iomux select"]
pub type Gpio3b3SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio3b3Sel>;
impl<'a, REG> Gpio3b3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b3Sel::B00)
    }
    #[doc = "mac_clk"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b3Sel::B01)
    }
    #[doc = "i2c5trackpad_scl"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b3Sel::B10)
    }
}
#[doc = "GPIO3B\\[4\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b4Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: mac_txen"]
    B01 = 1,
    #[doc = "2: uart1bb_sin"]
    B10 = 2,
}
impl From<Gpio3b4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3B4_SEL` reader - GPIO3B\\[4\\]
iomux select"]
pub type Gpio3b4SelR = crate::FieldReader<Gpio3b4Sel>;
impl Gpio3b4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3b4Sel> {
        match self.bits {
            0 => Some(Gpio3b4Sel::B00),
            1 => Some(Gpio3b4Sel::B01),
            2 => Some(Gpio3b4Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b4Sel::B00
    }
    #[doc = "mac_txen"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b4Sel::B01
    }
    #[doc = "uart1bb_sin"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b4Sel::B10
    }
}
#[doc = "Field `GPIO3B4_SEL` writer - GPIO3B\\[4\\]
iomux select"]
pub type Gpio3b4SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio3b4Sel>;
impl<'a, REG> Gpio3b4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b4Sel::B00)
    }
    #[doc = "mac_txen"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b4Sel::B01)
    }
    #[doc = "uart1bb_sin"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b4Sel::B10)
    }
}
#[doc = "GPIO3B\\[5\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b5Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: mac_mdio"]
    B01 = 1,
    #[doc = "2: uart1bb_sout"]
    B10 = 2,
}
impl From<Gpio3b5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3B5_SEL` reader - GPIO3B\\[5\\]
iomux select"]
pub type Gpio3b5SelR = crate::FieldReader<Gpio3b5Sel>;
impl Gpio3b5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3b5Sel> {
        match self.bits {
            0 => Some(Gpio3b5Sel::B00),
            1 => Some(Gpio3b5Sel::B01),
            2 => Some(Gpio3b5Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b5Sel::B00
    }
    #[doc = "mac_mdio"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b5Sel::B01
    }
    #[doc = "uart1bb_sout"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b5Sel::B10
    }
}
#[doc = "Field `GPIO3B5_SEL` writer - GPIO3B\\[5\\]
iomux select"]
pub type Gpio3b5SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio3b5Sel>;
impl<'a, REG> Gpio3b5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b5Sel::B00)
    }
    #[doc = "mac_mdio"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b5Sel::B01)
    }
    #[doc = "uart1bb_sout"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b5Sel::B10)
    }
}
#[doc = "GPIO3B\\[6\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b6Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: mac_rxclk"]
    B01 = 1,
    #[doc = "2: uart3gps_sin"]
    B10 = 2,
}
impl From<Gpio3b6Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b6Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b6Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3B6_SEL` reader - GPIO3B\\[6\\]
iomux select"]
pub type Gpio3b6SelR = crate::FieldReader<Gpio3b6Sel>;
impl Gpio3b6SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3b6Sel> {
        match self.bits {
            0 => Some(Gpio3b6Sel::B00),
            1 => Some(Gpio3b6Sel::B01),
            2 => Some(Gpio3b6Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b6Sel::B00
    }
    #[doc = "mac_rxclk"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b6Sel::B01
    }
    #[doc = "uart3gps_sin"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b6Sel::B10
    }
}
#[doc = "Field `GPIO3B6_SEL` writer - GPIO3B\\[6\\]
iomux select"]
pub type Gpio3b6SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio3b6Sel>;
impl<'a, REG> Gpio3b6SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b6Sel::B00)
    }
    #[doc = "mac_rxclk"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b6Sel::B01)
    }
    #[doc = "uart3gps_sin"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b6Sel::B10)
    }
}
#[doc = "GPIO3B\\[7\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b7Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: mac_crs"]
    B01 = 1,
    #[doc = "2: uart3gps_sout"]
    B10 = 2,
    #[doc = "3: cif_clkoutb"]
    B11 = 3,
}
impl From<Gpio3b7Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b7Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b7Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3B7_SEL` reader - GPIO3B\\[7\\]
iomux select"]
pub type Gpio3b7SelR = crate::FieldReader<Gpio3b7Sel>;
impl Gpio3b7SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3b7Sel {
        match self.bits {
            0 => Gpio3b7Sel::B00,
            1 => Gpio3b7Sel::B01,
            2 => Gpio3b7Sel::B10,
            3 => Gpio3b7Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b7Sel::B00
    }
    #[doc = "mac_crs"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b7Sel::B01
    }
    #[doc = "uart3gps_sout"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b7Sel::B10
    }
    #[doc = "cif_clkoutb"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3b7Sel::B11
    }
}
#[doc = "Field `GPIO3B7_SEL` writer - GPIO3B\\[7\\]
iomux select"]
pub type Gpio3b7SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3b7Sel>;
impl<'a, REG> Gpio3b7SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b7Sel::B00)
    }
    #[doc = "mac_crs"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b7Sel::B01)
    }
    #[doc = "uart3gps_sout"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b7Sel::B10)
    }
    #[doc = "cif_clkoutb"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b7Sel::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO3B\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3b0_sel(&self) -> Gpio3b0SelR {
        Gpio3b0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO3B\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3b1_sel(&self) -> Gpio3b1SelR {
        Gpio3b1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO3B\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3b2_sel(&self) -> Gpio3b2SelR {
        Gpio3b2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO3B\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3b3_sel(&self) -> Gpio3b3SelR {
        Gpio3b3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO3B\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3b4_sel(&self) -> Gpio3b4SelR {
        Gpio3b4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO3B\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3b5_sel(&self) -> Gpio3b5SelR {
        Gpio3b5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO3B\\[6\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3b6_sel(&self) -> Gpio3b6SelR {
        Gpio3b6SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO3B\\[7\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3b7_sel(&self) -> Gpio3b7SelR {
        Gpio3b7SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO3B\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b0_sel(&mut self) -> Gpio3b0SelW<Gpio3bIomuxSpec> {
        Gpio3b0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO3B\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b1_sel(&mut self) -> Gpio3b1SelW<Gpio3bIomuxSpec> {
        Gpio3b1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO3B\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b2_sel(&mut self) -> Gpio3b2SelW<Gpio3bIomuxSpec> {
        Gpio3b2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO3B\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b3_sel(&mut self) -> Gpio3b3SelW<Gpio3bIomuxSpec> {
        Gpio3b3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO3B\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b4_sel(&mut self) -> Gpio3b4SelW<Gpio3bIomuxSpec> {
        Gpio3b4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO3B\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b5_sel(&mut self) -> Gpio3b5SelW<Gpio3bIomuxSpec> {
        Gpio3b5SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO3B\\[6\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b6_sel(&mut self) -> Gpio3b6SelW<Gpio3bIomuxSpec> {
        Gpio3b6SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO3B\\[7\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b7_sel(&mut self) -> Gpio3b7SelW<Gpio3bIomuxSpec> {
        Gpio3b7SelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio3bIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3b_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3b_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio3bIomuxSpec;
impl crate::RegisterSpec for Gpio3bIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio3b_iomux::R`](R) reader structure"]
impl crate::Readable for Gpio3bIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio3b_iomux::W`](W) writer structure"]
impl crate::Writable for Gpio3bIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO3B_IOMUX to value 0"]
impl crate::Resettable for Gpio3bIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
