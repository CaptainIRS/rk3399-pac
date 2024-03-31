#[doc = "Register `CLKSEL_CON30` reader"]
pub type R = crate::R<ClkselCon30Spec>;
#[doc = "Register `CLKSEL_CON30` writer"]
pub type W = crate::W<ClkselCon30Spec>;
#[doc = "Field `CLK_I2S2_DIV_CON` reader - clk_i2s2 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkI2s2DivConR = crate::FieldReader;
#[doc = "Field `CLK_I2S2_DIV_CON` writer - clk_i2s2 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkI2s2DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_i2s2 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkI2s2PllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkI2s2PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkI2s2PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_I2S2_PLL_SEL` reader - clk_i2s2 clock source select control register"]
pub type ClkI2s2PllSelR = crate::BitReader<ClkI2s2PllSel>;
impl ClkI2s2PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2s2PllSel {
        match self.bits {
            false => ClkI2s2PllSel::B0,
            true => ClkI2s2PllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkI2s2PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkI2s2PllSel::B1
    }
}
#[doc = "Field `CLK_I2S2_PLL_SEL` writer - clk_i2s2 clock source select control register"]
pub type ClkI2s2PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkI2s2PllSel>;
impl<'a, REG> ClkI2s2PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s2PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s2PllSel::B1)
    }
}
#[doc = "clk_i2s2 clock select control register\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkI2s2Sel {
    #[doc = "0: clk_i2s2_divout"]
    B00 = 0,
    #[doc = "1: clk_i2s2_frac"]
    B01 = 1,
    #[doc = "2: clkin_i2s2 from IO"]
    B10 = 2,
    #[doc = "3: clk_12m"]
    B11 = 3,
}
impl From<ClkI2s2Sel> for u8 {
    #[inline(always)]
    fn from(variant: ClkI2s2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkI2s2Sel {
    type Ux = u8;
}
#[doc = "Field `CLK_I2S2_SEL` reader - clk_i2s2 clock select control register"]
pub type ClkI2s2SelR = crate::FieldReader<ClkI2s2Sel>;
impl ClkI2s2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2s2Sel {
        match self.bits {
            0 => ClkI2s2Sel::B00,
            1 => ClkI2s2Sel::B01,
            2 => ClkI2s2Sel::B10,
            3 => ClkI2s2Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "clk_i2s2_divout"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkI2s2Sel::B00
    }
    #[doc = "clk_i2s2_frac"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkI2s2Sel::B01
    }
    #[doc = "clkin_i2s2 from IO"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkI2s2Sel::B10
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ClkI2s2Sel::B11
    }
}
#[doc = "Field `CLK_I2S2_SEL` writer - clk_i2s2 clock select control register"]
pub type ClkI2s2SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ClkI2s2Sel>;
impl<'a, REG> ClkI2s2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clk_i2s2_divout"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s2Sel::B00)
    }
    #[doc = "clk_i2s2_frac"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s2Sel::B01)
    }
    #[doc = "clkin_i2s2 from IO"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s2Sel::B10)
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s2Sel::B11)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_i2s2 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_i2s2_div_con(&self) -> ClkI2s2DivConR {
        ClkI2s2DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - clk_i2s2 clock source select control register"]
    #[inline(always)]
    pub fn clk_i2s2_pll_sel(&self) -> ClkI2s2PllSelR {
        ClkI2s2PllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - clk_i2s2 clock select control register"]
    #[inline(always)]
    pub fn clk_i2s2_sel(&self) -> ClkI2s2SelR {
        ClkI2s2SelR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_i2s2 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s2_div_con(&mut self) -> ClkI2s2DivConW<ClkselCon30Spec> {
        ClkI2s2DivConW::new(self, 0)
    }
    #[doc = "Bit 7 - clk_i2s2 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s2_pll_sel(&mut self) -> ClkI2s2PllSelW<ClkselCon30Spec> {
        ClkI2s2PllSelW::new(self, 7)
    }
    #[doc = "Bits 8:9 - clk_i2s2 clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s2_sel(&mut self) -> ClkI2s2SelW<ClkselCon30Spec> {
        ClkI2s2SelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon30Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon30Spec;
impl crate::RegisterSpec for ClkselCon30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con30::R`](R) reader structure"]
impl crate::Readable for ClkselCon30Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con30::W`](W) writer structure"]
impl crate::Writable for ClkselCon30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON30 to value 0x0300"]
impl crate::Resettable for ClkselCon30Spec {
    const RESET_VALUE: u32 = 0x0300;
}
