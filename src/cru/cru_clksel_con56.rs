#[doc = "Register `CRU_CLKSEL_CON56` reader"]
pub type R = crate::R<CruClkselCon56Spec>;
#[doc = "Register `CRU_CLKSEL_CON56` writer"]
pub type W = crate::W<CruClkselCon56Spec>;
#[doc = "Field `CLK_CIF_DIV_CON` reader - clk_cif divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkCifDivConR = crate::FieldReader;
#[doc = "Field `CLK_CIF_DIV_CON` writer - clk_cif divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkCifDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_cif clock select control register\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkCifClkSel {
    #[doc = "0: clk_cif_src"]
    B0 = 0,
    #[doc = "1: xin_24m"]
    B1 = 1,
}
impl From<ClkCifClkSel> for bool {
    #[inline(always)]
    fn from(variant: ClkCifClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_CIF_CLK_SEL` reader - clk_cif clock select control register"]
pub type ClkCifClkSelR = crate::BitReader<ClkCifClkSel>;
impl ClkCifClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkCifClkSel {
        match self.bits {
            false => ClkCifClkSel::B0,
            true => ClkCifClkSel::B1,
        }
    }
    #[doc = "clk_cif_src"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkCifClkSel::B0
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkCifClkSel::B1
    }
}
#[doc = "Field `CLK_CIF_CLK_SEL` writer - clk_cif clock select control register"]
pub type ClkCifClkSelW<'a, REG> = crate::BitWriter<'a, REG, ClkCifClkSel>;
impl<'a, REG> ClkCifClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clk_cif_src"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCifClkSel::B0)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCifClkSel::B1)
    }
}
#[doc = "clk_cif clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkCifPllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B1x = 2,
}
impl From<ClkCifPllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkCifPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkCifPllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_CIF_PLL_SEL` reader - clk_cif clock source select control register"]
pub type ClkCifPllSelR = crate::FieldReader<ClkCifPllSel>;
impl ClkCifPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkCifPllSel> {
        match self.bits {
            0 => Some(ClkCifPllSel::B00),
            1 => Some(ClkCifPllSel::B01),
            2 => Some(ClkCifPllSel::B1x),
            _ => None,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkCifPllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkCifPllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkCifPllSel::B1x
    }
}
#[doc = "Field `CLK_CIF_PLL_SEL` writer - clk_cif clock source select control register"]
pub type ClkCifPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkCifPllSel>;
impl<'a, REG> ClkCifPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCifPllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCifPllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCifPllSel::B1x)
    }
}
#[doc = "Field `ACLK_GIC_DIV_CON` reader - aclk_gic divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkGicDivConR = crate::FieldReader;
#[doc = "Field `ACLK_GIC_DIV_CON` writer - aclk_gic divider control register\n\nclk=clk_src/(div_con+1)"]
pub type AclkGicDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_gic source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AclkGicPllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<AclkGicPllSel> for bool {
    #[inline(always)]
    fn from(variant: AclkGicPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACLK_GIC_PLL_SEL` reader - aclk_gic source select control register"]
pub type AclkGicPllSelR = crate::BitReader<AclkGicPllSel>;
impl AclkGicPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkGicPllSel {
        match self.bits {
            false => AclkGicPllSel::B0,
            true => AclkGicPllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AclkGicPllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AclkGicPllSel::B1
    }
}
#[doc = "Field `ACLK_GIC_PLL_SEL` writer - aclk_gic source select control register"]
pub type AclkGicPllSelW<'a, REG> = crate::BitWriter<'a, REG, AclkGicPllSel>;
impl<'a, REG> AclkGicPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AclkGicPllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AclkGicPllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - clk_cif divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_cif_div_con(&self) -> ClkCifDivConR {
        ClkCifDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - clk_cif clock select control register"]
    #[inline(always)]
    pub fn clk_cif_clk_sel(&self) -> ClkCifClkSelR {
        ClkCifClkSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - clk_cif clock source select control register"]
    #[inline(always)]
    pub fn clk_cif_pll_sel(&self) -> ClkCifPllSelR {
        ClkCifPllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - aclk_gic divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_gic_div_con(&self) -> AclkGicDivConR {
        AclkGicDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - aclk_gic source select control register"]
    #[inline(always)]
    pub fn aclk_gic_pll_sel(&self) -> AclkGicPllSelR {
        AclkGicPllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk_cif divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cif_div_con(&mut self) -> ClkCifDivConW<CruClkselCon56Spec> {
        ClkCifDivConW::new(self, 0)
    }
    #[doc = "Bit 5 - clk_cif clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cif_clk_sel(&mut self) -> ClkCifClkSelW<CruClkselCon56Spec> {
        ClkCifClkSelW::new(self, 5)
    }
    #[doc = "Bits 6:7 - clk_cif clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cif_pll_sel(&mut self) -> ClkCifPllSelW<CruClkselCon56Spec> {
        ClkCifPllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - aclk_gic divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gic_div_con(&mut self) -> AclkGicDivConW<CruClkselCon56Spec> {
        AclkGicDivConW::new(self, 8)
    }
    #[doc = "Bit 15 - aclk_gic source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gic_pll_sel(&mut self) -> AclkGicPllSelW<CruClkselCon56Spec> {
        AclkGicPllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon56Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con56::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con56::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon56Spec;
impl crate::RegisterSpec for CruClkselCon56Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con56::R`](R) reader structure"]
impl crate::Readable for CruClkselCon56Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con56::W`](W) writer structure"]
impl crate::Writable for CruClkselCon56Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON56 to value 0x0320"]
impl crate::Resettable for CruClkselCon56Spec {
    const RESET_VALUE: u32 = 0x0320;
}
