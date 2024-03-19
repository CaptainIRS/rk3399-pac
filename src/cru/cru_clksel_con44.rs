#[doc = "Register `CRU_CLKSEL_CON44` reader"]
pub type R = crate::R<CruClkselCon44Spec>;
#[doc = "Register `CRU_CLKSEL_CON44` writer"]
pub type W = crate::W<CruClkselCon44Spec>;
#[doc = "Field `PCLK_EDP_DIV_CON` reader - pclk_edp divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkEdpDivConR = crate::FieldReader;
#[doc = "Field `PCLK_EDP_DIV_CON` writer - pclk_edp divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkEdpDivConW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "pclk_edp clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PclkEdpPllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<PclkEdpPllSel> for bool {
    #[inline(always)]
    fn from(variant: PclkEdpPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCLK_EDP_PLL_SEL` reader - pclk_edp clock source select control register"]
pub type PclkEdpPllSelR = crate::BitReader<PclkEdpPllSel>;
impl PclkEdpPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PclkEdpPllSel {
        match self.bits {
            false => PclkEdpPllSel::B0,
            true => PclkEdpPllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PclkEdpPllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PclkEdpPllSel::B1
    }
}
#[doc = "Field `PCLK_EDP_PLL_SEL` writer - pclk_edp clock source select control register"]
pub type PclkEdpPllSelW<'a, REG> = crate::BitWriter<'a, REG, PclkEdpPllSel>;
impl<'a, REG> PclkEdpPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PclkEdpPllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PclkEdpPllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:13 - pclk_edp divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn pclk_edp_div_con(&self) -> PclkEdpDivConR {
        PclkEdpDivConR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - pclk_edp clock source select control register"]
    #[inline(always)]
    pub fn pclk_edp_pll_sel(&self) -> PclkEdpPllSelR {
        PclkEdpPllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13 - pclk_edp divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_edp_div_con(&mut self) -> PclkEdpDivConW<CruClkselCon44Spec> {
        PclkEdpDivConW::new(self, 8)
    }
    #[doc = "Bit 15 - pclk_edp clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_edp_pll_sel(&mut self) -> PclkEdpPllSelW<CruClkselCon44Spec> {
        PclkEdpPllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon44Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon44Spec;
impl crate::RegisterSpec for CruClkselCon44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con44::R`](R) reader structure"]
impl crate::Readable for CruClkselCon44Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con44::W`](W) writer structure"]
impl crate::Writable for CruClkselCon44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON44 to value 0x0700"]
impl crate::Resettable for CruClkselCon44Spec {
    const RESET_VALUE: u32 = 0x0700;
}
