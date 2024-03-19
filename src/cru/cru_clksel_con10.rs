#[doc = "Register `CRU_CLKSEL_CON10` reader"]
pub type R = crate::R<CruClkselCon10Spec>;
#[doc = "Register `CRU_CLKSEL_CON10` writer"]
pub type W = crate::W<CruClkselCon10Spec>;
#[doc = "Field `ACLK_IEP_DIV_CON` reader - aclk_iep divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkIepDivConR = crate::FieldReader;
#[doc = "Field `ACLK_IEP_DIV_CON` writer - aclk_iep divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkIepDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_iep clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkIepPllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B10 = 2,
    #[doc = "3: PPLL"]
    B11 = 3,
}
impl From<AclkIepPllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkIepPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkIepPllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_IEP_PLL_SEL` reader - aclk_iep clock source select control register"]
pub type AclkIepPllSelR = crate::FieldReader<AclkIepPllSel>;
impl AclkIepPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkIepPllSel {
        match self.bits {
            0 => AclkIepPllSel::B00,
            1 => AclkIepPllSel::B01,
            2 => AclkIepPllSel::B10,
            3 => AclkIepPllSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AclkIepPllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AclkIepPllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AclkIepPllSel::B10
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == AclkIepPllSel::B11
    }
}
#[doc = "Field `ACLK_IEP_PLL_SEL` writer - aclk_iep clock source select control register"]
pub type AclkIepPllSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AclkIepPllSel>;
impl<'a, REG> AclkIepPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AclkIepPllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AclkIepPllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AclkIepPllSel::B10)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(AclkIepPllSel::B11)
    }
}
#[doc = "Field `HCLK_IEP_DIV_CON` reader - hclk_iep divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkIepDivConR = crate::FieldReader;
#[doc = "Field `HCLK_IEP_DIV_CON` writer - hclk_iep divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkIepDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_iep divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_iep_div_con(&self) -> AclkIepDivConR {
        AclkIepDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - aclk_iep clock source select control register"]
    #[inline(always)]
    pub fn aclk_iep_pll_sel(&self) -> AclkIepPllSelR {
        AclkIepPllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - hclk_iep divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn hclk_iep_div_con(&self) -> HclkIepDivConR {
        HclkIepDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_iep divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_iep_div_con(&mut self) -> AclkIepDivConW<CruClkselCon10Spec> {
        AclkIepDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - aclk_iep clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_iep_pll_sel(&mut self) -> AclkIepPllSelW<CruClkselCon10Spec> {
        AclkIepPllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - hclk_iep divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_iep_div_con(&mut self) -> HclkIepDivConW<CruClkselCon10Spec> {
        HclkIepDivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon10Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon10Spec;
impl crate::RegisterSpec for CruClkselCon10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con10::R`](R) reader structure"]
impl crate::Readable for CruClkselCon10Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con10::W`](W) writer structure"]
impl crate::Writable for CruClkselCon10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON10 to value 0x0101"]
impl crate::Resettable for CruClkselCon10Spec {
    const RESET_VALUE: u32 = 0x0101;
}
