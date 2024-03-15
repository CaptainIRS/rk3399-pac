#[doc = "Register `CRU_CLKSEL_CON63` reader"]
pub type R = crate::R<CruClkselCon63Spec>;
#[doc = "Register `CRU_CLKSEL_CON63` writer"]
pub type W = crate::W<CruClkselCon63Spec>;
#[doc = "Field `CLK_I2C3_DIV_CON` reader - i2c3 divider control register clk=clk_src/(div_con+1)"]
pub type ClkI2c3DivConR = crate::FieldReader;
#[doc = "Field `CLK_I2C3_DIV_CON` writer - i2c3 divider control register clk=clk_src/(div_con+1)"]
pub type ClkI2c3DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_i2c3 clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkI2c3PllSel {
    #[doc = "0: GPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkI2c3PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkI2c3PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_I2C3_PLL_SEL` reader - clk_i2c3 clock source select control register"]
pub type ClkI2c3PllSelR = crate::BitReader<ClkI2c3PllSel>;
impl ClkI2c3PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2c3PllSel {
        match self.bits {
            false => ClkI2c3PllSel::B0,
            true => ClkI2c3PllSel::B1,
        }
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkI2c3PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkI2c3PllSel::B1
    }
}
#[doc = "Field `CLK_I2C3_PLL_SEL` writer - clk_i2c3 clock source select control register"]
pub type ClkI2c3PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkI2c3PllSel>;
impl<'a, REG> ClkI2c3PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2c3PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2c3PllSel::B1)
    }
}
#[doc = "Field `CLK_I2C7_DIV_CON` reader - i2c7 divider control register clk=clk_src/(div_con+1)"]
pub type ClkI2c7DivConR = crate::FieldReader;
#[doc = "Field `CLK_I2C7_DIV_CON` writer - i2c7 divider control register clk=clk_src/(div_con+1)"]
pub type ClkI2c7DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_i2c7 clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkI2c7PllSel {
    #[doc = "0: GPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkI2c7PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkI2c7PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_I2C7_PLL_SEL` reader - clk_i2c7 clock source select control register"]
pub type ClkI2c7PllSelR = crate::BitReader<ClkI2c7PllSel>;
impl ClkI2c7PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2c7PllSel {
        match self.bits {
            false => ClkI2c7PllSel::B0,
            true => ClkI2c7PllSel::B1,
        }
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkI2c7PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkI2c7PllSel::B1
    }
}
#[doc = "Field `CLK_I2C7_PLL_SEL` writer - clk_i2c7 clock source select control register"]
pub type ClkI2c7PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkI2c7PllSel>;
impl<'a, REG> ClkI2c7PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2c7PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2c7PllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - i2c3 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_i2c3_div_con(&self) -> ClkI2c3DivConR {
        ClkI2c3DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - clk_i2c3 clock source select control register"]
    #[inline(always)]
    pub fn clk_i2c3_pll_sel(&self) -> ClkI2c3PllSelR {
        ClkI2c3PllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - i2c7 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_i2c7_div_con(&self) -> ClkI2c7DivConR {
        ClkI2c7DivConR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - clk_i2c7 clock source select control register"]
    #[inline(always)]
    pub fn clk_i2c7_pll_sel(&self) -> ClkI2c7PllSelR {
        ClkI2c7PllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - i2c3 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c3_div_con(&mut self) -> ClkI2c3DivConW<CruClkselCon63Spec> {
        ClkI2c3DivConW::new(self, 0)
    }
    #[doc = "Bit 7 - clk_i2c3 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c3_pll_sel(&mut self) -> ClkI2c3PllSelW<CruClkselCon63Spec> {
        ClkI2c3PllSelW::new(self, 7)
    }
    #[doc = "Bits 8:14 - i2c7 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c7_div_con(&mut self) -> ClkI2c7DivConW<CruClkselCon63Spec> {
        ClkI2c7DivConW::new(self, 8)
    }
    #[doc = "Bit 15 - clk_i2c7 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c7_pll_sel(&mut self) -> ClkI2c7PllSelW<CruClkselCon63Spec> {
        ClkI2c7PllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon63Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con63::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con63::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon63Spec;
impl crate::RegisterSpec for CruClkselCon63Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con63::R`](R) reader structure"]
impl crate::Readable for CruClkselCon63Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con63::W`](W) writer structure"]
impl crate::Writable for CruClkselCon63Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON63 to value 0x0303"]
impl crate::Resettable for CruClkselCon63Spec {
    const RESET_VALUE: u32 = 0x0303;
}
