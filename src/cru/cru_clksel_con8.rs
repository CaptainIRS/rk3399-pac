#[doc = "Register `CRU_CLKSEL_CON8` reader"]
pub type R = crate::R<CruClkselCon8Spec>;
#[doc = "Register `CRU_CLKSEL_CON8` writer"]
pub type W = crate::W<CruClkselCon8Spec>;
#[doc = "Field `ACLK_VDU_DIV_CON` reader - aclk_vdu divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkVduDivConR = crate::FieldReader;
#[doc = "Field `ACLK_VDU_DIV_CON` writer - aclk_vdu divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkVduDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_vdu clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkVduPllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B10 = 2,
    #[doc = "3: PPLL"]
    B11 = 3,
}
impl From<AclkVduPllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkVduPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkVduPllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_VDU_PLL_SEL` reader - aclk_vdu clock source select control register"]
pub type AclkVduPllSelR = crate::FieldReader<AclkVduPllSel>;
impl AclkVduPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkVduPllSel {
        match self.bits {
            0 => AclkVduPllSel::B00,
            1 => AclkVduPllSel::B01,
            2 => AclkVduPllSel::B10,
            3 => AclkVduPllSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AclkVduPllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AclkVduPllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AclkVduPllSel::B10
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == AclkVduPllSel::B11
    }
}
#[doc = "Field `ACLK_VDU_PLL_SEL` writer - aclk_vdu clock source select control register"]
pub type AclkVduPllSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AclkVduPllSel>;
impl<'a, REG> AclkVduPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVduPllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVduPllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVduPllSel::B10)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(AclkVduPllSel::B11)
    }
}
#[doc = "Field `HCLK_VDU_DIV_CON` reader - hclk_vdu divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkVduDivConR = crate::FieldReader;
#[doc = "Field `HCLK_VDU_DIV_CON` writer - hclk_vdu divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkVduDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_vdu divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_vdu_div_con(&self) -> AclkVduDivConR {
        AclkVduDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - aclk_vdu clock source select control register"]
    #[inline(always)]
    pub fn aclk_vdu_pll_sel(&self) -> AclkVduPllSelR {
        AclkVduPllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - hclk_vdu divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn hclk_vdu_div_con(&self) -> HclkVduDivConR {
        HclkVduDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_vdu divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vdu_div_con(&mut self) -> AclkVduDivConW<CruClkselCon8Spec> {
        AclkVduDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - aclk_vdu clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vdu_pll_sel(&mut self) -> AclkVduPllSelW<CruClkselCon8Spec> {
        AclkVduPllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - hclk_vdu divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vdu_div_con(&mut self) -> HclkVduDivConW<CruClkselCon8Spec> {
        HclkVduDivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon8Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon8Spec;
impl crate::RegisterSpec for CruClkselCon8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con8::R`](R) reader structure"]
impl crate::Readable for CruClkselCon8Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con8::W`](W) writer structure"]
impl crate::Writable for CruClkselCon8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON8 to value 0x0101"]
impl crate::Resettable for CruClkselCon8Spec {
    const RESET_VALUE: u32 = 0x0101;
}
