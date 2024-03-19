#[doc = "Register `CRU_CLKSEL_CON61` reader"]
pub type R = crate::R<CruClkselCon61Spec>;
#[doc = "Register `CRU_CLKSEL_CON61` writer"]
pub type W = crate::W<CruClkselCon61Spec>;
#[doc = "Field `CLK_I2C1_DIV_CON` reader - i2c1 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkI2c1DivConR = crate::FieldReader;
#[doc = "Field `CLK_I2C1_DIV_CON` writer - i2c1 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkI2c1DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_i2c1 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkI2c1PllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkI2c1PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkI2c1PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_I2C1_PLL_SEL` reader - clk_i2c1 clock source select control register"]
pub type ClkI2c1PllSelR = crate::BitReader<ClkI2c1PllSel>;
impl ClkI2c1PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2c1PllSel {
        match self.bits {
            false => ClkI2c1PllSel::B0,
            true => ClkI2c1PllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkI2c1PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkI2c1PllSel::B1
    }
}
#[doc = "Field `CLK_I2C1_PLL_SEL` writer - clk_i2c1 clock source select control register"]
pub type ClkI2c1PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkI2c1PllSel>;
impl<'a, REG> ClkI2c1PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2c1PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2c1PllSel::B1)
    }
}
#[doc = "Field `CLK_I2C5_DIV_CON` reader - i2c5 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkI2c5DivConR = crate::FieldReader;
#[doc = "Field `CLK_I2C5_DIV_CON` writer - i2c5 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkI2c5DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_i2c5 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkI2c5PllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkI2c5PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkI2c5PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_I2C5_PLL_SEL` reader - clk_i2c5 clock source select control register"]
pub type ClkI2c5PllSelR = crate::BitReader<ClkI2c5PllSel>;
impl ClkI2c5PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2c5PllSel {
        match self.bits {
            false => ClkI2c5PllSel::B0,
            true => ClkI2c5PllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkI2c5PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkI2c5PllSel::B1
    }
}
#[doc = "Field `CLK_I2C5_PLL_SEL` writer - clk_i2c5 clock source select control register"]
pub type ClkI2c5PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkI2c5PllSel>;
impl<'a, REG> ClkI2c5PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2c5PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2c5PllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - i2c1 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_i2c1_div_con(&self) -> ClkI2c1DivConR {
        ClkI2c1DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - clk_i2c1 clock source select control register"]
    #[inline(always)]
    pub fn clk_i2c1_pll_sel(&self) -> ClkI2c1PllSelR {
        ClkI2c1PllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - i2c5 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_i2c5_div_con(&self) -> ClkI2c5DivConR {
        ClkI2c5DivConR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - clk_i2c5 clock source select control register"]
    #[inline(always)]
    pub fn clk_i2c5_pll_sel(&self) -> ClkI2c5PllSelR {
        ClkI2c5PllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - i2c1 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c1_div_con(&mut self) -> ClkI2c1DivConW<CruClkselCon61Spec> {
        ClkI2c1DivConW::new(self, 0)
    }
    #[doc = "Bit 7 - clk_i2c1 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c1_pll_sel(&mut self) -> ClkI2c1PllSelW<CruClkselCon61Spec> {
        ClkI2c1PllSelW::new(self, 7)
    }
    #[doc = "Bits 8:14 - i2c5 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c5_div_con(&mut self) -> ClkI2c5DivConW<CruClkselCon61Spec> {
        ClkI2c5DivConW::new(self, 8)
    }
    #[doc = "Bit 15 - clk_i2c5 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c5_pll_sel(&mut self) -> ClkI2c5PllSelW<CruClkselCon61Spec> {
        ClkI2c5PllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon61Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con61::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con61::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon61Spec;
impl crate::RegisterSpec for CruClkselCon61Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con61::R`](R) reader structure"]
impl crate::Readable for CruClkselCon61Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con61::W`](W) writer structure"]
impl crate::Writable for CruClkselCon61Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON61 to value 0x0303"]
impl crate::Resettable for CruClkselCon61Spec {
    const RESET_VALUE: u32 = 0x0303;
}
