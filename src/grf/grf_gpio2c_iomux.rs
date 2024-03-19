#[doc = "Register `GRF_GPIO2C_IOMUX` reader"]
pub type R = crate::R<GrfGpio2cIomuxSpec>;
#[doc = "Register `GRF_GPIO2C_IOMUX` writer"]
pub type W = crate::W<GrfGpio2cIomuxSpec>;
#[doc = "GPIO2C\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: uart0bt_sin"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2c0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2C0_SEL` reader - GPIO2C\\[0\\]
iomux select"]
pub type Gpio2c0SelR = crate::FieldReader<Gpio2c0Sel>;
impl Gpio2c0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c0Sel {
        match self.bits {
            0 => Gpio2c0Sel::B00,
            1 => Gpio2c0Sel::B01,
            2 => Gpio2c0Sel::B10,
            3 => Gpio2c0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c0Sel::B00
    }
    #[doc = "uart0bt_sin"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c0Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c0Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c0Sel::B11
    }
}
#[doc = "Field `GPIO2C0_SEL` writer - GPIO2C\\[0\\]
iomux select"]
pub type Gpio2c0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c0Sel>;
impl<'a, REG> Gpio2c0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c0Sel::B00)
    }
    #[doc = "uart0bt_sin"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c0Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c0Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c0Sel::B11)
    }
}
#[doc = "GPIO2C\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: uart0bt_sout"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2c1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2C1_SEL` reader - GPIO2C\\[1\\]
iomux select"]
pub type Gpio2c1SelR = crate::FieldReader<Gpio2c1Sel>;
impl Gpio2c1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c1Sel {
        match self.bits {
            0 => Gpio2c1Sel::B00,
            1 => Gpio2c1Sel::B01,
            2 => Gpio2c1Sel::B10,
            3 => Gpio2c1Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c1Sel::B00
    }
    #[doc = "uart0bt_sout"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c1Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c1Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c1Sel::B11
    }
}
#[doc = "Field `GPIO2C1_SEL` writer - GPIO2C\\[1\\]
iomux select"]
pub type Gpio2c1SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c1Sel>;
impl<'a, REG> Gpio2c1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c1Sel::B00)
    }
    #[doc = "uart0bt_sout"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c1Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c1Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c1Sel::B11)
    }
}
#[doc = "GPIO2C\\[2\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c2Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: uart0bt_ctsn"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2c2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2C2_SEL` reader - GPIO2C\\[2\\]
iomux select"]
pub type Gpio2c2SelR = crate::FieldReader<Gpio2c2Sel>;
impl Gpio2c2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c2Sel {
        match self.bits {
            0 => Gpio2c2Sel::B00,
            1 => Gpio2c2Sel::B01,
            2 => Gpio2c2Sel::B10,
            3 => Gpio2c2Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c2Sel::B00
    }
    #[doc = "uart0bt_ctsn"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c2Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c2Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c2Sel::B11
    }
}
#[doc = "Field `GPIO2C2_SEL` writer - GPIO2C\\[2\\]
iomux select"]
pub type Gpio2c2SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c2Sel>;
impl<'a, REG> Gpio2c2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c2Sel::B00)
    }
    #[doc = "uart0bt_ctsn"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c2Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c2Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c2Sel::B11)
    }
}
#[doc = "GPIO2C\\[3\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c3Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: uart0bt_rtsn"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2c3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2C3_SEL` reader - GPIO2C\\[3\\]
iomux select"]
pub type Gpio2c3SelR = crate::FieldReader<Gpio2c3Sel>;
impl Gpio2c3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c3Sel {
        match self.bits {
            0 => Gpio2c3Sel::B00,
            1 => Gpio2c3Sel::B01,
            2 => Gpio2c3Sel::B10,
            3 => Gpio2c3Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c3Sel::B00
    }
    #[doc = "uart0bt_rtsn"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c3Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c3Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c3Sel::B11
    }
}
#[doc = "Field `GPIO2C3_SEL` writer - GPIO2C\\[3\\]
iomux select"]
pub type Gpio2c3SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c3Sel>;
impl<'a, REG> Gpio2c3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c3Sel::B00)
    }
    #[doc = "uart0bt_rtsn"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c3Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c3Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c3Sel::B11)
    }
}
#[doc = "GPIO2C\\[4\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c4Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdio_data0"]
    B01 = 1,
    #[doc = "2: spi5expplus_rxd"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2c4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2C4_SEL` reader - GPIO2C\\[4\\]
iomux select"]
pub type Gpio2c4SelR = crate::FieldReader<Gpio2c4Sel>;
impl Gpio2c4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c4Sel {
        match self.bits {
            0 => Gpio2c4Sel::B00,
            1 => Gpio2c4Sel::B01,
            2 => Gpio2c4Sel::B10,
            3 => Gpio2c4Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c4Sel::B00
    }
    #[doc = "sdio_data0"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c4Sel::B01
    }
    #[doc = "spi5expplus_rxd"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c4Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c4Sel::B11
    }
}
#[doc = "Field `GPIO2C4_SEL` writer - GPIO2C\\[4\\]
iomux select"]
pub type Gpio2c4SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c4Sel>;
impl<'a, REG> Gpio2c4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c4Sel::B00)
    }
    #[doc = "sdio_data0"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c4Sel::B01)
    }
    #[doc = "spi5expplus_rxd"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c4Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c4Sel::B11)
    }
}
#[doc = "GPIO2C\\[5\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c5Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdio_data1"]
    B01 = 1,
    #[doc = "2: spi5expplus_txd"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2c5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2C5_SEL` reader - GPIO2C\\[5\\]
iomux select"]
pub type Gpio2c5SelR = crate::FieldReader<Gpio2c5Sel>;
impl Gpio2c5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c5Sel {
        match self.bits {
            0 => Gpio2c5Sel::B00,
            1 => Gpio2c5Sel::B01,
            2 => Gpio2c5Sel::B10,
            3 => Gpio2c5Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c5Sel::B00
    }
    #[doc = "sdio_data1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c5Sel::B01
    }
    #[doc = "spi5expplus_txd"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c5Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c5Sel::B11
    }
}
#[doc = "Field `GPIO2C5_SEL` writer - GPIO2C\\[5\\]
iomux select"]
pub type Gpio2c5SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c5Sel>;
impl<'a, REG> Gpio2c5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c5Sel::B00)
    }
    #[doc = "sdio_data1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c5Sel::B01)
    }
    #[doc = "spi5expplus_txd"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c5Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c5Sel::B11)
    }
}
#[doc = "GPIO2C\\[6\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c6Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdio_data2"]
    B01 = 1,
    #[doc = "2: spi5expplus_clk"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2c6Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c6Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c6Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2C6_SEL` reader - GPIO2C\\[6\\]
iomux select"]
pub type Gpio2c6SelR = crate::FieldReader<Gpio2c6Sel>;
impl Gpio2c6SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c6Sel {
        match self.bits {
            0 => Gpio2c6Sel::B00,
            1 => Gpio2c6Sel::B01,
            2 => Gpio2c6Sel::B10,
            3 => Gpio2c6Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c6Sel::B00
    }
    #[doc = "sdio_data2"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c6Sel::B01
    }
    #[doc = "spi5expplus_clk"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c6Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c6Sel::B11
    }
}
#[doc = "Field `GPIO2C6_SEL` writer - GPIO2C\\[6\\]
iomux select"]
pub type Gpio2c6SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c6Sel>;
impl<'a, REG> Gpio2c6SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c6Sel::B00)
    }
    #[doc = "sdio_data2"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c6Sel::B01)
    }
    #[doc = "spi5expplus_clk"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c6Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c6Sel::B11)
    }
}
#[doc = "GPIO2C\\[7\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c7Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdio_data3"]
    B01 = 1,
    #[doc = "2: spi5expplus_csn0"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2c7Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c7Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c7Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2C7_SEL` reader - GPIO2C\\[7\\]
iomux select"]
pub type Gpio2c7SelR = crate::FieldReader<Gpio2c7Sel>;
impl Gpio2c7SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c7Sel {
        match self.bits {
            0 => Gpio2c7Sel::B00,
            1 => Gpio2c7Sel::B01,
            2 => Gpio2c7Sel::B10,
            3 => Gpio2c7Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c7Sel::B00
    }
    #[doc = "sdio_data3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c7Sel::B01
    }
    #[doc = "spi5expplus_csn0"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c7Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c7Sel::B11
    }
}
#[doc = "Field `GPIO2C7_SEL` writer - GPIO2C\\[7\\]
iomux select"]
pub type Gpio2c7SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c7Sel>;
impl<'a, REG> Gpio2c7SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c7Sel::B00)
    }
    #[doc = "sdio_data3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c7Sel::B01)
    }
    #[doc = "spi5expplus_csn0"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c7Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c7Sel::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO2C\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2c0_sel(&self) -> Gpio2c0SelR {
        Gpio2c0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO2C\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2c1_sel(&self) -> Gpio2c1SelR {
        Gpio2c1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO2C\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2c2_sel(&self) -> Gpio2c2SelR {
        Gpio2c2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO2C\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2c3_sel(&self) -> Gpio2c3SelR {
        Gpio2c3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO2C\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2c4_sel(&self) -> Gpio2c4SelR {
        Gpio2c4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO2C\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2c5_sel(&self) -> Gpio2c5SelR {
        Gpio2c5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO2C\\[6\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2c6_sel(&self) -> Gpio2c6SelR {
        Gpio2c6SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO2C\\[7\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2c7_sel(&self) -> Gpio2c7SelR {
        Gpio2c7SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO2C\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c0_sel(&mut self) -> Gpio2c0SelW<GrfGpio2cIomuxSpec> {
        Gpio2c0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO2C\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c1_sel(&mut self) -> Gpio2c1SelW<GrfGpio2cIomuxSpec> {
        Gpio2c1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO2C\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c2_sel(&mut self) -> Gpio2c2SelW<GrfGpio2cIomuxSpec> {
        Gpio2c2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO2C\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c3_sel(&mut self) -> Gpio2c3SelW<GrfGpio2cIomuxSpec> {
        Gpio2c3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO2C\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c4_sel(&mut self) -> Gpio2c4SelW<GrfGpio2cIomuxSpec> {
        Gpio2c4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO2C\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c5_sel(&mut self) -> Gpio2c5SelW<GrfGpio2cIomuxSpec> {
        Gpio2c5SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO2C\\[6\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c6_sel(&mut self) -> Gpio2c6SelW<GrfGpio2cIomuxSpec> {
        Gpio2c6SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO2C\\[7\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c7_sel(&mut self) -> Gpio2c7SelW<GrfGpio2cIomuxSpec> {
        Gpio2c7SelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio2cIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2C iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2c_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2c_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio2cIomuxSpec;
impl crate::RegisterSpec for GrfGpio2cIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio2c_iomux::R`](R) reader structure"]
impl crate::Readable for GrfGpio2cIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio2c_iomux::W`](W) writer structure"]
impl crate::Writable for GrfGpio2cIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO2C_IOMUX to value 0"]
impl crate::Resettable for GrfGpio2cIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
