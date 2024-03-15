#[doc = "Register `CRU_CLKSEL_CON55` reader"]
pub type R = crate::R<CruClkselCon55Spec>;
#[doc = "Register `CRU_CLKSEL_CON55` writer"]
pub type W = crate::W<CruClkselCon55Spec>;
#[doc = "Field `CLK_ISP0_DIV_CON` reader - clk_isp0 divider control register clk=clk_src/(div_con+1)"]
pub type ClkIsp0DivConR = crate::FieldReader;
#[doc = "Field `CLK_ISP0_DIV_CON` writer - clk_isp0 divider control register clk=clk_src/(div_con+1)"]
pub type ClkIsp0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_isp0 clock source select control register\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkIsp0PllSel {
    #[doc = "0: NPLL"]
    B00 = 0,
    #[doc = "1: NPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B1x = 2,
}
impl From<ClkIsp0PllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkIsp0PllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkIsp0PllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_ISP0_PLL_SEL` reader - clk_isp0 clock source select control register"]
pub type ClkIsp0PllSelR = crate::FieldReader<ClkIsp0PllSel>;
impl ClkIsp0PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkIsp0PllSel> {
        match self.bits {
            0 => Some(ClkIsp0PllSel::B00),
            1 => Some(ClkIsp0PllSel::B01),
            2 => Some(ClkIsp0PllSel::B1x),
            _ => None,
        }
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkIsp0PllSel::B00
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkIsp0PllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkIsp0PllSel::B1x
    }
}
#[doc = "Field `CLK_ISP0_PLL_SEL` writer - clk_isp0 clock source select control register"]
pub type ClkIsp0PllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkIsp0PllSel>;
impl<'a, REG> ClkIsp0PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkIsp0PllSel::B00)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkIsp0PllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkIsp0PllSel::B1x)
    }
}
#[doc = "Field `CLK_ISP1_DIV_CON` reader - clk_isp1 divider control register clk=clk_src/(div_con+1)"]
pub type ClkIsp1DivConR = crate::FieldReader;
#[doc = "Field `CLK_ISP1_DIV_CON` writer - clk_isp1 divider control register clk=clk_src/(div_con+1)"]
pub type ClkIsp1DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_isp1 clock source select control register\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkIsp1PllSel {
    #[doc = "0: NPLL"]
    B00 = 0,
    #[doc = "1: NPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B1x = 2,
}
impl From<ClkIsp1PllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkIsp1PllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkIsp1PllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_ISP1_PLL_SEL` reader - clk_isp1 clock source select control register"]
pub type ClkIsp1PllSelR = crate::FieldReader<ClkIsp1PllSel>;
impl ClkIsp1PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkIsp1PllSel> {
        match self.bits {
            0 => Some(ClkIsp1PllSel::B00),
            1 => Some(ClkIsp1PllSel::B01),
            2 => Some(ClkIsp1PllSel::B1x),
            _ => None,
        }
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkIsp1PllSel::B00
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkIsp1PllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkIsp1PllSel::B1x
    }
}
#[doc = "Field `CLK_ISP1_PLL_SEL` writer - clk_isp1 clock source select control register"]
pub type ClkIsp1PllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkIsp1PllSel>;
impl<'a, REG> ClkIsp1PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkIsp1PllSel::B00)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkIsp1PllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkIsp1PllSel::B1x)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - clk_isp0 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_isp0_div_con(&self) -> ClkIsp0DivConR {
        ClkIsp0DivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - clk_isp0 clock source select control register"]
    #[inline(always)]
    pub fn clk_isp0_pll_sel(&self) -> ClkIsp0PllSelR {
        ClkIsp0PllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - clk_isp1 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_isp1_div_con(&self) -> ClkIsp1DivConR {
        ClkIsp1DivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - clk_isp1 clock source select control register"]
    #[inline(always)]
    pub fn clk_isp1_pll_sel(&self) -> ClkIsp1PllSelR {
        ClkIsp1PllSelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk_isp0 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_isp0_div_con(&mut self) -> ClkIsp0DivConW<CruClkselCon55Spec> {
        ClkIsp0DivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - clk_isp0 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_isp0_pll_sel(&mut self) -> ClkIsp0PllSelW<CruClkselCon55Spec> {
        ClkIsp0PllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - clk_isp1 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_isp1_div_con(&mut self) -> ClkIsp1DivConW<CruClkselCon55Spec> {
        ClkIsp1DivConW::new(self, 8)
    }
    #[doc = "Bits 14:15 - clk_isp1 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_isp1_pll_sel(&mut self) -> ClkIsp1PllSelW<CruClkselCon55Spec> {
        ClkIsp1PllSelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon55Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con55::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con55::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon55Spec;
impl crate::RegisterSpec for CruClkselCon55Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con55::R`](R) reader structure"]
impl crate::Readable for CruClkselCon55Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con55::W`](W) writer structure"]
impl crate::Writable for CruClkselCon55Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON55 to value 0x8181"]
impl crate::Resettable for CruClkselCon55Spec {
    const RESET_VALUE: u32 = 0x8181;
}
