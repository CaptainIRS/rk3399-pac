#[doc = "Register `CRU_CLKSEL_CON46` reader"]
pub type R = crate::R<CruClkselCon46Spec>;
#[doc = "Register `CRU_CLKSEL_CON46` writer"]
pub type W = crate::W<CruClkselCon46Spec>;
#[doc = "Field `CLK_DP_CORE_DIV_CON` reader - clk_dp_core divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkDpCoreDivConR = crate::FieldReader;
#[doc = "Field `CLK_DP_CORE_DIV_CON` writer - clk_dp_core divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkDpCoreDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_dp_core clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkDpCorePllSel {
    #[doc = "0: NPLL"]
    B00 = 0,
    #[doc = "1: CPLL"]
    B01 = 1,
    #[doc = "2: GPLL"]
    B10 = 2,
}
impl From<ClkDpCorePllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkDpCorePllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkDpCorePllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_DP_CORE_PLL_SEL` reader - clk_dp_core clock source select control register"]
pub type ClkDpCorePllSelR = crate::FieldReader<ClkDpCorePllSel>;
impl ClkDpCorePllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkDpCorePllSel> {
        match self.bits {
            0 => Some(ClkDpCorePllSel::B00),
            1 => Some(ClkDpCorePllSel::B01),
            2 => Some(ClkDpCorePllSel::B10),
            _ => None,
        }
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkDpCorePllSel::B00
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkDpCorePllSel::B01
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkDpCorePllSel::B10
    }
}
#[doc = "Field `CLK_DP_CORE_PLL_SEL` writer - clk_dp_core clock source select control register"]
pub type ClkDpCorePllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkDpCorePllSel>;
impl<'a, REG> ClkDpCorePllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDpCorePllSel::B00)
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDpCorePllSel::B01)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDpCorePllSel::B10)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - clk_dp_core divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_dp_core_div_con(&self) -> ClkDpCoreDivConR {
        ClkDpCoreDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - clk_dp_core clock source select control register"]
    #[inline(always)]
    pub fn clk_dp_core_pll_sel(&self) -> ClkDpCorePllSelR {
        ClkDpCorePllSelR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk_dp_core divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dp_core_div_con(&mut self) -> ClkDpCoreDivConW<CruClkselCon46Spec> {
        ClkDpCoreDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - clk_dp_core clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dp_core_pll_sel(&mut self) -> ClkDpCorePllSelW<CruClkselCon46Spec> {
        ClkDpCorePllSelW::new(self, 6)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon46Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con46::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con46::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon46Spec;
impl crate::RegisterSpec for CruClkselCon46Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con46::R`](R) reader structure"]
impl crate::Readable for CruClkselCon46Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con46::W`](W) writer structure"]
impl crate::Writable for CruClkselCon46Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON46 to value 0x04"]
impl crate::Resettable for CruClkselCon46Spec {
    const RESET_VALUE: u32 = 0x04;
}
