#[doc = "Register `GRF_GPIO2B_IOMUX` reader"]
pub type R = crate::R<GrfGpio2bIomuxSpec>;
#[doc = "Register `GRF_GPIO2B_IOMUX` writer"]
pub type W = crate::W<GrfGpio2bIomuxSpec>;
#[doc = "GPIO2B\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: vop_dclk"]
    B01 = 1,
    #[doc = "2: i2c7nfc_scl"]
    B10 = 2,
    #[doc = "3: cif_vsync"]
    B11 = 3,
}
impl From<Gpio2b0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2B0_SEL` reader - GPIO2B\\[0\\]
iomux select"]
pub type Gpio2b0SelR = crate::FieldReader<Gpio2b0Sel>;
impl Gpio2b0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b0Sel {
        match self.bits {
            0 => Gpio2b0Sel::B00,
            1 => Gpio2b0Sel::B01,
            2 => Gpio2b0Sel::B10,
            3 => Gpio2b0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b0Sel::B00
    }
    #[doc = "vop_dclk"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b0Sel::B01
    }
    #[doc = "i2c7nfc_scl"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b0Sel::B10
    }
    #[doc = "cif_vsync"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b0Sel::B11
    }
}
#[doc = "Field `GPIO2B0_SEL` writer - GPIO2B\\[0\\]
iomux select"]
pub type Gpio2b0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b0Sel>;
impl<'a, REG> Gpio2b0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b0Sel::B00)
    }
    #[doc = "vop_dclk"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b0Sel::B01)
    }
    #[doc = "i2c7nfc_scl"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b0Sel::B10)
    }
    #[doc = "cif_vsync"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b0Sel::B11)
    }
}
#[doc = "GPIO2B\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: spi2tpm_rxd"]
    B01 = 1,
    #[doc = "2: i2c6tpm_sda"]
    B10 = 2,
    #[doc = "3: cif_href"]
    B11 = 3,
}
impl From<Gpio2b1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2B1_SEL` reader - GPIO2B\\[1\\]
iomux select"]
pub type Gpio2b1SelR = crate::FieldReader<Gpio2b1Sel>;
impl Gpio2b1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b1Sel {
        match self.bits {
            0 => Gpio2b1Sel::B00,
            1 => Gpio2b1Sel::B01,
            2 => Gpio2b1Sel::B10,
            3 => Gpio2b1Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b1Sel::B00
    }
    #[doc = "spi2tpm_rxd"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b1Sel::B01
    }
    #[doc = "i2c6tpm_sda"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b1Sel::B10
    }
    #[doc = "cif_href"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b1Sel::B11
    }
}
#[doc = "Field `GPIO2B1_SEL` writer - GPIO2B\\[1\\]
iomux select"]
pub type Gpio2b1SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b1Sel>;
impl<'a, REG> Gpio2b1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b1Sel::B00)
    }
    #[doc = "spi2tpm_rxd"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b1Sel::B01)
    }
    #[doc = "i2c6tpm_sda"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b1Sel::B10)
    }
    #[doc = "cif_href"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b1Sel::B11)
    }
}
#[doc = "GPIO2B\\[2\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b2Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: spi2tpm_txd"]
    B01 = 1,
    #[doc = "2: i2c6tpm_scl"]
    B10 = 2,
    #[doc = "3: cif_clkin"]
    B11 = 3,
}
impl From<Gpio2b2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2B2_SEL` reader - GPIO2B\\[2\\]
iomux select"]
pub type Gpio2b2SelR = crate::FieldReader<Gpio2b2Sel>;
impl Gpio2b2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b2Sel {
        match self.bits {
            0 => Gpio2b2Sel::B00,
            1 => Gpio2b2Sel::B01,
            2 => Gpio2b2Sel::B10,
            3 => Gpio2b2Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b2Sel::B00
    }
    #[doc = "spi2tpm_txd"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b2Sel::B01
    }
    #[doc = "i2c6tpm_scl"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b2Sel::B10
    }
    #[doc = "cif_clkin"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b2Sel::B11
    }
}
#[doc = "Field `GPIO2B2_SEL` writer - GPIO2B\\[2\\]
iomux select"]
pub type Gpio2b2SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b2Sel>;
impl<'a, REG> Gpio2b2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b2Sel::B00)
    }
    #[doc = "spi2tpm_txd"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b2Sel::B01)
    }
    #[doc = "i2c6tpm_scl"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b2Sel::B10)
    }
    #[doc = "cif_clkin"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b2Sel::B11)
    }
}
#[doc = "GPIO2B\\[3\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b3Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: spi2tpm_clk"]
    B01 = 1,
    #[doc = "2: vop_den"]
    B10 = 2,
    #[doc = "3: cif_clkouta"]
    B11 = 3,
}
impl From<Gpio2b3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2B3_SEL` reader - GPIO2B\\[3\\]
iomux select"]
pub type Gpio2b3SelR = crate::FieldReader<Gpio2b3Sel>;
impl Gpio2b3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b3Sel {
        match self.bits {
            0 => Gpio2b3Sel::B00,
            1 => Gpio2b3Sel::B01,
            2 => Gpio2b3Sel::B10,
            3 => Gpio2b3Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b3Sel::B00
    }
    #[doc = "spi2tpm_clk"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b3Sel::B01
    }
    #[doc = "vop_den"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b3Sel::B10
    }
    #[doc = "cif_clkouta"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b3Sel::B11
    }
}
#[doc = "Field `GPIO2B3_SEL` writer - GPIO2B\\[3\\]
iomux select"]
pub type Gpio2b3SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b3Sel>;
impl<'a, REG> Gpio2b3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b3Sel::B00)
    }
    #[doc = "spi2tpm_clk"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b3Sel::B01)
    }
    #[doc = "vop_den"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b3Sel::B10)
    }
    #[doc = "cif_clkouta"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b3Sel::B11)
    }
}
#[doc = "GPIO2B\\[4\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b4Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: spi2tpm_csn0"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2b4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2B4_SEL` reader - GPIO2B\\[4\\]
iomux select"]
pub type Gpio2b4SelR = crate::FieldReader<Gpio2b4Sel>;
impl Gpio2b4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b4Sel {
        match self.bits {
            0 => Gpio2b4Sel::B00,
            1 => Gpio2b4Sel::B01,
            2 => Gpio2b4Sel::B10,
            3 => Gpio2b4Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b4Sel::B00
    }
    #[doc = "spi2tpm_csn0"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b4Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b4Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b4Sel::B11
    }
}
#[doc = "Field `GPIO2B4_SEL` writer - GPIO2B\\[4\\]
iomux select"]
pub type Gpio2b4SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b4Sel>;
impl<'a, REG> Gpio2b4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b4Sel::B00)
    }
    #[doc = "spi2tpm_csn0"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b4Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b4Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b4Sel::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO2B\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2b0_sel(&self) -> Gpio2b0SelR {
        Gpio2b0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO2B\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2b1_sel(&self) -> Gpio2b1SelR {
        Gpio2b1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO2B\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2b2_sel(&self) -> Gpio2b2SelR {
        Gpio2b2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO2B\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2b3_sel(&self) -> Gpio2b3SelR {
        Gpio2b3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO2B\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2b4_sel(&self) -> Gpio2b4SelR {
        Gpio2b4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO2B\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b0_sel(&mut self) -> Gpio2b0SelW<GrfGpio2bIomuxSpec> {
        Gpio2b0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO2B\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b1_sel(&mut self) -> Gpio2b1SelW<GrfGpio2bIomuxSpec> {
        Gpio2b1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO2B\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b2_sel(&mut self) -> Gpio2b2SelW<GrfGpio2bIomuxSpec> {
        Gpio2b2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO2B\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b3_sel(&mut self) -> Gpio2b3SelW<GrfGpio2bIomuxSpec> {
        Gpio2b3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO2B\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b4_sel(&mut self) -> Gpio2b4SelW<GrfGpio2bIomuxSpec> {
        Gpio2b4SelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio2bIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2B iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2b_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2b_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio2bIomuxSpec;
impl crate::RegisterSpec for GrfGpio2bIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio2b_iomux::R`](R) reader structure"]
impl crate::Readable for GrfGpio2bIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio2b_iomux::W`](W) writer structure"]
impl crate::Writable for GrfGpio2bIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO2B_IOMUX to value 0"]
impl crate::Resettable for GrfGpio2bIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
