#[doc = "Register `CLKSEL_CON59` reader"]
pub type R = crate::R<ClkselCon59Spec>;
#[doc = "Register `CLKSEL_CON59` writer"]
pub type W = crate::W<ClkselCon59Spec>;
#[doc = "Field `CLK_SPI0_DIV_CON` reader - spi0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkSpi0DivConR = crate::FieldReader;
#[doc = "Field `CLK_SPI0_DIV_CON` writer - spi0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkSpi0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_spi0 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSpi0PllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkSpi0PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkSpi0PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SPI0_PLL_SEL` reader - clk_spi0 clock source select control register"]
pub type ClkSpi0PllSelR = crate::BitReader<ClkSpi0PllSel>;
impl ClkSpi0PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSpi0PllSel {
        match self.bits {
            false => ClkSpi0PllSel::B0,
            true => ClkSpi0PllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkSpi0PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkSpi0PllSel::B1
    }
}
#[doc = "Field `CLK_SPI0_PLL_SEL` writer - clk_spi0 clock source select control register"]
pub type ClkSpi0PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkSpi0PllSel>;
impl<'a, REG> ClkSpi0PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpi0PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpi0PllSel::B1)
    }
}
#[doc = "Field `CLK_SPI1_DIV_CON` reader - spi1 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkSpi1DivConR = crate::FieldReader;
#[doc = "Field `CLK_SPI1_DIV_CON` writer - spi1 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkSpi1DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_spi1 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSpi1PllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkSpi1PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkSpi1PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SPI1_PLL_SEL` reader - clk_spi1 clock source select control register"]
pub type ClkSpi1PllSelR = crate::BitReader<ClkSpi1PllSel>;
impl ClkSpi1PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSpi1PllSel {
        match self.bits {
            false => ClkSpi1PllSel::B0,
            true => ClkSpi1PllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkSpi1PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkSpi1PllSel::B1
    }
}
#[doc = "Field `CLK_SPI1_PLL_SEL` writer - clk_spi1 clock source select control register"]
pub type ClkSpi1PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkSpi1PllSel>;
impl<'a, REG> ClkSpi1PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpi1PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpi1PllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - spi0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_spi0_div_con(&self) -> ClkSpi0DivConR {
        ClkSpi0DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - clk_spi0 clock source select control register"]
    #[inline(always)]
    pub fn clk_spi0_pll_sel(&self) -> ClkSpi0PllSelR {
        ClkSpi0PllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - spi1 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_spi1_div_con(&self) -> ClkSpi1DivConR {
        ClkSpi1DivConR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - clk_spi1 clock source select control register"]
    #[inline(always)]
    pub fn clk_spi1_pll_sel(&self) -> ClkSpi1PllSelR {
        ClkSpi1PllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - spi0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi0_div_con(&mut self) -> ClkSpi0DivConW<ClkselCon59Spec> {
        ClkSpi0DivConW::new(self, 0)
    }
    #[doc = "Bit 7 - clk_spi0 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi0_pll_sel(&mut self) -> ClkSpi0PllSelW<ClkselCon59Spec> {
        ClkSpi0PllSelW::new(self, 7)
    }
    #[doc = "Bits 8:14 - spi1 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi1_div_con(&mut self) -> ClkSpi1DivConW<ClkselCon59Spec> {
        ClkSpi1DivConW::new(self, 8)
    }
    #[doc = "Bit 15 - clk_spi1 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi1_pll_sel(&mut self) -> ClkSpi1PllSelW<ClkselCon59Spec> {
        ClkSpi1PllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon59Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con59::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con59::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon59Spec;
impl crate::RegisterSpec for ClkselCon59Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con59::R`](R) reader structure"]
impl crate::Readable for ClkselCon59Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con59::W`](W) writer structure"]
impl crate::Writable for ClkselCon59Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON59 to value 0x0707"]
impl crate::Resettable for ClkselCon59Spec {
    const RESET_VALUE: u32 = 0x0707;
}
