#[doc = "Register `CRU_CLKSEL_CON54` reader"]
pub type R = crate::R<CruClkselCon54Spec>;
#[doc = "Register `CRU_CLKSEL_CON54` writer"]
pub type W = crate::W<CruClkselCon54Spec>;
#[doc = "Field `ACLK_ISP1_DIV_CON` reader - aclk_isp1 divider control register clk=clk_src/(div_con+1)"]
pub type AclkIsp1DivConR = crate::FieldReader;
#[doc = "Field `ACLK_ISP1_DIV_CON` writer - aclk_isp1 divider control register clk=clk_src/(div_con+1)"]
pub type AclkIsp1DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_isp1 clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkIsp1PllSel {
    #[doc = "0: PPLL"]
    B00 = 0,
    #[doc = "1: PPLL"]
    B01 = 1,
    #[doc = "2: PPLL"]
    B10 = 2,
}
impl From<AclkIsp1PllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkIsp1PllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkIsp1PllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_ISP1_PLL_SEL` reader - aclk_isp1 clock source select control register"]
pub type AclkIsp1PllSelR = crate::FieldReader<AclkIsp1PllSel>;
impl AclkIsp1PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AclkIsp1PllSel> {
        match self.bits {
            0 => Some(AclkIsp1PllSel::B00),
            1 => Some(AclkIsp1PllSel::B01),
            2 => Some(AclkIsp1PllSel::B10),
            _ => None,
        }
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AclkIsp1PllSel::B00
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AclkIsp1PllSel::B01
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AclkIsp1PllSel::B10
    }
}
#[doc = "Field `ACLK_ISP1_PLL_SEL` writer - aclk_isp1 clock source select control register"]
pub type AclkIsp1PllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, AclkIsp1PllSel>;
impl<'a, REG> AclkIsp1PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AclkIsp1PllSel::B00)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AclkIsp1PllSel::B01)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AclkIsp1PllSel::B10)
    }
}
#[doc = "Field `HCLK_ISP1_DIV_CON` reader - hclk_isp1 divider control register clk=clk_src/(div_con+1)"]
pub type HclkIsp1DivConR = crate::FieldReader;
#[doc = "Field `HCLK_ISP1_DIV_CON` writer - hclk_isp1 divider control register clk=clk_src/(div_con+1)"]
pub type HclkIsp1DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_isp1 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_isp1_div_con(&self) -> AclkIsp1DivConR {
        AclkIsp1DivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - aclk_isp1 clock source select control register"]
    #[inline(always)]
    pub fn aclk_isp1_pll_sel(&self) -> AclkIsp1PllSelR {
        AclkIsp1PllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - hclk_isp1 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn hclk_isp1_div_con(&self) -> HclkIsp1DivConR {
        HclkIsp1DivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_isp1 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_isp1_div_con(&mut self) -> AclkIsp1DivConW<CruClkselCon54Spec> {
        AclkIsp1DivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - aclk_isp1 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_isp1_pll_sel(&mut self) -> AclkIsp1PllSelW<CruClkselCon54Spec> {
        AclkIsp1PllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - hclk_isp1 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_isp1_div_con(&mut self) -> HclkIsp1DivConW<CruClkselCon54Spec> {
        HclkIsp1DivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon54Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con54::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con54::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon54Spec;
impl crate::RegisterSpec for CruClkselCon54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con54::R`](R) reader structure"]
impl crate::Readable for CruClkselCon54Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con54::W`](W) writer structure"]
impl crate::Writable for CruClkselCon54Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON54 to value 0x0101"]
impl crate::Resettable for CruClkselCon54Spec {
    const RESET_VALUE: u32 = 0x0101;
}
