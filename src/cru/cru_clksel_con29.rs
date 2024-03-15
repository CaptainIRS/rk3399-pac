#[doc = "Register `CRU_CLKSEL_CON29` reader"]
pub type R = crate::R<CruClkselCon29Spec>;
#[doc = "Register `CRU_CLKSEL_CON29` writer"]
pub type W = crate::W<CruClkselCon29Spec>;
#[doc = "Field `CLK_I2S1_DIV_CON` reader - clk_i2s1 divider control register clk=clk_src/(div_con+1)"]
pub type ClkI2s1DivConR = crate::FieldReader;
#[doc = "Field `CLK_I2S1_DIV_CON` writer - clk_i2s1 divider control register clk=clk_src/(div_con+1)"]
pub type ClkI2s1DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_i2s1 clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkI2s1PllSel {
    #[doc = "0: GPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkI2s1PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkI2s1PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_I2S1_PLL_SEL` reader - clk_i2s1 clock source select control register"]
pub type ClkI2s1PllSelR = crate::BitReader<ClkI2s1PllSel>;
impl ClkI2s1PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2s1PllSel {
        match self.bits {
            false => ClkI2s1PllSel::B0,
            true => ClkI2s1PllSel::B1,
        }
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkI2s1PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkI2s1PllSel::B1
    }
}
#[doc = "Field `CLK_I2S1_PLL_SEL` writer - clk_i2s1 clock source select control register"]
pub type ClkI2s1PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkI2s1PllSel>;
impl<'a, REG> ClkI2s1PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s1PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s1PllSel::B1)
    }
}
#[doc = "clk_i2s1 clock select control register\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkI2s1Sel {
    #[doc = "0: clk_12m"]
    B00 = 0,
    #[doc = "1: clk_12m"]
    B01 = 1,
    #[doc = "2: clk_12m"]
    B10 = 2,
    #[doc = "3: clk_12m"]
    B11 = 3,
}
impl From<ClkI2s1Sel> for u8 {
    #[inline(always)]
    fn from(variant: ClkI2s1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkI2s1Sel {
    type Ux = u8;
}
#[doc = "Field `CLK_I2S1_SEL` reader - clk_i2s1 clock select control register"]
pub type ClkI2s1SelR = crate::FieldReader<ClkI2s1Sel>;
impl ClkI2s1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2s1Sel {
        match self.bits {
            0 => ClkI2s1Sel::B00,
            1 => ClkI2s1Sel::B01,
            2 => ClkI2s1Sel::B10,
            3 => ClkI2s1Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkI2s1Sel::B00
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkI2s1Sel::B01
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkI2s1Sel::B10
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ClkI2s1Sel::B11
    }
}
#[doc = "Field `CLK_I2S1_SEL` writer - clk_i2s1 clock select control register"]
pub type ClkI2s1SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ClkI2s1Sel>;
impl<'a, REG> ClkI2s1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s1Sel::B00)
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s1Sel::B01)
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s1Sel::B10)
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s1Sel::B11)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_i2s1 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_i2s1_div_con(&self) -> ClkI2s1DivConR {
        ClkI2s1DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - clk_i2s1 clock source select control register"]
    #[inline(always)]
    pub fn clk_i2s1_pll_sel(&self) -> ClkI2s1PllSelR {
        ClkI2s1PllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - clk_i2s1 clock select control register"]
    #[inline(always)]
    pub fn clk_i2s1_sel(&self) -> ClkI2s1SelR {
        ClkI2s1SelR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_i2s1 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s1_div_con(&mut self) -> ClkI2s1DivConW<CruClkselCon29Spec> {
        ClkI2s1DivConW::new(self, 0)
    }
    #[doc = "Bit 7 - clk_i2s1 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s1_pll_sel(&mut self) -> ClkI2s1PllSelW<CruClkselCon29Spec> {
        ClkI2s1PllSelW::new(self, 7)
    }
    #[doc = "Bits 8:9 - clk_i2s1 clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s1_sel(&mut self) -> ClkI2s1SelW<CruClkselCon29Spec> {
        ClkI2s1SelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon29Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon29Spec;
impl crate::RegisterSpec for CruClkselCon29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con29::R`](R) reader structure"]
impl crate::Readable for CruClkselCon29Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con29::W`](W) writer structure"]
impl crate::Writable for CruClkselCon29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON29 to value 0x0300"]
impl crate::Resettable for CruClkselCon29Spec {
    const RESET_VALUE: u32 = 0x0300;
}
