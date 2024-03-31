#[doc = "Register `CLKSEL_CON28` reader"]
pub type R = crate::R<ClkselCon28Spec>;
#[doc = "Register `CLKSEL_CON28` writer"]
pub type W = crate::W<ClkselCon28Spec>;
#[doc = "Field `CLK_I2S0_DIV_CON` reader - clk_i2s0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkI2s0DivConR = crate::FieldReader;
#[doc = "Field `CLK_I2S0_DIV_CON` writer - clk_i2s0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkI2s0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_i2s0 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkI2s0PllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkI2s0PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkI2s0PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_I2S0_PLL_SEL` reader - clk_i2s0 clock source select control register"]
pub type ClkI2s0PllSelR = crate::BitReader<ClkI2s0PllSel>;
impl ClkI2s0PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2s0PllSel {
        match self.bits {
            false => ClkI2s0PllSel::B0,
            true => ClkI2s0PllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkI2s0PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkI2s0PllSel::B1
    }
}
#[doc = "Field `CLK_I2S0_PLL_SEL` writer - clk_i2s0 clock source select control register"]
pub type ClkI2s0PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkI2s0PllSel>;
impl<'a, REG> ClkI2s0PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s0PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s0PllSel::B1)
    }
}
#[doc = "clk_i2s0 clock select control register\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkI2s0Sel {
    #[doc = "0: clk_i2s0_divout"]
    B00 = 0,
    #[doc = "1: clk_i2s0_frac"]
    B01 = 1,
    #[doc = "2: clkin_i2s from IO"]
    B10 = 2,
    #[doc = "3: clk_12m"]
    B11 = 3,
}
impl From<ClkI2s0Sel> for u8 {
    #[inline(always)]
    fn from(variant: ClkI2s0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkI2s0Sel {
    type Ux = u8;
}
#[doc = "Field `CLK_I2S0_SEL` reader - clk_i2s0 clock select control register"]
pub type ClkI2s0SelR = crate::FieldReader<ClkI2s0Sel>;
impl ClkI2s0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2s0Sel {
        match self.bits {
            0 => ClkI2s0Sel::B00,
            1 => ClkI2s0Sel::B01,
            2 => ClkI2s0Sel::B10,
            3 => ClkI2s0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "clk_i2s0_divout"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkI2s0Sel::B00
    }
    #[doc = "clk_i2s0_frac"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkI2s0Sel::B01
    }
    #[doc = "clkin_i2s from IO"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkI2s0Sel::B10
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ClkI2s0Sel::B11
    }
}
#[doc = "Field `CLK_I2S0_SEL` writer - clk_i2s0 clock select control register"]
pub type ClkI2s0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ClkI2s0Sel>;
impl<'a, REG> ClkI2s0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clk_i2s0_divout"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s0Sel::B00)
    }
    #[doc = "clk_i2s0_frac"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s0Sel::B01)
    }
    #[doc = "clkin_i2s from IO"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s0Sel::B10)
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2s0Sel::B11)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_i2s0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_i2s0_div_con(&self) -> ClkI2s0DivConR {
        ClkI2s0DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - clk_i2s0 clock source select control register"]
    #[inline(always)]
    pub fn clk_i2s0_pll_sel(&self) -> ClkI2s0PllSelR {
        ClkI2s0PllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - clk_i2s0 clock select control register"]
    #[inline(always)]
    pub fn clk_i2s0_sel(&self) -> ClkI2s0SelR {
        ClkI2s0SelR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_i2s0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s0_div_con(&mut self) -> ClkI2s0DivConW<ClkselCon28Spec> {
        ClkI2s0DivConW::new(self, 0)
    }
    #[doc = "Bit 7 - clk_i2s0 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s0_pll_sel(&mut self) -> ClkI2s0PllSelW<ClkselCon28Spec> {
        ClkI2s0PllSelW::new(self, 7)
    }
    #[doc = "Bits 8:9 - clk_i2s0 clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s0_sel(&mut self) -> ClkI2s0SelW<ClkselCon28Spec> {
        ClkI2s0SelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon28Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon28Spec;
impl crate::RegisterSpec for ClkselCon28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con28::R`](R) reader structure"]
impl crate::Readable for ClkselCon28Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con28::W`](W) writer structure"]
impl crate::Writable for ClkselCon28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON28 to value 0x0300"]
impl crate::Resettable for ClkselCon28Spec {
    const RESET_VALUE: u32 = 0x0300;
}
