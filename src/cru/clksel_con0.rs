#[doc = "Register `CLKSEL_CON0` reader"]
pub type R = crate::R<ClkselCon0Spec>;
#[doc = "Register `CLKSEL_CON0` writer"]
pub type W = crate::W<ClkselCon0Spec>;
#[doc = "Field `CLK_CORE_L_DIV_CON` reader - clk_core_l divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkCoreLDivConR = crate::FieldReader;
#[doc = "Field `CLK_CORE_L_DIV_CON` writer - clk_core_l divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkCoreLDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_core_l clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkCoreLPllSel {
    #[doc = "0: LPLL"]
    B00 = 0,
    #[doc = "1: BPLL"]
    B01 = 1,
    #[doc = "2: DPLL"]
    B10 = 2,
    #[doc = "3: GPLL"]
    B11 = 3,
}
impl From<ClkCoreLPllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkCoreLPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkCoreLPllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_CORE_L_PLL_SEL` reader - clk_core_l clock source select control register"]
pub type ClkCoreLPllSelR = crate::FieldReader<ClkCoreLPllSel>;
impl ClkCoreLPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkCoreLPllSel {
        match self.bits {
            0 => ClkCoreLPllSel::B00,
            1 => ClkCoreLPllSel::B01,
            2 => ClkCoreLPllSel::B10,
            3 => ClkCoreLPllSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "LPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkCoreLPllSel::B00
    }
    #[doc = "BPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkCoreLPllSel::B01
    }
    #[doc = "DPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkCoreLPllSel::B10
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ClkCoreLPllSel::B11
    }
}
#[doc = "Field `CLK_CORE_L_PLL_SEL` writer - clk_core_l clock source select control register"]
pub type ClkCoreLPllSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ClkCoreLPllSel>;
impl<'a, REG> ClkCoreLPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCoreLPllSel::B00)
    }
    #[doc = "BPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCoreLPllSel::B01)
    }
    #[doc = "DPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCoreLPllSel::B10)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCoreLPllSel::B11)
    }
}
#[doc = "Field `ACLKM_CORE_L_DIV_CON` reader - aclkm_core_l divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkmCoreLDivConR = crate::FieldReader;
#[doc = "Field `ACLKM_CORE_L_DIV_CON` writer - aclkm_core_l divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkmCoreLDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - clk_core_l divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_core_l_div_con(&self) -> ClkCoreLDivConR {
        ClkCoreLDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - clk_core_l clock source select control register"]
    #[inline(always)]
    pub fn clk_core_l_pll_sel(&self) -> ClkCoreLPllSelR {
        ClkCoreLPllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - aclkm_core_l divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclkm_core_l_div_con(&self) -> AclkmCoreLDivConR {
        AclkmCoreLDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk_core_l divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_core_l_div_con(&mut self) -> ClkCoreLDivConW<ClkselCon0Spec> {
        ClkCoreLDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - clk_core_l clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_core_l_pll_sel(&mut self) -> ClkCoreLPllSelW<ClkselCon0Spec> {
        ClkCoreLPllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - aclkm_core_l divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclkm_core_l_div_con(&mut self) -> AclkmCoreLDivConW<ClkselCon0Spec> {
        AclkmCoreLDivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon0Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon0Spec;
impl crate::RegisterSpec for ClkselCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con0::R`](R) reader structure"]
impl crate::Readable for ClkselCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con0::W`](W) writer structure"]
impl crate::Writable for ClkselCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON0 to value 0x0101"]
impl crate::Resettable for ClkselCon0Spec {
    const RESET_VALUE: u32 = 0x0101;
}
