#[doc = "Register `CRU_CLKSEL_CON25` reader"]
pub type R = crate::R<CruClkselCon25Spec>;
#[doc = "Register `CRU_CLKSEL_CON25` writer"]
pub type W = crate::W<CruClkselCon25Spec>;
#[doc = "Field `HCLK_PERILP1_DIV_CON` reader - hclk_perilp1 divider control register clk=clk_src/(div_con+1)"]
pub type HclkPerilp1DivConR = crate::FieldReader;
#[doc = "Field `HCLK_PERILP1_DIV_CON` writer - hclk_perilp1 divider control register clk=clk_src/(div_con+1)"]
pub type HclkPerilp1DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "hclk_perilp1 clock source select control register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HclkPerilp1PllSel {
    #[doc = "0: GPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<HclkPerilp1PllSel> for bool {
    #[inline(always)]
    fn from(variant: HclkPerilp1PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HCLK_PERILP1_PLL_SEL` reader - hclk_perilp1 clock source select control register"]
pub type HclkPerilp1PllSelR = crate::BitReader<HclkPerilp1PllSel>;
impl HclkPerilp1PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HclkPerilp1PllSel {
        match self.bits {
            false => HclkPerilp1PllSel::B0,
            true => HclkPerilp1PllSel::B1,
        }
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HclkPerilp1PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HclkPerilp1PllSel::B1
    }
}
#[doc = "Field `HCLK_PERILP1_PLL_SEL` writer - hclk_perilp1 clock source select control register"]
pub type HclkPerilp1PllSelW<'a, REG> = crate::BitWriter<'a, REG, HclkPerilp1PllSel>;
impl<'a, REG> HclkPerilp1PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HclkPerilp1PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HclkPerilp1PllSel::B1)
    }
}
#[doc = "Field `PCLK_PERILP1_DIV_CON` reader - pclk_perilp1 divider control register clk=hclk_perilp1/(div_con+1)"]
pub type PclkPerilp1DivConR = crate::FieldReader;
#[doc = "Field `PCLK_PERILP1_DIV_CON` writer - pclk_perilp1 divider control register clk=hclk_perilp1/(div_con+1)"]
pub type PclkPerilp1DivConW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - hclk_perilp1 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn hclk_perilp1_div_con(&self) -> HclkPerilp1DivConR {
        HclkPerilp1DivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - hclk_perilp1 clock source select control register"]
    #[inline(always)]
    pub fn hclk_perilp1_pll_sel(&self) -> HclkPerilp1PllSelR {
        HclkPerilp1PllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - pclk_perilp1 divider control register clk=hclk_perilp1/(div_con+1)"]
    #[inline(always)]
    pub fn pclk_perilp1_div_con(&self) -> PclkPerilp1DivConR {
        PclkPerilp1DivConR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - hclk_perilp1 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_perilp1_div_con(&mut self) -> HclkPerilp1DivConW<CruClkselCon25Spec> {
        HclkPerilp1DivConW::new(self, 0)
    }
    #[doc = "Bit 7 - hclk_perilp1 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_perilp1_pll_sel(&mut self) -> HclkPerilp1PllSelW<CruClkselCon25Spec> {
        HclkPerilp1PllSelW::new(self, 7)
    }
    #[doc = "Bits 8:10 - pclk_perilp1 divider control register clk=hclk_perilp1/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_perilp1_div_con(&mut self) -> PclkPerilp1DivConW<CruClkselCon25Spec> {
        PclkPerilp1DivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon25Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon25Spec;
impl crate::RegisterSpec for CruClkselCon25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con25::R`](R) reader structure"]
impl crate::Readable for CruClkselCon25Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con25::W`](W) writer structure"]
impl crate::Writable for CruClkselCon25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON25 to value 0x0183"]
impl crate::Resettable for CruClkselCon25Spec {
    const RESET_VALUE: u32 = 0x0183;
}
