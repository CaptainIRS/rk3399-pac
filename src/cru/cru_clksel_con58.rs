#[doc = "Register `CRU_CLKSEL_CON58` reader"]
pub type R = crate::R<CruClkselCon58Spec>;
#[doc = "Register `CRU_CLKSEL_CON58` writer"]
pub type W = crate::W<CruClkselCon58Spec>;
#[doc = "Field `CLK_TEST_DIV_CON` reader - test divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkTestDivConR = crate::FieldReader;
#[doc = "Field `CLK_TEST_DIV_CON` writer - test divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkTestDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_frac clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkTestfracPllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkTestfracPllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkTestfracPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_TESTFRAC_PLL_SEL` reader - clk_frac clock source select control register"]
pub type ClkTestfracPllSelR = crate::BitReader<ClkTestfracPllSel>;
impl ClkTestfracPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkTestfracPllSel {
        match self.bits {
            false => ClkTestfracPllSel::B0,
            true => ClkTestfracPllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkTestfracPllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkTestfracPllSel::B1
    }
}
#[doc = "Field `CLK_TESTFRAC_PLL_SEL` writer - clk_frac clock source select control register"]
pub type ClkTestfracPllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkTestfracPllSel>;
impl<'a, REG> ClkTestfracPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTestfracPllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTestfracPllSel::B1)
    }
}
#[doc = "Field `CLK_SPI5_DIV_CON` reader - spi5 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkSpi5DivConR = crate::FieldReader;
#[doc = "Field `CLK_SPI5_DIV_CON` writer - spi5 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkSpi5DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_spi5 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSpi5PllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkSpi5PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkSpi5PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SPI5_PLL_SEL` reader - clk_spi5 clock source select control register"]
pub type ClkSpi5PllSelR = crate::BitReader<ClkSpi5PllSel>;
impl ClkSpi5PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSpi5PllSel {
        match self.bits {
            false => ClkSpi5PllSel::B0,
            true => ClkSpi5PllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkSpi5PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkSpi5PllSel::B1
    }
}
#[doc = "Field `CLK_SPI5_PLL_SEL` writer - clk_spi5 clock source select control register"]
pub type ClkSpi5PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkSpi5PllSel>;
impl<'a, REG> ClkSpi5PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpi5PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpi5PllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - test divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_test_div_con(&self) -> ClkTestDivConR {
        ClkTestDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - clk_frac clock source select control register"]
    #[inline(always)]
    pub fn clk_testfrac_pll_sel(&self) -> ClkTestfracPllSelR {
        ClkTestfracPllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - spi5 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_spi5_div_con(&self) -> ClkSpi5DivConR {
        ClkSpi5DivConR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - clk_spi5 clock source select control register"]
    #[inline(always)]
    pub fn clk_spi5_pll_sel(&self) -> ClkSpi5PllSelR {
        ClkSpi5PllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - test divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_test_div_con(&mut self) -> ClkTestDivConW<CruClkselCon58Spec> {
        ClkTestDivConW::new(self, 0)
    }
    #[doc = "Bit 7 - clk_frac clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_testfrac_pll_sel(&mut self) -> ClkTestfracPllSelW<CruClkselCon58Spec> {
        ClkTestfracPllSelW::new(self, 7)
    }
    #[doc = "Bits 8:14 - spi5 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi5_div_con(&mut self) -> ClkSpi5DivConW<CruClkselCon58Spec> {
        ClkSpi5DivConW::new(self, 8)
    }
    #[doc = "Bit 15 - clk_spi5 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi5_pll_sel(&mut self) -> ClkSpi5PllSelW<CruClkselCon58Spec> {
        ClkSpi5PllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon58Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con58::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con58::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon58Spec;
impl crate::RegisterSpec for CruClkselCon58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con58::R`](R) reader structure"]
impl crate::Readable for CruClkselCon58Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con58::W`](W) writer structure"]
impl crate::Writable for CruClkselCon58Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON58 to value 0x071f"]
impl crate::Resettable for CruClkselCon58Spec {
    const RESET_VALUE: u32 = 0x071f;
}
