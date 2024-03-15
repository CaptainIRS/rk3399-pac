#[doc = "Register `PMUCRU_CLKSEL_CON0` reader"]
pub type R = crate::R<PmucruClkselCon0Spec>;
#[doc = "Register `PMUCRU_CLKSEL_CON0` writer"]
pub type W = crate::W<PmucruClkselCon0Spec>;
#[doc = "Field `PMU_PCLK_DIV_CON` reader - pmu_pclk divider control register clk=clk_src/(div_con+1)"]
pub type PmuPclkDivConR = crate::FieldReader;
#[doc = "Field `PMU_PCLK_DIV_CON` writer - pmu_pclk divider control register clk=clk_src/(div_con+1)"]
pub type PmuPclkDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CM0S_DIV_CON` reader - cm0s clock source select control register clk=clk_src/(div_con+1)"]
pub type Cm0sDivConR = crate::FieldReader;
#[doc = "Field `CM0S_DIV_CON` writer - cm0s clock source select control register clk=clk_src/(div_con+1)"]
pub type Cm0sDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "cm0s_clk divider control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cm0sClkPllSel {
    #[doc = "0: xin_24m"]
    B0 = 0,
    #[doc = "1: xin_24m"]
    B1 = 1,
}
impl From<Cm0sClkPllSel> for bool {
    #[inline(always)]
    fn from(variant: Cm0sClkPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CM0S_CLK_PLL_SEL` reader - cm0s_clk divider control register"]
pub type Cm0sClkPllSelR = crate::BitReader<Cm0sClkPllSel>;
impl Cm0sClkPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm0sClkPllSel {
        match self.bits {
            false => Cm0sClkPllSel::B0,
            true => Cm0sClkPllSel::B1,
        }
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cm0sClkPllSel::B0
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cm0sClkPllSel::B1
    }
}
#[doc = "Field `CM0S_CLK_PLL_SEL` writer - cm0s_clk divider control register"]
pub type Cm0sClkPllSelW<'a, REG> = crate::BitWriter<'a, REG, Cm0sClkPllSel>;
impl<'a, REG> Cm0sClkPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cm0sClkPllSel::B0)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cm0sClkPllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - pmu_pclk divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn pmu_pclk_div_con(&self) -> PmuPclkDivConR {
        PmuPclkDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - cm0s clock source select control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn cm0s_div_con(&self) -> Cm0sDivConR {
        Cm0sDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - cm0s_clk divider control register"]
    #[inline(always)]
    pub fn cm0s_clk_pll_sel(&self) -> Cm0sClkPllSelR {
        Cm0sClkPllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - pmu_pclk divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pmu_pclk_div_con(&mut self) -> PmuPclkDivConW<PmucruClkselCon0Spec> {
        PmuPclkDivConW::new(self, 0)
    }
    #[doc = "Bits 8:12 - cm0s clock source select control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn cm0s_div_con(&mut self) -> Cm0sDivConW<PmucruClkselCon0Spec> {
        Cm0sDivConW::new(self, 8)
    }
    #[doc = "Bit 15 - cm0s_clk divider control register"]
    #[inline(always)]
    #[must_use]
    pub fn cm0s_clk_pll_sel(&mut self) -> Cm0sClkPllSelW<PmucruClkselCon0Spec> {
        Cm0sClkPllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PmucruClkselCon0Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clksel_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clksel_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucruClkselCon0Spec;
impl crate::RegisterSpec for PmucruClkselCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucru_clksel_con0::R`](R) reader structure"]
impl crate::Readable for PmucruClkselCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmucru_clksel_con0::W`](W) writer structure"]
impl crate::Writable for PmucruClkselCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUCRU_CLKSEL_CON0 to value 0x0706"]
impl crate::Resettable for PmucruClkselCon0Spec {
    const RESET_VALUE: u32 = 0x0706;
}
