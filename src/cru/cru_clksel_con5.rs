#[doc = "Register `CRU_CLKSEL_CON5` reader"]
pub type R = crate::R<CruClkselCon5Spec>;
#[doc = "Register `CRU_CLKSEL_CON5` writer"]
pub type W = crate::W<CruClkselCon5Spec>;
#[doc = "Field `ACLK_CCI_DIV_CON` reader - aclk_cci divider control register clk=clk_src/(div_con+1)"]
pub type AclkCciDivConR = crate::FieldReader;
#[doc = "Field `ACLK_CCI_DIV_CON` writer - aclk_cci divider control register clk=clk_src/(div_con+1)"]
pub type AclkCciDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "aclk_cci clock source select control register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AclkCciPllSel {
    #[doc = "0: VPLL"]
    B00 = 0,
    #[doc = "1: VPLL"]
    B01 = 1,
    #[doc = "2: VPLL"]
    B10 = 2,
    #[doc = "3: VPLL"]
    B11 = 3,
}
impl From<AclkCciPllSel> for u8 {
    #[inline(always)]
    fn from(variant: AclkCciPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AclkCciPllSel {
    type Ux = u8;
}
#[doc = "Field `ACLK_CCI_PLL_SEL` reader - aclk_cci clock source select control register"]
pub type AclkCciPllSelR = crate::FieldReader<AclkCciPllSel>;
impl AclkCciPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AclkCciPllSel {
        match self.bits {
            0 => AclkCciPllSel::B00,
            1 => AclkCciPllSel::B01,
            2 => AclkCciPllSel::B10,
            3 => AclkCciPllSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == AclkCciPllSel::B00
    }
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == AclkCciPllSel::B01
    }
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == AclkCciPllSel::B10
    }
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == AclkCciPllSel::B11
    }
}
#[doc = "Field `ACLK_CCI_PLL_SEL` writer - aclk_cci clock source select control register"]
pub type AclkCciPllSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AclkCciPllSel>;
impl<'a, REG> AclkCciPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(AclkCciPllSel::B00)
    }
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(AclkCciPllSel::B01)
    }
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(AclkCciPllSel::B10)
    }
    #[doc = "VPLL"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(AclkCciPllSel::B11)
    }
}
#[doc = "Field `CLK_CCI_TRACE_DIV_CON` reader - clk_cci_trace divider control register clk=clk_src/(div_con+1)"]
pub type ClkCciTraceDivConR = crate::FieldReader;
#[doc = "Field `CLK_CCI_TRACE_DIV_CON` writer - clk_cci_trace divider control register clk=clk_src/(div_con+1)"]
pub type ClkCciTraceDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_cci_trace clock source select control register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkCciTracePllSel {
    #[doc = "0: GPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkCciTracePllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkCciTracePllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_CCI_TRACE_PLL_SEL` reader - clk_cci_trace clock source select control register"]
pub type ClkCciTracePllSelR = crate::BitReader<ClkCciTracePllSel>;
impl ClkCciTracePllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkCciTracePllSel {
        match self.bits {
            false => ClkCciTracePllSel::B0,
            true => ClkCciTracePllSel::B1,
        }
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkCciTracePllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkCciTracePllSel::B1
    }
}
#[doc = "Field `CLK_CCI_TRACE_PLL_SEL` writer - clk_cci_trace clock source select control register"]
pub type ClkCciTracePllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkCciTracePllSel>;
impl<'a, REG> ClkCciTracePllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCciTracePllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCciTracePllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - aclk_cci divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn aclk_cci_div_con(&self) -> AclkCciDivConR {
        AclkCciDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - aclk_cci clock source select control register"]
    #[inline(always)]
    pub fn aclk_cci_pll_sel(&self) -> AclkCciPllSelR {
        AclkCciPllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - clk_cci_trace divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_cci_trace_div_con(&self) -> ClkCciTraceDivConR {
        ClkCciTraceDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - clk_cci_trace clock source select control register"]
    #[inline(always)]
    pub fn clk_cci_trace_pll_sel(&self) -> ClkCciTracePllSelR {
        ClkCciTracePllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - aclk_cci divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_cci_div_con(&mut self) -> AclkCciDivConW<CruClkselCon5Spec> {
        AclkCciDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - aclk_cci clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_cci_pll_sel(&mut self) -> AclkCciPllSelW<CruClkselCon5Spec> {
        AclkCciPllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - clk_cci_trace divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cci_trace_div_con(&mut self) -> ClkCciTraceDivConW<CruClkselCon5Spec> {
        ClkCciTraceDivConW::new(self, 8)
    }
    #[doc = "Bit 15 - clk_cci_trace clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cci_trace_pll_sel(&mut self) -> ClkCciTracePllSelW<CruClkselCon5Spec> {
        ClkCciTracePllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon5Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon5Spec;
impl crate::RegisterSpec for CruClkselCon5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con5::R`](R) reader structure"]
impl crate::Readable for CruClkselCon5Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con5::W`](W) writer structure"]
impl crate::Writable for CruClkselCon5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON5 to value 0x8341"]
impl crate::Resettable for CruClkselCon5Spec {
    const RESET_VALUE: u32 = 0x8341;
}
