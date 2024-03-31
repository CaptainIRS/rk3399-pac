#[doc = "Register `CLKSEL_CON4` reader"]
pub type R = crate::R<ClkselCon4Spec>;
#[doc = "Register `CLKSEL_CON4` writer"]
pub type W = crate::W<ClkselCon4Spec>;
#[doc = "Field `CLK_CS_DIV_CON` reader - clk_cs divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkCsDivConR = crate::FieldReader;
#[doc = "Field `CLK_CS_DIV_CON` writer - clk_cs divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkCsDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_cs clock source select control register\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkCsPllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B1x = 2,
}
impl From<ClkCsPllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkCsPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkCsPllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_CS_PLL_SEL` reader - clk_cs clock source select control register"]
pub type ClkCsPllSelR = crate::FieldReader<ClkCsPllSel>;
impl ClkCsPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkCsPllSel> {
        match self.bits {
            0 => Some(ClkCsPllSel::B00),
            1 => Some(ClkCsPllSel::B01),
            2 => Some(ClkCsPllSel::B1x),
            _ => None,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkCsPllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkCsPllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkCsPllSel::B1x
    }
}
#[doc = "Field `CLK_CS_PLL_SEL` writer - clk_cs clock source select control register"]
pub type ClkCsPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkCsPllSel>;
impl<'a, REG> ClkCsPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCsPllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCsPllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCsPllSel::B1x)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - clk_cs divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_cs_div_con(&self) -> ClkCsDivConR {
        ClkCsDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - clk_cs clock source select control register"]
    #[inline(always)]
    pub fn clk_cs_pll_sel(&self) -> ClkCsPllSelR {
        ClkCsPllSelR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk_cs divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cs_div_con(&mut self) -> ClkCsDivConW<ClkselCon4Spec> {
        ClkCsDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - clk_cs clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cs_pll_sel(&mut self) -> ClkCsPllSelW<ClkselCon4Spec> {
        ClkCsPllSelW::new(self, 6)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon4Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon4Spec;
impl crate::RegisterSpec for ClkselCon4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con4::R`](R) reader structure"]
impl crate::Readable for ClkselCon4Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con4::W`](W) writer structure"]
impl crate::Writable for ClkselCon4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON4 to value 0x41"]
impl crate::Resettable for ClkselCon4Spec {
    const RESET_VALUE: u32 = 0x41;
}
