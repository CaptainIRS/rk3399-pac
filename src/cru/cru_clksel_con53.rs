#[doc = "Register `CRU_CLKSEL_CON53` reader"]
pub type R = crate::R<CruClkselCon53Spec>;
#[doc = "Register `CRU_CLKSEL_CON53` writer"]
pub type W = crate::W<CruClkselCon53Spec>;
#[doc = "Field `ACLK_ISP0_DIV_CON` reader - aclk_isp0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkIsp0DivConR = crate::FieldReader;
#[doc = "Field `ACLK_ISP0_DIV_CON` writer - aclk_isp0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkIsp0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_isp0 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkIsp0PllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: PPLL"]
    B10 = 2,
}
impl From<AclkIsp0PllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkIsp0PllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkIsp0PllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_ISP0_PLL_SEL` reader - aclk_isp0 clock source select control register"]
pub type AclkIsp0PllSelR = crate::FieldReader<AclkIsp0PllSel>;
impl AclkIsp0PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AclkIsp0PllSel> {
        match self.bits {
            0 => Some(AclkIsp0PllSel::B00),
            1 => Some(AclkIsp0PllSel::B01),
            2 => Some(AclkIsp0PllSel::B10),
            _ => None,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AclkIsp0PllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AclkIsp0PllSel::B01
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AclkIsp0PllSel::B10
    }
}
#[doc = "Field `ACLK_ISP0_PLL_SEL` writer - aclk_isp0 clock source select control register"]
pub type AclkIsp0PllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, AclkIsp0PllSel>;
impl<'a, REG> AclkIsp0PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AclkIsp0PllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AclkIsp0PllSel::B01)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AclkIsp0PllSel::B10)
    }
}
#[doc = "Field `HCLK_ISP0_DIV_CON` reader - hclk_isp0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkIsp0DivConR = crate::FieldReader;
#[doc = "Field `HCLK_ISP0_DIV_CON` writer - hclk_isp0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type HclkIsp0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_isp0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_isp0_div_con(&self) -> AclkIsp0DivConR {
        AclkIsp0DivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - aclk_isp0 clock source select control register"]
    #[inline(always)]
    pub fn aclk_isp0_pll_sel(&self) -> AclkIsp0PllSelR {
        AclkIsp0PllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - hclk_isp0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn hclk_isp0_div_con(&self) -> HclkIsp0DivConR {
        HclkIsp0DivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_isp0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_isp0_div_con(&mut self) -> AclkIsp0DivConW<CruClkselCon53Spec> {
        AclkIsp0DivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - aclk_isp0 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_isp0_pll_sel(&mut self) -> AclkIsp0PllSelW<CruClkselCon53Spec> {
        AclkIsp0PllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - hclk_isp0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_isp0_div_con(&mut self) -> HclkIsp0DivConW<CruClkselCon53Spec> {
        HclkIsp0DivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon53Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register53\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con53::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con53::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon53Spec;
impl crate::RegisterSpec for CruClkselCon53Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con53::R`](R) reader structure"]
impl crate::Readable for CruClkselCon53Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con53::W`](W) writer structure"]
impl crate::Writable for CruClkselCon53Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON53 to value 0x0101"]
impl crate::Resettable for CruClkselCon53Spec {
    const RESET_VALUE: u32 = 0x0101;
}
