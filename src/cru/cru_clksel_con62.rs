#[doc = "Register `CRU_CLKSEL_CON62` reader"]
pub type R = crate::R<CruClkselCon62Spec>;
#[doc = "Register `CRU_CLKSEL_CON62` writer"]
pub type W = crate::W<CruClkselCon62Spec>;
#[doc = "Field `CLK_I2C2_DIV_CON` reader - i2c2 divider control register clk=clk_src/(div_con+1)"]
pub type ClkI2c2DivConR = crate::FieldReader;
#[doc = "Field `CLK_I2C2_DIV_CON` writer - i2c2 divider control register clk=clk_src/(div_con+1)"]
pub type ClkI2c2DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_i2c2 clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkI2c2PllSel {
    #[doc = "0: GPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkI2c2PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkI2c2PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_I2C2_PLL_SEL` reader - clk_i2c2 clock source select control register"]
pub type ClkI2c2PllSelR = crate::BitReader<ClkI2c2PllSel>;
impl ClkI2c2PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2c2PllSel {
        match self.bits {
            false => ClkI2c2PllSel::B0,
            true => ClkI2c2PllSel::B1,
        }
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkI2c2PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkI2c2PllSel::B1
    }
}
#[doc = "Field `CLK_I2C2_PLL_SEL` writer - clk_i2c2 clock source select control register"]
pub type ClkI2c2PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkI2c2PllSel>;
impl<'a, REG> ClkI2c2PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2c2PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2c2PllSel::B1)
    }
}
#[doc = "Field `CLK_I2C6_DIV_CON` reader - i2c6 divider control register clk=clk_src/(div_con+1)"]
pub type ClkI2c6DivConR = crate::FieldReader;
#[doc = "Field `CLK_I2C6_DIV_CON` writer - i2c6 divider control register clk=clk_src/(div_con+1)"]
pub type ClkI2c6DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_i2c6 clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkI2c6PllSel {
    #[doc = "0: GPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkI2c6PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkI2c6PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_I2C6_PLL_SEL` reader - clk_i2c6 clock source select control register"]
pub type ClkI2c6PllSelR = crate::BitReader<ClkI2c6PllSel>;
impl ClkI2c6PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2c6PllSel {
        match self.bits {
            false => ClkI2c6PllSel::B0,
            true => ClkI2c6PllSel::B1,
        }
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkI2c6PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkI2c6PllSel::B1
    }
}
#[doc = "Field `CLK_I2C6_PLL_SEL` writer - clk_i2c6 clock source select control register"]
pub type ClkI2c6PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkI2c6PllSel>;
impl<'a, REG> ClkI2c6PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2c6PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2c6PllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - i2c2 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_i2c2_div_con(&self) -> ClkI2c2DivConR {
        ClkI2c2DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - clk_i2c2 clock source select control register"]
    #[inline(always)]
    pub fn clk_i2c2_pll_sel(&self) -> ClkI2c2PllSelR {
        ClkI2c2PllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - i2c6 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_i2c6_div_con(&self) -> ClkI2c6DivConR {
        ClkI2c6DivConR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - clk_i2c6 clock source select control register"]
    #[inline(always)]
    pub fn clk_i2c6_pll_sel(&self) -> ClkI2c6PllSelR {
        ClkI2c6PllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - i2c2 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c2_div_con(&mut self) -> ClkI2c2DivConW<CruClkselCon62Spec> {
        ClkI2c2DivConW::new(self, 0)
    }
    #[doc = "Bit 7 - clk_i2c2 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c2_pll_sel(&mut self) -> ClkI2c2PllSelW<CruClkselCon62Spec> {
        ClkI2c2PllSelW::new(self, 7)
    }
    #[doc = "Bits 8:14 - i2c6 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c6_div_con(&mut self) -> ClkI2c6DivConW<CruClkselCon62Spec> {
        ClkI2c6DivConW::new(self, 8)
    }
    #[doc = "Bit 15 - clk_i2c6 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c6_pll_sel(&mut self) -> ClkI2c6PllSelW<CruClkselCon62Spec> {
        ClkI2c6PllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon62Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con62::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con62::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon62Spec;
impl crate::RegisterSpec for CruClkselCon62Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con62::R`](R) reader structure"]
impl crate::Readable for CruClkselCon62Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con62::W`](W) writer structure"]
impl crate::Writable for CruClkselCon62Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON62 to value 0x0303"]
impl crate::Resettable for CruClkselCon62Spec {
    const RESET_VALUE: u32 = 0x0303;
}
