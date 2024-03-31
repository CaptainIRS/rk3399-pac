#[doc = "Register `CLKSEL_CON18` reader"]
pub type R = crate::R<ClkselCon18Spec>;
#[doc = "Register `CLKSEL_CON18` writer"]
pub type W = crate::W<ClkselCon18Spec>;
#[doc = "Field `CLK_PCIE_CORE_DIV_CON` reader - clk_pcie_core divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkPcieCoreDivConR = crate::FieldReader;
#[doc = "Field `CLK_PCIE_CORE_DIV_CON` writer - clk_pcie_core divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkPcieCoreDivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_pcie_core clock select control register\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkPcieCoreClkSel {
    #[doc = "0: clk_pcie_core"]
    B0 = 0,
    #[doc = "1: pipe_clk_pcie from PCIE PHY"]
    B1 = 1,
}
impl From<ClkPcieCoreClkSel> for bool {
    #[inline(always)]
    fn from(variant: ClkPcieCoreClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_PCIE_CORE_CLK_SEL` reader - clk_pcie_core clock select control register"]
pub type ClkPcieCoreClkSelR = crate::BitReader<ClkPcieCoreClkSel>;
impl ClkPcieCoreClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkPcieCoreClkSel {
        match self.bits {
            false => ClkPcieCoreClkSel::B0,
            true => ClkPcieCoreClkSel::B1,
        }
    }
    #[doc = "clk_pcie_core"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkPcieCoreClkSel::B0
    }
    #[doc = "pipe_clk_pcie from PCIE PHY"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkPcieCoreClkSel::B1
    }
}
#[doc = "Field `CLK_PCIE_CORE_CLK_SEL` writer - clk_pcie_core clock select control register"]
pub type ClkPcieCoreClkSelW<'a, REG> = crate::BitWriter<'a, REG, ClkPcieCoreClkSel>;
impl<'a, REG> ClkPcieCoreClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clk_pcie_core"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPcieCoreClkSel::B0)
    }
    #[doc = "pipe_clk_pcie from PCIE PHY"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPcieCoreClkSel::B1)
    }
}
#[doc = "clk_pcie_core clock source select control register\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkPcieCorePllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B1x = 2,
}
impl From<ClkPcieCorePllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkPcieCorePllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkPcieCorePllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_PCIE_CORE_PLL_SEL` reader - clk_pcie_core clock source select control register"]
pub type ClkPcieCorePllSelR = crate::FieldReader<ClkPcieCorePllSel>;
impl ClkPcieCorePllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkPcieCorePllSel> {
        match self.bits {
            0 => Some(ClkPcieCorePllSel::B00),
            1 => Some(ClkPcieCorePllSel::B01),
            2 => Some(ClkPcieCorePllSel::B1x),
            _ => None,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkPcieCorePllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkPcieCorePllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkPcieCorePllSel::B1x
    }
}
#[doc = "Field `CLK_PCIE_CORE_PLL_SEL` writer - clk_pcie_core clock source select control register"]
pub type ClkPcieCorePllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkPcieCorePllSel>;
impl<'a, REG> ClkPcieCorePllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPcieCorePllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPcieCorePllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPcieCorePllSel::B1x)
    }
}
#[doc = "clk_pciephy_ref clock select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkPciephyRefSel {
    #[doc = "0: clk_pcie_ref24m"]
    B0 = 0,
    #[doc = "1: clk_pcie_ref100m"]
    B1 = 1,
}
impl From<ClkPciephyRefSel> for bool {
    #[inline(always)]
    fn from(variant: ClkPciephyRefSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_PCIEPHY_REF_SEL` reader - clk_pciephy_ref clock select control register"]
pub type ClkPciephyRefSelR = crate::BitReader<ClkPciephyRefSel>;
impl ClkPciephyRefSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkPciephyRefSel {
        match self.bits {
            false => ClkPciephyRefSel::B0,
            true => ClkPciephyRefSel::B1,
        }
    }
    #[doc = "clk_pcie_ref24m"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkPciephyRefSel::B0
    }
    #[doc = "clk_pcie_ref100m"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkPciephyRefSel::B1
    }
}
#[doc = "Field `CLK_PCIEPHY_REF_SEL` writer - clk_pciephy_ref clock select control register"]
pub type ClkPciephyRefSelW<'a, REG> = crate::BitWriter<'a, REG, ClkPciephyRefSel>;
impl<'a, REG> ClkPciephyRefSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clk_pcie_ref24m"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPciephyRefSel::B0)
    }
    #[doc = "clk_pcie_ref100m"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPciephyRefSel::B1)
    }
}
#[doc = "Field `CLK_PCIEPHY_REF100M_DIV_CON` reader - clk_pciephy_ref100m divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkPciephyRef100mDivConR = crate::FieldReader;
#[doc = "Field `CLK_PCIEPHY_REF100M_DIV_CON` writer - clk_pciephy_ref100m divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkPciephyRef100mDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_pcie_core divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_pcie_core_div_con(&self) -> ClkPcieCoreDivConR {
        ClkPcieCoreDivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - clk_pcie_core clock select control register"]
    #[inline(always)]
    pub fn clk_pcie_core_clk_sel(&self) -> ClkPcieCoreClkSelR {
        ClkPcieCoreClkSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - clk_pcie_core clock source select control register"]
    #[inline(always)]
    pub fn clk_pcie_core_pll_sel(&self) -> ClkPcieCorePllSelR {
        ClkPcieCorePllSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - clk_pciephy_ref clock select control register"]
    #[inline(always)]
    pub fn clk_pciephy_ref_sel(&self) -> ClkPciephyRefSelR {
        ClkPciephyRefSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - clk_pciephy_ref100m divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_pciephy_ref100m_div_con(&self) -> ClkPciephyRef100mDivConR {
        ClkPciephyRef100mDivConR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_pcie_core divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pcie_core_div_con(&mut self) -> ClkPcieCoreDivConW<ClkselCon18Spec> {
        ClkPcieCoreDivConW::new(self, 0)
    }
    #[doc = "Bit 7 - clk_pcie_core clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pcie_core_clk_sel(&mut self) -> ClkPcieCoreClkSelW<ClkselCon18Spec> {
        ClkPcieCoreClkSelW::new(self, 7)
    }
    #[doc = "Bits 8:9 - clk_pcie_core clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pcie_core_pll_sel(&mut self) -> ClkPcieCorePllSelW<ClkselCon18Spec> {
        ClkPcieCorePllSelW::new(self, 8)
    }
    #[doc = "Bit 10 - clk_pciephy_ref clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pciephy_ref_sel(&mut self) -> ClkPciephyRefSelW<ClkselCon18Spec> {
        ClkPciephyRefSelW::new(self, 10)
    }
    #[doc = "Bits 11:15 - clk_pciephy_ref100m divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pciephy_ref100m_div_con(&mut self) -> ClkPciephyRef100mDivConW<ClkselCon18Spec> {
        ClkPciephyRef100mDivConW::new(self, 11)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon18Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon18Spec;
impl crate::RegisterSpec for ClkselCon18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con18::R`](R) reader structure"]
impl crate::Readable for ClkselCon18Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con18::W`](W) writer structure"]
impl crate::Writable for ClkselCon18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON18 to value 0x4a87"]
impl crate::Resettable for ClkselCon18Spec {
    const RESET_VALUE: u32 = 0x4a87;
}
