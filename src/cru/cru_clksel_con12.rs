#[doc = "Register `CRU_CLKSEL_CON12` reader"]
pub type R = crate::R<CruClkselCon12Spec>;
#[doc = "Register `CRU_CLKSEL_CON12` writer"]
pub type W = crate::W<CruClkselCon12Spec>;
#[doc = "Field `CLK_RGA_CORE_DIV_CON` reader - clk_rga_core divider control register clk=clk_src/(div_con+1)"]
pub type ClkRgaCoreDivConR = crate::FieldReader;
#[doc = "Field `CLK_RGA_CORE_DIV_CON` writer - clk_rga_core divider control register clk=clk_src/(div_con+1)"]
pub type ClkRgaCoreDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_rga_core clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkRgaCorePllSel {
    #[doc = "0: PPLL"]
    B00 = 0,
    #[doc = "1: PPLL"]
    B01 = 1,
    #[doc = "2: PPLL"]
    B10 = 2,
    #[doc = "3: PPLL"]
    B11 = 3,
}
impl From<ClkRgaCorePllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkRgaCorePllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkRgaCorePllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_RGA_CORE_PLL_SEL` reader - clk_rga_core clock source select control register"]
pub type ClkRgaCorePllSelR = crate::FieldReader<ClkRgaCorePllSel>;
impl ClkRgaCorePllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkRgaCorePllSel {
        match self.bits {
            0 => ClkRgaCorePllSel::B00,
            1 => ClkRgaCorePllSel::B01,
            2 => ClkRgaCorePllSel::B10,
            3 => ClkRgaCorePllSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkRgaCorePllSel::B00
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkRgaCorePllSel::B01
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkRgaCorePllSel::B10
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ClkRgaCorePllSel::B11
    }
}
#[doc = "Field `CLK_RGA_CORE_PLL_SEL` writer - clk_rga_core clock source select control register"]
pub type ClkRgaCorePllSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ClkRgaCorePllSel>;
impl<'a, REG> ClkRgaCorePllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkRgaCorePllSel::B00)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkRgaCorePllSel::B01)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkRgaCorePllSel::B10)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(ClkRgaCorePllSel::B11)
    }
}
#[doc = "Field `ACLK_CENTER_DIV_CON` reader - aclk_center divider control register clk=clk_src/(div_con+1)"]
pub type AclkCenterDivConR = crate::FieldReader;
#[doc = "Field `ACLK_CENTER_DIV_CON` writer - aclk_center divider control register clk=clk_src/(div_con+1)"]
pub type AclkCenterDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_center clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkCenterPllSel {
    #[doc = "0: NPLL"]
    B00 = 0,
    #[doc = "1: NPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B1x = 2,
}
impl From<AclkCenterPllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkCenterPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkCenterPllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_CENTER_PLL_SEL` reader - aclk_center clock source select control register"]
pub type AclkCenterPllSelR = crate::FieldReader<AclkCenterPllSel>;
impl AclkCenterPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AclkCenterPllSel> {
        match self.bits {
            0 => Some(AclkCenterPllSel::B00),
            1 => Some(AclkCenterPllSel::B01),
            2 => Some(AclkCenterPllSel::B1x),
            _ => None,
        }
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AclkCenterPllSel::B00
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AclkCenterPllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == AclkCenterPllSel::B1x
    }
}
#[doc = "Field `ACLK_CENTER_PLL_SEL` writer - aclk_center clock source select control register"]
pub type AclkCenterPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, AclkCenterPllSel>;
impl<'a, REG> AclkCenterPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AclkCenterPllSel::B00)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AclkCenterPllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(AclkCenterPllSel::B1x)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - clk_rga_core divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_rga_core_div_con(&self) -> ClkRgaCoreDivConR {
        ClkRgaCoreDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - clk_rga_core clock source select control register"]
    #[inline(always)]
    pub fn clk_rga_core_pll_sel(&self) -> ClkRgaCorePllSelR {
        ClkRgaCorePllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - aclk_center divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_center_div_con(&self) -> AclkCenterDivConR {
        AclkCenterDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - aclk_center clock source select control register"]
    #[inline(always)]
    pub fn aclk_center_pll_sel(&self) -> AclkCenterPllSelR {
        AclkCenterPllSelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk_rga_core divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_rga_core_div_con(&mut self) -> ClkRgaCoreDivConW<CruClkselCon12Spec> {
        ClkRgaCoreDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - clk_rga_core clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_rga_core_pll_sel(&mut self) -> ClkRgaCorePllSelW<CruClkselCon12Spec> {
        ClkRgaCorePllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - aclk_center divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_center_div_con(&mut self) -> AclkCenterDivConW<CruClkselCon12Spec> {
        AclkCenterDivConW::new(self, 8)
    }
    #[doc = "Bits 14:15 - aclk_center clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_center_pll_sel(&mut self) -> AclkCenterPllSelW<CruClkselCon12Spec> {
        AclkCenterPllSelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon12Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon12Spec;
impl crate::RegisterSpec for CruClkselCon12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con12::R`](R) reader structure"]
impl crate::Readable for CruClkselCon12Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con12::W`](W) writer structure"]
impl crate::Writable for CruClkselCon12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON12 to value 0x0100"]
impl crate::Resettable for CruClkselCon12Spec {
    const RESET_VALUE: u32 = 0x0100;
}
