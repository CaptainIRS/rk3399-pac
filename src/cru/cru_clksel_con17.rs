#[doc = "Register `CRU_CLKSEL_CON17` reader"]
pub type R = crate::R<CruClkselCon17Spec>;
#[doc = "Register `CRU_CLKSEL_CON17` writer"]
pub type W = crate::W<CruClkselCon17Spec>;
#[doc = "Field `CLK_PCIE_PM_DIV_CON` reader - clk_pcie_pm divider control register clk=clk_src/(div_con+1)"]
pub type ClkPciePmDivConR = crate::FieldReader;
#[doc = "Field `CLK_PCIE_PM_DIV_CON` writer - clk_pcie_pm divider control register clk=clk_src/(div_con+1)"]
pub type ClkPciePmDivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_pcie_pm clock source select control register\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkPciePmPllSel {
    #[doc = "0: reserved"]
    B000 = 0,
    #[doc = "1: reserved"]
    B001 = 1,
    #[doc = "2: reserved"]
    B010 = 2,
    #[doc = "3: reserved"]
    B011 = 3,
    #[doc = "4: reserved"]
    B1xx = 4,
}
impl From<ClkPciePmPllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkPciePmPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkPciePmPllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_PCIE_PM_PLL_SEL` reader - clk_pcie_pm clock source select control register"]
pub type ClkPciePmPllSelR = crate::FieldReader<ClkPciePmPllSel>;
impl ClkPciePmPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkPciePmPllSel> {
        match self.bits {
            0 => Some(ClkPciePmPllSel::B000),
            1 => Some(ClkPciePmPllSel::B001),
            2 => Some(ClkPciePmPllSel::B010),
            3 => Some(ClkPciePmPllSel::B011),
            4 => Some(ClkPciePmPllSel::B1xx),
            _ => None,
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == ClkPciePmPllSel::B000
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == ClkPciePmPllSel::B001
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == ClkPciePmPllSel::B010
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == ClkPciePmPllSel::B011
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b1xx(&self) -> bool {
        *self == ClkPciePmPllSel::B1xx
    }
}
#[doc = "Field `CLK_PCIE_PM_PLL_SEL` writer - clk_pcie_pm clock source select control register"]
pub type ClkPciePmPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, ClkPciePmPllSel>;
impl<'a, REG> ClkPciePmPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPciePmPllSel::B000)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPciePmPllSel::B001)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPciePmPllSel::B010)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPciePmPllSel::B011)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b1xx(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPciePmPllSel::B1xx)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_pcie_pm divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_pcie_pm_div_con(&self) -> ClkPciePmDivConR {
        ClkPciePmDivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - clk_pcie_pm clock source select control register"]
    #[inline(always)]
    pub fn clk_pcie_pm_pll_sel(&self) -> ClkPciePmPllSelR {
        ClkPciePmPllSelR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_pcie_pm divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pcie_pm_div_con(&mut self) -> ClkPciePmDivConW<CruClkselCon17Spec> {
        ClkPciePmDivConW::new(self, 0)
    }
    #[doc = "Bits 8:10 - clk_pcie_pm clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pcie_pm_pll_sel(&mut self) -> ClkPciePmPllSelW<CruClkselCon17Spec> {
        ClkPciePmPllSelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon17Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon17Spec;
impl crate::RegisterSpec for CruClkselCon17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con17::R`](R) reader structure"]
impl crate::Readable for CruClkselCon17Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con17::W`](W) writer structure"]
impl crate::Writable for CruClkselCon17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON17 to value 0x0300"]
impl crate::Resettable for CruClkselCon17Spec {
    const RESET_VALUE: u32 = 0x0300;
}
