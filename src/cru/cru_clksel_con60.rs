#[doc = "Register `CRU_CLKSEL_CON60` reader"]
pub type R = crate::R<CruClkselCon60Spec>;
#[doc = "Register `CRU_CLKSEL_CON60` writer"]
pub type W = crate::W<CruClkselCon60Spec>;
#[doc = "Field `CLK_SPI2_DIV_CON` reader - spi2 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkSpi2DivConR = crate::FieldReader;
#[doc = "Field `CLK_SPI2_DIV_CON` writer - spi2 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkSpi2DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_spi2 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSpi2PllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkSpi2PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkSpi2PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SPI2_PLL_SEL` reader - clk_spi2 clock source select control register"]
pub type ClkSpi2PllSelR = crate::BitReader<ClkSpi2PllSel>;
impl ClkSpi2PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSpi2PllSel {
        match self.bits {
            false => ClkSpi2PllSel::B0,
            true => ClkSpi2PllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkSpi2PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkSpi2PllSel::B1
    }
}
#[doc = "Field `CLK_SPI2_PLL_SEL` writer - clk_spi2 clock source select control register"]
pub type ClkSpi2PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkSpi2PllSel>;
impl<'a, REG> ClkSpi2PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpi2PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpi2PllSel::B1)
    }
}
#[doc = "Field `CLK_SPI4_DIV_CON` reader - spi4 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkSpi4DivConR = crate::FieldReader;
#[doc = "Field `CLK_SPI4_DIV_CON` writer - spi4 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkSpi4DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_spi4 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSpi4PllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkSpi4PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkSpi4PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SPI4_PLL_SEL` reader - clk_spi4 clock source select control register"]
pub type ClkSpi4PllSelR = crate::BitReader<ClkSpi4PllSel>;
impl ClkSpi4PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSpi4PllSel {
        match self.bits {
            false => ClkSpi4PllSel::B0,
            true => ClkSpi4PllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkSpi4PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkSpi4PllSel::B1
    }
}
#[doc = "Field `CLK_SPI4_PLL_SEL` writer - clk_spi4 clock source select control register"]
pub type ClkSpi4PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkSpi4PllSel>;
impl<'a, REG> ClkSpi4PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpi4PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpi4PllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - spi2 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_spi2_div_con(&self) -> ClkSpi2DivConR {
        ClkSpi2DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - clk_spi2 clock source select control register"]
    #[inline(always)]
    pub fn clk_spi2_pll_sel(&self) -> ClkSpi2PllSelR {
        ClkSpi2PllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - spi4 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_spi4_div_con(&self) -> ClkSpi4DivConR {
        ClkSpi4DivConR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - clk_spi4 clock source select control register"]
    #[inline(always)]
    pub fn clk_spi4_pll_sel(&self) -> ClkSpi4PllSelR {
        ClkSpi4PllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - spi2 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi2_div_con(&mut self) -> ClkSpi2DivConW<CruClkselCon60Spec> {
        ClkSpi2DivConW::new(self, 0)
    }
    #[doc = "Bit 7 - clk_spi2 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi2_pll_sel(&mut self) -> ClkSpi2PllSelW<CruClkselCon60Spec> {
        ClkSpi2PllSelW::new(self, 7)
    }
    #[doc = "Bits 8:14 - spi4 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi4_div_con(&mut self) -> ClkSpi4DivConW<CruClkselCon60Spec> {
        ClkSpi4DivConW::new(self, 8)
    }
    #[doc = "Bit 15 - clk_spi4 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi4_pll_sel(&mut self) -> ClkSpi4PllSelW<CruClkselCon60Spec> {
        ClkSpi4PllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon60Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con60::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con60::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon60Spec;
impl crate::RegisterSpec for CruClkselCon60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con60::R`](R) reader structure"]
impl crate::Readable for CruClkselCon60Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con60::W`](W) writer structure"]
impl crate::Writable for CruClkselCon60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON60 to value 0x0707"]
impl crate::Resettable for CruClkselCon60Spec {
    const RESET_VALUE: u32 = 0x0707;
}
