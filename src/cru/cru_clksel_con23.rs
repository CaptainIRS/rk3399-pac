#[doc = "Register `CRU_CLKSEL_CON23` reader"]
pub type R = crate::R<CruClkselCon23Spec>;
#[doc = "Register `CRU_CLKSEL_CON23` writer"]
pub type W = crate::W<CruClkselCon23Spec>;
#[doc = "Field `ACLK_PERILP0_DIV_CON` reader - aclk_perilp0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkPerilp0DivConR = crate::FieldReader;
#[doc = "Field `ACLK_PERILP0_DIV_CON` writer - aclk_perilp0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkPerilp0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_perilp0 clock source select control register\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AclkPerilp0PllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<AclkPerilp0PllSel> for bool {
    #[inline(always)]
    fn from(variant: AclkPerilp0PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACLK_PERILP0_PLL_SEL` reader - aclk_perilp0 clock source select control register"]
pub type AclkPerilp0PllSelR = crate::BitReader<AclkPerilp0PllSel>;
impl AclkPerilp0PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkPerilp0PllSel {
        match self.bits {
            false => AclkPerilp0PllSel::B0,
            true => AclkPerilp0PllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AclkPerilp0PllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AclkPerilp0PllSel::B1
    }
}
#[doc = "Field `ACLK_PERILP0_PLL_SEL` writer - aclk_perilp0 clock source select control register"]
pub type AclkPerilp0PllSelW<'a, REG> = crate::BitWriter<'a, REG, AclkPerilp0PllSel>;
impl<'a, REG> AclkPerilp0PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AclkPerilp0PllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AclkPerilp0PllSel::B1)
    }
}
#[doc = "Field `HCLK_PERILP0_DIV_CON` reader - perilp0_hclk divider control register\n\nclk=aclk_perilp0/(div_con+1)"]
pub type HclkPerilp0DivConR = crate::FieldReader;
#[doc = "Field `HCLK_PERILP0_DIV_CON` writer - perilp0_hclk divider control register\n\nclk=aclk_perilp0/(div_con+1)"]
pub type HclkPerilp0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCLK_PERILP0_DIV_CON` reader - perilp0_pclk divider control register\n\nclk=aclk_perilp0/(div_con+1)"]
pub type PclkPerilp0DivConR = crate::FieldReader;
#[doc = "Field `PCLK_PERILP0_DIV_CON` writer - perilp0_pclk divider control register\n\nclk=aclk_perilp0/(div_con+1)"]
pub type PclkPerilp0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_perilp0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_perilp0_div_con(&self) -> AclkPerilp0DivConR {
        AclkPerilp0DivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - aclk_perilp0 clock source select control register"]
    #[inline(always)]
    pub fn aclk_perilp0_pll_sel(&self) -> AclkPerilp0PllSelR {
        AclkPerilp0PllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - perilp0_hclk divider control register\n\nclk=aclk_perilp0/(div_con+1)"]
    #[inline(always)]
    pub fn hclk_perilp0_div_con(&self) -> HclkPerilp0DivConR {
        HclkPerilp0DivConR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14 - perilp0_pclk divider control register\n\nclk=aclk_perilp0/(div_con+1)"]
    #[inline(always)]
    pub fn pclk_perilp0_div_con(&self) -> PclkPerilp0DivConR {
        PclkPerilp0DivConR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_perilp0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perilp0_div_con(&mut self) -> AclkPerilp0DivConW<CruClkselCon23Spec> {
        AclkPerilp0DivConW::new(self, 0)
    }
    #[doc = "Bit 7 - aclk_perilp0 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perilp0_pll_sel(&mut self) -> AclkPerilp0PllSelW<CruClkselCon23Spec> {
        AclkPerilp0PllSelW::new(self, 7)
    }
    #[doc = "Bits 8:9 - perilp0_hclk divider control register\n\nclk=aclk_perilp0/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_perilp0_div_con(&mut self) -> HclkPerilp0DivConW<CruClkselCon23Spec> {
        HclkPerilp0DivConW::new(self, 8)
    }
    #[doc = "Bits 12:14 - perilp0_pclk divider control register\n\nclk=aclk_perilp0/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_perilp0_div_con(&mut self) -> PclkPerilp0DivConW<CruClkselCon23Spec> {
        PclkPerilp0DivConW::new(self, 12)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon23Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon23Spec;
impl crate::RegisterSpec for CruClkselCon23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con23::R`](R) reader structure"]
impl crate::Readable for CruClkselCon23Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con23::W`](W) writer structure"]
impl crate::Writable for CruClkselCon23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON23 to value 0x3181"]
impl crate::Resettable for CruClkselCon23Spec {
    const RESET_VALUE: u32 = 0x3181;
}
