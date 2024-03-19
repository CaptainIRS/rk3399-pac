#[doc = "Register `CRU_CLKSEL_CON20` reader"]
pub type R = crate::R<CruClkselCon20Spec>;
#[doc = "Register `CRU_CLKSEL_CON20` writer"]
pub type W = crate::W<CruClkselCon20Spec>;
#[doc = "Field `ACLK_GMAC_DIV_CON` reader - aclk_gmac divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkGmacDivConR = crate::FieldReader;
#[doc = "Field `ACLK_GMAC_DIV_CON` writer - aclk_gmac divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkGmacDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_gmac clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AclkGmacPllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<AclkGmacPllSel> for bool {
    #[inline(always)]
    fn from(variant: AclkGmacPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACLK_GMAC_PLL_SEL` reader - aclk_gmac clock source select control register"]
pub type AclkGmacPllSelR = crate::BitReader<AclkGmacPllSel>;
impl AclkGmacPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkGmacPllSel {
        match self.bits {
            false => AclkGmacPllSel::B0,
            true => AclkGmacPllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AclkGmacPllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AclkGmacPllSel::B1
    }
}
#[doc = "Field `ACLK_GMAC_PLL_SEL` writer - aclk_gmac clock source select control register"]
pub type AclkGmacPllSelW<'a, REG> = crate::BitWriter<'a, REG, AclkGmacPllSel>;
impl<'a, REG> AclkGmacPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AclkGmacPllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AclkGmacPllSel::B1)
    }
}
#[doc = "Field `CLK_GMAC_DIV_CON` reader - clk_gmac divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkGmacDivConR = crate::FieldReader;
#[doc = "Field `CLK_GMAC_DIV_CON` writer - clk_gmac divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkGmacDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_gmac clock source select control register\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkGmacPllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B1x = 2,
}
impl From<ClkGmacPllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkGmacPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkGmacPllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_GMAC_PLL_SEL` reader - clk_gmac clock source select control register"]
pub type ClkGmacPllSelR = crate::FieldReader<ClkGmacPllSel>;
impl ClkGmacPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkGmacPllSel> {
        match self.bits {
            0 => Some(ClkGmacPllSel::B00),
            1 => Some(ClkGmacPllSel::B01),
            2 => Some(ClkGmacPllSel::B1x),
            _ => None,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkGmacPllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkGmacPllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkGmacPllSel::B1x
    }
}
#[doc = "Field `CLK_GMAC_PLL_SEL` writer - clk_gmac clock source select control register"]
pub type ClkGmacPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkGmacPllSel>;
impl<'a, REG> ClkGmacPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkGmacPllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkGmacPllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkGmacPllSel::B1x)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_gmac divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_gmac_div_con(&self) -> AclkGmacDivConR {
        AclkGmacDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - aclk_gmac clock source select control register"]
    #[inline(always)]
    pub fn aclk_gmac_pll_sel(&self) -> AclkGmacPllSelR {
        AclkGmacPllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - clk_gmac divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_gmac_div_con(&self) -> ClkGmacDivConR {
        ClkGmacDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - clk_gmac clock source select control register"]
    #[inline(always)]
    pub fn clk_gmac_pll_sel(&self) -> ClkGmacPllSelR {
        ClkGmacPllSelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_gmac divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gmac_div_con(&mut self) -> AclkGmacDivConW<CruClkselCon20Spec> {
        AclkGmacDivConW::new(self, 0)
    }
    #[doc = "Bit 7 - aclk_gmac clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gmac_pll_sel(&mut self) -> AclkGmacPllSelW<CruClkselCon20Spec> {
        AclkGmacPllSelW::new(self, 7)
    }
    #[doc = "Bits 8:12 - clk_gmac divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gmac_div_con(&mut self) -> ClkGmacDivConW<CruClkselCon20Spec> {
        ClkGmacDivConW::new(self, 8)
    }
    #[doc = "Bits 14:15 - clk_gmac clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gmac_pll_sel(&mut self) -> ClkGmacPllSelW<CruClkselCon20Spec> {
        ClkGmacPllSelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon20Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon20Spec;
impl crate::RegisterSpec for CruClkselCon20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con20::R`](R) reader structure"]
impl crate::Readable for CruClkselCon20Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con20::W`](W) writer structure"]
impl crate::Writable for CruClkselCon20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON20 to value 0x9303"]
impl crate::Resettable for CruClkselCon20Spec {
    const RESET_VALUE: u32 = 0x9303;
}
