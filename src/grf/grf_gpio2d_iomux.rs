#[doc = "Register `GRF_GPIO2D_IOMUX` reader"]
pub type R = crate::R<GrfGpio2dIomuxSpec>;
#[doc = "Register `GRF_GPIO2D_IOMUX` writer"]
pub type W = crate::W<GrfGpio2dIomuxSpec>;
#[doc = "GPIO2D\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdio_cmd"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2d0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2D0_SEL` reader - GPIO2D\\[0\\]
iomux select"]
pub type Gpio2d0SelR = crate::FieldReader<Gpio2d0Sel>;
impl Gpio2d0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d0Sel {
        match self.bits {
            0 => Gpio2d0Sel::B00,
            1 => Gpio2d0Sel::B01,
            2 => Gpio2d0Sel::B10,
            3 => Gpio2d0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d0Sel::B00
    }
    #[doc = "sdio_cmd"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d0Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d0Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d0Sel::B11
    }
}
#[doc = "Field `GPIO2D0_SEL` writer - GPIO2D\\[0\\]
iomux select"]
pub type Gpio2d0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d0Sel>;
impl<'a, REG> Gpio2d0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d0Sel::B00)
    }
    #[doc = "sdio_cmd"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d0Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d0Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d0Sel::B11)
    }
}
#[doc = "GPIO2D\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdio_clkout"]
    B01 = 1,
    #[doc = "2: test_clkout1"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2d1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2D1_SEL` reader - GPIO2D\\[1\\]
iomux select"]
pub type Gpio2d1SelR = crate::FieldReader<Gpio2d1Sel>;
impl Gpio2d1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d1Sel {
        match self.bits {
            0 => Gpio2d1Sel::B00,
            1 => Gpio2d1Sel::B01,
            2 => Gpio2d1Sel::B10,
            3 => Gpio2d1Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d1Sel::B00
    }
    #[doc = "sdio_clkout"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d1Sel::B01
    }
    #[doc = "test_clkout1"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d1Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d1Sel::B11
    }
}
#[doc = "Field `GPIO2D1_SEL` writer - GPIO2D\\[1\\]
iomux select"]
pub type Gpio2d1SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d1Sel>;
impl<'a, REG> Gpio2d1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d1Sel::B00)
    }
    #[doc = "sdio_clkout"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d1Sel::B01)
    }
    #[doc = "test_clkout1"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d1Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d1Sel::B11)
    }
}
#[doc = "GPIO2D\\[2\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d2Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdio_detectn"]
    B01 = 1,
    #[doc = "2: pcie_clkreqn"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2d2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2D2_SEL` reader - GPIO2D\\[2\\]
iomux select"]
pub type Gpio2d2SelR = crate::FieldReader<Gpio2d2Sel>;
impl Gpio2d2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d2Sel {
        match self.bits {
            0 => Gpio2d2Sel::B00,
            1 => Gpio2d2Sel::B01,
            2 => Gpio2d2Sel::B10,
            3 => Gpio2d2Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d2Sel::B00
    }
    #[doc = "sdio_detectn"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d2Sel::B01
    }
    #[doc = "pcie_clkreqn"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d2Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d2Sel::B11
    }
}
#[doc = "Field `GPIO2D2_SEL` writer - GPIO2D\\[2\\]
iomux select"]
pub type Gpio2d2SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d2Sel>;
impl<'a, REG> Gpio2d2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d2Sel::B00)
    }
    #[doc = "sdio_detectn"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d2Sel::B01)
    }
    #[doc = "pcie_clkreqn"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d2Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d2Sel::B11)
    }
}
#[doc = "GPIO2D\\[3\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d3Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdio_pwren"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2d3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2D3_SEL` reader - GPIO2D\\[3\\]
iomux select"]
pub type Gpio2d3SelR = crate::FieldReader<Gpio2d3Sel>;
impl Gpio2d3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d3Sel {
        match self.bits {
            0 => Gpio2d3Sel::B00,
            1 => Gpio2d3Sel::B01,
            2 => Gpio2d3Sel::B10,
            3 => Gpio2d3Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d3Sel::B00
    }
    #[doc = "sdio_pwren"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d3Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d3Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d3Sel::B11
    }
}
#[doc = "Field `GPIO2D3_SEL` writer - GPIO2D\\[3\\]
iomux select"]
pub type Gpio2d3SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d3Sel>;
impl<'a, REG> Gpio2d3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d3Sel::B00)
    }
    #[doc = "sdio_pwren"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d3Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d3Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d3Sel::B11)
    }
}
#[doc = "GPIO2D\\[4\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d4Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: sdio_bkpwr"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio2d4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2D4_SEL` reader - GPIO2D\\[4\\]
iomux select"]
pub type Gpio2d4SelR = crate::FieldReader<Gpio2d4Sel>;
impl Gpio2d4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d4Sel {
        match self.bits {
            0 => Gpio2d4Sel::B00,
            1 => Gpio2d4Sel::B01,
            2 => Gpio2d4Sel::B10,
            3 => Gpio2d4Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d4Sel::B00
    }
    #[doc = "sdio_bkpwr"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d4Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d4Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d4Sel::B11
    }
}
#[doc = "Field `GPIO2D4_SEL` writer - GPIO2D\\[4\\]
iomux select"]
pub type Gpio2d4SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d4Sel>;
impl<'a, REG> Gpio2d4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d4Sel::B00)
    }
    #[doc = "sdio_bkpwr"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d4Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d4Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d4Sel::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO2D\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2d0_sel(&self) -> Gpio2d0SelR {
        Gpio2d0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO2D\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2d1_sel(&self) -> Gpio2d1SelR {
        Gpio2d1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO2D\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2d2_sel(&self) -> Gpio2d2SelR {
        Gpio2d2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO2D\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2d3_sel(&self) -> Gpio2d3SelR {
        Gpio2d3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO2D\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2d4_sel(&self) -> Gpio2d4SelR {
        Gpio2d4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO2D\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d0_sel(&mut self) -> Gpio2d0SelW<GrfGpio2dIomuxSpec> {
        Gpio2d0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO2D\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d1_sel(&mut self) -> Gpio2d1SelW<GrfGpio2dIomuxSpec> {
        Gpio2d1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO2D\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d2_sel(&mut self) -> Gpio2d2SelW<GrfGpio2dIomuxSpec> {
        Gpio2d2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO2D\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d3_sel(&mut self) -> Gpio2d3SelW<GrfGpio2dIomuxSpec> {
        Gpio2d3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO2D\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d4_sel(&mut self) -> Gpio2d4SelW<GrfGpio2dIomuxSpec> {
        Gpio2d4SelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio2dIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2D iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2d_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2d_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio2dIomuxSpec;
impl crate::RegisterSpec for GrfGpio2dIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio2d_iomux::R`](R) reader structure"]
impl crate::Readable for GrfGpio2dIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio2d_iomux::W`](W) writer structure"]
impl crate::Writable for GrfGpio2dIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO2D_IOMUX to value 0"]
impl crate::Resettable for GrfGpio2dIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
