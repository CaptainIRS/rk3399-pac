#[doc = "Register `GRF_GPIO4B_IOMUX` reader"]
pub type R = crate::R<GrfGpio4bIomuxSpec>;
#[doc = "Register `GRF_GPIO4B_IOMUX` writer"]
pub type W = crate::W<GrfGpio4bIomuxSpec>;
#[doc = "GPIO4B\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4b0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdmmc_data0"]
    B01 = 1,
    #[doc = "2: uart2dbga_sin"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio4b0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4b0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4b0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4B0_SEL` reader - GPIO4B\\[0\\]
iomux select"]
pub type Gpio4b0SelR = crate::FieldReader<Gpio4b0Sel>;
impl Gpio4b0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4b0Sel {
        match self.bits {
            0 => Gpio4b0Sel::B00,
            1 => Gpio4b0Sel::B01,
            2 => Gpio4b0Sel::B10,
            3 => Gpio4b0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4b0Sel::B00
    }
    #[doc = "sdmmc_data0"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4b0Sel::B01
    }
    #[doc = "uart2dbga_sin"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4b0Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4b0Sel::B11
    }
}
#[doc = "Field `GPIO4B0_SEL` writer - GPIO4B\\[0\\]
iomux select"]
pub type Gpio4b0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4b0Sel>;
impl<'a, REG> Gpio4b0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b0Sel::B00)
    }
    #[doc = "sdmmc_data0"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b0Sel::B01)
    }
    #[doc = "uart2dbga_sin"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b0Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b0Sel::B11)
    }
}
#[doc = "GPIO4B\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4b1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdmmc_data1"]
    B01 = 1,
    #[doc = "2: uart2dbga_sout"]
    B10 = 2,
    #[doc = "3: hdcpjtag_trstn"]
    B11 = 3,
}
impl From<Gpio4b1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4b1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4b1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4B1_SEL` reader - GPIO4B\\[1\\]
iomux select"]
pub type Gpio4b1SelR = crate::FieldReader<Gpio4b1Sel>;
impl Gpio4b1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4b1Sel {
        match self.bits {
            0 => Gpio4b1Sel::B00,
            1 => Gpio4b1Sel::B01,
            2 => Gpio4b1Sel::B10,
            3 => Gpio4b1Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4b1Sel::B00
    }
    #[doc = "sdmmc_data1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4b1Sel::B01
    }
    #[doc = "uart2dbga_sout"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4b1Sel::B10
    }
    #[doc = "hdcpjtag_trstn"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4b1Sel::B11
    }
}
#[doc = "Field `GPIO4B1_SEL` writer - GPIO4B\\[1\\]
iomux select"]
pub type Gpio4b1SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4b1Sel>;
impl<'a, REG> Gpio4b1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b1Sel::B00)
    }
    #[doc = "sdmmc_data1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b1Sel::B01)
    }
    #[doc = "uart2dbga_sout"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b1Sel::B10)
    }
    #[doc = "hdcpjtag_trstn"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b1Sel::B11)
    }
}
#[doc = "GPIO4B\\[2\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4b2Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdmmc_data2"]
    B01 = 1,
    #[doc = "2: cxcsjtag_tck"]
    B10 = 2,
    #[doc = "3: hdcpjtag_tdi"]
    B11 = 3,
}
impl From<Gpio4b2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4b2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4b2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4B2_SEL` reader - GPIO4B\\[2\\]
iomux select"]
pub type Gpio4b2SelR = crate::FieldReader<Gpio4b2Sel>;
impl Gpio4b2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4b2Sel {
        match self.bits {
            0 => Gpio4b2Sel::B00,
            1 => Gpio4b2Sel::B01,
            2 => Gpio4b2Sel::B10,
            3 => Gpio4b2Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4b2Sel::B00
    }
    #[doc = "sdmmc_data2"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4b2Sel::B01
    }
    #[doc = "cxcsjtag_tck"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4b2Sel::B10
    }
    #[doc = "hdcpjtag_tdi"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4b2Sel::B11
    }
}
#[doc = "Field `GPIO4B2_SEL` writer - GPIO4B\\[2\\]
iomux select"]
pub type Gpio4b2SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4b2Sel>;
impl<'a, REG> Gpio4b2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b2Sel::B00)
    }
    #[doc = "sdmmc_data2"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b2Sel::B01)
    }
    #[doc = "cxcsjtag_tck"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b2Sel::B10)
    }
    #[doc = "hdcpjtag_tdi"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b2Sel::B11)
    }
}
#[doc = "GPIO4B\\[3\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4b3Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdmmc_data3"]
    B01 = 1,
    #[doc = "2: cxcsjtag_tms"]
    B10 = 2,
    #[doc = "3: hdcpjtag_tdo"]
    B11 = 3,
}
impl From<Gpio4b3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4b3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4b3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4B3_SEL` reader - GPIO4B\\[3\\]
iomux select"]
pub type Gpio4b3SelR = crate::FieldReader<Gpio4b3Sel>;
impl Gpio4b3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4b3Sel {
        match self.bits {
            0 => Gpio4b3Sel::B00,
            1 => Gpio4b3Sel::B01,
            2 => Gpio4b3Sel::B10,
            3 => Gpio4b3Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4b3Sel::B00
    }
    #[doc = "sdmmc_data3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4b3Sel::B01
    }
    #[doc = "cxcsjtag_tms"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4b3Sel::B10
    }
    #[doc = "hdcpjtag_tdo"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4b3Sel::B11
    }
}
#[doc = "Field `GPIO4B3_SEL` writer - GPIO4B\\[3\\]
iomux select"]
pub type Gpio4b3SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4b3Sel>;
impl<'a, REG> Gpio4b3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b3Sel::B00)
    }
    #[doc = "sdmmc_data3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b3Sel::B01)
    }
    #[doc = "cxcsjtag_tms"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b3Sel::B10)
    }
    #[doc = "hdcpjtag_tdo"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b3Sel::B11)
    }
}
#[doc = "GPIO4B\\[4\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4b4Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdmmc_clkout"]
    B01 = 1,
    #[doc = "2: mcujtag_tck"]
    B10 = 2,
    #[doc = "3: hdcpjtag_tck"]
    B11 = 3,
}
impl From<Gpio4b4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4b4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4b4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4B4_SEL` reader - GPIO4B\\[4\\]
iomux select"]
pub type Gpio4b4SelR = crate::FieldReader<Gpio4b4Sel>;
impl Gpio4b4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4b4Sel {
        match self.bits {
            0 => Gpio4b4Sel::B00,
            1 => Gpio4b4Sel::B01,
            2 => Gpio4b4Sel::B10,
            3 => Gpio4b4Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4b4Sel::B00
    }
    #[doc = "sdmmc_clkout"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4b4Sel::B01
    }
    #[doc = "mcujtag_tck"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4b4Sel::B10
    }
    #[doc = "hdcpjtag_tck"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4b4Sel::B11
    }
}
#[doc = "Field `GPIO4B4_SEL` writer - GPIO4B\\[4\\]
iomux select"]
pub type Gpio4b4SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4b4Sel>;
impl<'a, REG> Gpio4b4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b4Sel::B00)
    }
    #[doc = "sdmmc_clkout"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b4Sel::B01)
    }
    #[doc = "mcujtag_tck"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b4Sel::B10)
    }
    #[doc = "hdcpjtag_tck"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b4Sel::B11)
    }
}
#[doc = "GPIO4B\\[5\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4b5Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdmmc_cmd"]
    B01 = 1,
    #[doc = "2: mcujtag_tms"]
    B10 = 2,
    #[doc = "3: hdcpjtag_tms"]
    B11 = 3,
}
impl From<Gpio4b5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4b5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4b5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4B5_SEL` reader - GPIO4B\\[5\\]
iomux select"]
pub type Gpio4b5SelR = crate::FieldReader<Gpio4b5Sel>;
impl Gpio4b5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4b5Sel {
        match self.bits {
            0 => Gpio4b5Sel::B00,
            1 => Gpio4b5Sel::B01,
            2 => Gpio4b5Sel::B10,
            3 => Gpio4b5Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4b5Sel::B00
    }
    #[doc = "sdmmc_cmd"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4b5Sel::B01
    }
    #[doc = "mcujtag_tms"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4b5Sel::B10
    }
    #[doc = "hdcpjtag_tms"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4b5Sel::B11
    }
}
#[doc = "Field `GPIO4B5_SEL` writer - GPIO4B\\[5\\]
iomux select"]
pub type Gpio4b5SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4b5Sel>;
impl<'a, REG> Gpio4b5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b5Sel::B00)
    }
    #[doc = "sdmmc_cmd"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b5Sel::B01)
    }
    #[doc = "mcujtag_tms"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b5Sel::B10)
    }
    #[doc = "hdcpjtag_tms"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b5Sel::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO4B\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4b0_sel(&self) -> Gpio4b0SelR {
        Gpio4b0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO4B\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4b1_sel(&self) -> Gpio4b1SelR {
        Gpio4b1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO4B\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4b2_sel(&self) -> Gpio4b2SelR {
        Gpio4b2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO4B\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4b3_sel(&self) -> Gpio4b3SelR {
        Gpio4b3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO4B\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4b4_sel(&self) -> Gpio4b4SelR {
        Gpio4b4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO4B\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4b5_sel(&self) -> Gpio4b5SelR {
        Gpio4b5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO4B\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b0_sel(&mut self) -> Gpio4b0SelW<GrfGpio4bIomuxSpec> {
        Gpio4b0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO4B\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b1_sel(&mut self) -> Gpio4b1SelW<GrfGpio4bIomuxSpec> {
        Gpio4b1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO4B\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b2_sel(&mut self) -> Gpio4b2SelW<GrfGpio4bIomuxSpec> {
        Gpio4b2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO4B\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b3_sel(&mut self) -> Gpio4b3SelW<GrfGpio4bIomuxSpec> {
        Gpio4b3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO4B\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b4_sel(&mut self) -> Gpio4b4SelW<GrfGpio4bIomuxSpec> {
        Gpio4b4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO4B\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b5_sel(&mut self) -> Gpio4b5SelW<GrfGpio4bIomuxSpec> {
        Gpio4b5SelW::new(self, 10)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio4bIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4b_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4b_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio4bIomuxSpec;
impl crate::RegisterSpec for GrfGpio4bIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio4b_iomux::R`](R) reader structure"]
impl crate::Readable for GrfGpio4bIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio4b_iomux::W`](W) writer structure"]
impl crate::Writable for GrfGpio4bIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO4B_IOMUX to value 0"]
impl crate::Resettable for GrfGpio4bIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
