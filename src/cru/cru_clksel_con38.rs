#[doc = "Register `CRU_CLKSEL_CON38` reader"]
pub type R = crate::R<CruClkselCon38Spec>;
#[doc = "Register `CRU_CLKSEL_CON38` writer"]
pub type W = crate::W<CruClkselCon38Spec>;
#[doc = "Field `CLK_TESTOUT1_DIV_CON` reader - clk_testout1 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkTestout1DivConR = crate::FieldReader;
#[doc = "Field `CLK_TESTOUT1_DIV_CON` writer - clk_testout1 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkTestout1DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_testout1 clock select control register\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkTestout1ClkSel {
    #[doc = "0: clk_testout_src"]
    B0 = 0,
    #[doc = "1: xin_24m"]
    B1 = 1,
}
impl From<ClkTestout1ClkSel> for bool {
    #[inline(always)]
    fn from(variant: ClkTestout1ClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_TESTOUT1_CLK_SEL` reader - clk_testout1 clock select control register"]
pub type ClkTestout1ClkSelR = crate::BitReader<ClkTestout1ClkSel>;
impl ClkTestout1ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkTestout1ClkSel {
        match self.bits {
            false => ClkTestout1ClkSel::B0,
            true => ClkTestout1ClkSel::B1,
        }
    }
    #[doc = "clk_testout_src"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkTestout1ClkSel::B0
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkTestout1ClkSel::B1
    }
}
#[doc = "Field `CLK_TESTOUT1_CLK_SEL` writer - clk_testout1 clock select control register"]
pub type ClkTestout1ClkSelW<'a, REG> = crate::BitWriter<'a, REG, ClkTestout1ClkSel>;
impl<'a, REG> ClkTestout1ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clk_testout_src"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTestout1ClkSel::B0)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTestout1ClkSel::B1)
    }
}
#[doc = "clk_testout1 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkTestout1PllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B1x = 2,
}
impl From<ClkTestout1PllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkTestout1PllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkTestout1PllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_TESTOUT1_PLL_SEL` reader - clk_testout1 clock source select control register"]
pub type ClkTestout1PllSelR = crate::FieldReader<ClkTestout1PllSel>;
impl ClkTestout1PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkTestout1PllSel> {
        match self.bits {
            0 => Some(ClkTestout1PllSel::B00),
            1 => Some(ClkTestout1PllSel::B01),
            2 => Some(ClkTestout1PllSel::B1x),
            _ => None,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkTestout1PllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkTestout1PllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkTestout1PllSel::B1x
    }
}
#[doc = "Field `CLK_TESTOUT1_PLL_SEL` writer - clk_testout1 clock source select control register"]
pub type ClkTestout1PllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkTestout1PllSel>;
impl<'a, REG> ClkTestout1PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTestout1PllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTestout1PllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTestout1PllSel::B1x)
    }
}
#[doc = "Field `CLK_TESTOUT2_DIV_CON` reader - clk_testout2 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkTestout2DivConR = crate::FieldReader;
#[doc = "Field `CLK_TESTOUT2_DIV_CON` writer - clk_testout2 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkTestout2DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_testout2 clock select control register\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkTestout2ClkSel {
    #[doc = "0: clk_testout_src"]
    B0 = 0,
    #[doc = "1: xin_24m"]
    B1 = 1,
}
impl From<ClkTestout2ClkSel> for bool {
    #[inline(always)]
    fn from(variant: ClkTestout2ClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_TESTOUT2_CLK_SEL` reader - clk_testout2 clock select control register"]
pub type ClkTestout2ClkSelR = crate::BitReader<ClkTestout2ClkSel>;
impl ClkTestout2ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkTestout2ClkSel {
        match self.bits {
            false => ClkTestout2ClkSel::B0,
            true => ClkTestout2ClkSel::B1,
        }
    }
    #[doc = "clk_testout_src"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkTestout2ClkSel::B0
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkTestout2ClkSel::B1
    }
}
#[doc = "Field `CLK_TESTOUT2_CLK_SEL` writer - clk_testout2 clock select control register"]
pub type ClkTestout2ClkSelW<'a, REG> = crate::BitWriter<'a, REG, ClkTestout2ClkSel>;
impl<'a, REG> ClkTestout2ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clk_testout_src"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTestout2ClkSel::B0)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTestout2ClkSel::B1)
    }
}
#[doc = "clk_testout2 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkTestout2PllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B1x = 2,
}
impl From<ClkTestout2PllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkTestout2PllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkTestout2PllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_TESTOUT2_PLL_SEL` reader - clk_testout2 clock source select control register"]
pub type ClkTestout2PllSelR = crate::FieldReader<ClkTestout2PllSel>;
impl ClkTestout2PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkTestout2PllSel> {
        match self.bits {
            0 => Some(ClkTestout2PllSel::B00),
            1 => Some(ClkTestout2PllSel::B01),
            2 => Some(ClkTestout2PllSel::B1x),
            _ => None,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkTestout2PllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkTestout2PllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkTestout2PllSel::B1x
    }
}
#[doc = "Field `CLK_TESTOUT2_PLL_SEL` writer - clk_testout2 clock source select control register"]
pub type ClkTestout2PllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkTestout2PllSel>;
impl<'a, REG> ClkTestout2PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTestout2PllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTestout2PllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTestout2PllSel::B1x)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - clk_testout1 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_testout1_div_con(&self) -> ClkTestout1DivConR {
        ClkTestout1DivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - clk_testout1 clock select control register"]
    #[inline(always)]
    pub fn clk_testout1_clk_sel(&self) -> ClkTestout1ClkSelR {
        ClkTestout1ClkSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - clk_testout1 clock source select control register"]
    #[inline(always)]
    pub fn clk_testout1_pll_sel(&self) -> ClkTestout1PllSelR {
        ClkTestout1PllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - clk_testout2 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_testout2_div_con(&self) -> ClkTestout2DivConR {
        ClkTestout2DivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - clk_testout2 clock select control register"]
    #[inline(always)]
    pub fn clk_testout2_clk_sel(&self) -> ClkTestout2ClkSelR {
        ClkTestout2ClkSelR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - clk_testout2 clock source select control register"]
    #[inline(always)]
    pub fn clk_testout2_pll_sel(&self) -> ClkTestout2PllSelR {
        ClkTestout2PllSelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk_testout1 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_testout1_div_con(&mut self) -> ClkTestout1DivConW<CruClkselCon38Spec> {
        ClkTestout1DivConW::new(self, 0)
    }
    #[doc = "Bit 5 - clk_testout1 clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_testout1_clk_sel(&mut self) -> ClkTestout1ClkSelW<CruClkselCon38Spec> {
        ClkTestout1ClkSelW::new(self, 5)
    }
    #[doc = "Bits 6:7 - clk_testout1 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_testout1_pll_sel(&mut self) -> ClkTestout1PllSelW<CruClkselCon38Spec> {
        ClkTestout1PllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - clk_testout2 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_testout2_div_con(&mut self) -> ClkTestout2DivConW<CruClkselCon38Spec> {
        ClkTestout2DivConW::new(self, 8)
    }
    #[doc = "Bit 13 - clk_testout2 clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_testout2_clk_sel(&mut self) -> ClkTestout2ClkSelW<CruClkselCon38Spec> {
        ClkTestout2ClkSelW::new(self, 13)
    }
    #[doc = "Bits 14:15 - clk_testout2 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_testout2_pll_sel(&mut self) -> ClkTestout2PllSelW<CruClkselCon38Spec> {
        ClkTestout2PllSelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon38Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon38Spec;
impl crate::RegisterSpec for CruClkselCon38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con38::R`](R) reader structure"]
impl crate::Readable for CruClkselCon38Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con38::W`](W) writer structure"]
impl crate::Writable for CruClkselCon38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON38 to value 0x3f3f"]
impl crate::Resettable for CruClkselCon38Spec {
    const RESET_VALUE: u32 = 0x3f3f;
}
