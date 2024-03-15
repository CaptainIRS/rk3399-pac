#[doc = "Register `CRU_CLKSEL_CON9` reader"]
pub type R = crate::R<CruClkselCon9Spec>;
#[doc = "Register `CRU_CLKSEL_CON9` writer"]
pub type W = crate::W<CruClkselCon9Spec>;
#[doc = "Field `CLK_VDU_CORE_DIV_CON` reader - clk_vdu_core divider control register clk=clk_src/(div_con+1)"]
pub type ClkVduCoreDivConR = crate::FieldReader;
#[doc = "Field `CLK_VDU_CORE_DIV_CON` writer - clk_vdu_core divider control register clk=clk_src/(div_con+1)"]
pub type ClkVduCoreDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_vdu_core clock source select control register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkVduCorePllSel {
    #[doc = "0: NPLL"]
    B00 = 0,
    #[doc = "1: NPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B1x = 2,
}
impl From<ClkVduCorePllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkVduCorePllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkVduCorePllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_VDU_CORE_PLL_SEL` reader - clk_vdu_core clock source select control register"]
pub type ClkVduCorePllSelR = crate::FieldReader<ClkVduCorePllSel>;
impl ClkVduCorePllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkVduCorePllSel> {
        match self.bits {
            0 => Some(ClkVduCorePllSel::B00),
            1 => Some(ClkVduCorePllSel::B01),
            2 => Some(ClkVduCorePllSel::B1x),
            _ => None,
        }
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkVduCorePllSel::B00
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkVduCorePllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkVduCorePllSel::B1x
    }
}
#[doc = "Field `CLK_VDU_CORE_PLL_SEL` writer - clk_vdu_core clock source select control register"]
pub type ClkVduCorePllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkVduCorePllSel>;
impl<'a, REG> ClkVduCorePllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkVduCorePllSel::B00)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkVduCorePllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkVduCorePllSel::B1x)
    }
}
#[doc = "Field `CLK_VDU_CA_DIV_CON` reader - clk_vdu_ca divider control register clk=clk_src/(div_con+1)"]
pub type ClkVduCaDivConR = crate::FieldReader;
#[doc = "Field `CLK_VDU_CA_DIV_CON` writer - clk_vdu_ca divider control register clk=clk_src/(div_con+1)"]
pub type ClkVduCaDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_vdu_ca clock source select control register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkVduCaPllSel {
    #[doc = "0: NPLL"]
    B00 = 0,
    #[doc = "1: NPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B1x = 2,
}
impl From<ClkVduCaPllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkVduCaPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkVduCaPllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_VDU_CA_PLL_SEL` reader - clk_vdu_ca clock source select control register"]
pub type ClkVduCaPllSelR = crate::FieldReader<ClkVduCaPllSel>;
impl ClkVduCaPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkVduCaPllSel> {
        match self.bits {
            0 => Some(ClkVduCaPllSel::B00),
            1 => Some(ClkVduCaPllSel::B01),
            2 => Some(ClkVduCaPllSel::B1x),
            _ => None,
        }
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkVduCaPllSel::B00
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkVduCaPllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkVduCaPllSel::B1x
    }
}
#[doc = "Field `CLK_VDU_CA_PLL_SEL` writer - clk_vdu_ca clock source select control register"]
pub type ClkVduCaPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkVduCaPllSel>;
impl<'a, REG> ClkVduCaPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkVduCaPllSel::B00)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkVduCaPllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkVduCaPllSel::B1x)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - clk_vdu_core divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_vdu_core_div_con(&self) -> ClkVduCoreDivConR {
        ClkVduCoreDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - clk_vdu_core clock source select control register"]
    #[inline(always)]
    pub fn clk_vdu_core_pll_sel(&self) -> ClkVduCorePllSelR {
        ClkVduCorePllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - clk_vdu_ca divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_vdu_ca_div_con(&self) -> ClkVduCaDivConR {
        ClkVduCaDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - clk_vdu_ca clock source select control register"]
    #[inline(always)]
    pub fn clk_vdu_ca_pll_sel(&self) -> ClkVduCaPllSelR {
        ClkVduCaPllSelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk_vdu_core divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vdu_core_div_con(&mut self) -> ClkVduCoreDivConW<CruClkselCon9Spec> {
        ClkVduCoreDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - clk_vdu_core clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vdu_core_pll_sel(&mut self) -> ClkVduCorePllSelW<CruClkselCon9Spec> {
        ClkVduCorePllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - clk_vdu_ca divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vdu_ca_div_con(&mut self) -> ClkVduCaDivConW<CruClkselCon9Spec> {
        ClkVduCaDivConW::new(self, 8)
    }
    #[doc = "Bits 14:15 - clk_vdu_ca clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vdu_ca_pll_sel(&mut self) -> ClkVduCaPllSelW<CruClkselCon9Spec> {
        ClkVduCaPllSelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon9Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon9Spec;
impl crate::RegisterSpec for CruClkselCon9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con9::R`](R) reader structure"]
impl crate::Readable for CruClkselCon9Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con9::W`](W) writer structure"]
impl crate::Writable for CruClkselCon9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON9 to value 0x4141"]
impl crate::Resettable for CruClkselCon9Spec {
    const RESET_VALUE: u32 = 0x4141;
}
