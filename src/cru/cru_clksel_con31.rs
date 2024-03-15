#[doc = "Register `CRU_CLKSEL_CON31` reader"]
pub type R = crate::R<CruClkselCon31Spec>;
#[doc = "Register `CRU_CLKSEL_CON31` writer"]
pub type W = crate::W<CruClkselCon31Spec>;
#[doc = "clk_i2s_ch clock select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkI2sChSel {
    #[doc = "0: clk_i2s2"]
    B00 = 0,
    #[doc = "1: clk_i2s2"]
    B01 = 1,
    #[doc = "2: clk_i2s2"]
    B10 = 2,
}
impl From<ClkI2sChSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkI2sChSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkI2sChSel {
    type Ux = u8;
}
#[doc = "Field `CLK_I2S_CH_SEL` reader - clk_i2s_ch clock select control register"]
pub type ClkI2sChSelR = crate::FieldReader<ClkI2sChSel>;
impl ClkI2sChSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkI2sChSel> {
        match self.bits {
            0 => Some(ClkI2sChSel::B00),
            1 => Some(ClkI2sChSel::B01),
            2 => Some(ClkI2sChSel::B10),
            _ => None,
        }
    }
    #[doc = "clk_i2s2"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkI2sChSel::B00
    }
    #[doc = "clk_i2s2"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkI2sChSel::B01
    }
    #[doc = "clk_i2s2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkI2sChSel::B10
    }
}
#[doc = "Field `CLK_I2S_CH_SEL` writer - clk_i2s_ch clock select control register"]
pub type ClkI2sChSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkI2sChSel>;
impl<'a, REG> ClkI2sChSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clk_i2s2"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2sChSel::B00)
    }
    #[doc = "clk_i2s2"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2sChSel::B01)
    }
    #[doc = "clk_i2s2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2sChSel::B10)
    }
}
#[doc = "clk_i2sout clock select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkI2soutSel {
    #[doc = "0: clk_12m"]
    B0 = 0,
    #[doc = "1: clk_12m"]
    B1 = 1,
}
impl From<ClkI2soutSel> for bool {
    #[inline(always)]
    fn from(variant: ClkI2soutSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_I2SOUT_SEL` reader - clk_i2sout clock select control register"]
pub type ClkI2soutSelR = crate::BitReader<ClkI2soutSel>;
impl ClkI2soutSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkI2soutSel {
        match self.bits {
            false => ClkI2soutSel::B0,
            true => ClkI2soutSel::B1,
        }
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkI2soutSel::B0
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkI2soutSel::B1
    }
}
#[doc = "Field `CLK_I2SOUT_SEL` writer - clk_i2sout clock select control register"]
pub type ClkI2soutSelW<'a, REG> = crate::BitWriter<'a, REG, ClkI2soutSel>;
impl<'a, REG> ClkI2soutSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2soutSel::B0)
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkI2soutSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - clk_i2s_ch clock select control register"]
    #[inline(always)]
    pub fn clk_i2s_ch_sel(&self) -> ClkI2sChSelR {
        ClkI2sChSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - clk_i2sout clock select control register"]
    #[inline(always)]
    pub fn clk_i2sout_sel(&self) -> ClkI2soutSelR {
        ClkI2soutSelR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - clk_i2s_ch clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s_ch_sel(&mut self) -> ClkI2sChSelW<CruClkselCon31Spec> {
        ClkI2sChSelW::new(self, 0)
    }
    #[doc = "Bit 2 - clk_i2sout clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2sout_sel(&mut self) -> ClkI2soutSelW<CruClkselCon31Spec> {
        ClkI2soutSelW::new(self, 2)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon31Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon31Spec;
impl crate::RegisterSpec for CruClkselCon31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con31::R`](R) reader structure"]
impl crate::Readable for CruClkselCon31Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con31::W`](W) writer structure"]
impl crate::Writable for CruClkselCon31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON31 to value 0"]
impl crate::Resettable for CruClkselCon31Spec {
    const RESET_VALUE: u32 = 0;
}
