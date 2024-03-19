#[doc = "Register `CRU_CLKSEL_CON22` reader"]
pub type R = crate::R<CruClkselCon22Spec>;
#[doc = "Register `CRU_CLKSEL_CON22` writer"]
pub type W = crate::W<CruClkselCon22Spec>;
#[doc = "Field `CLK_EMMC_DIV_CON` reader - clk_emmc divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkEmmcDivConR = crate::FieldReader;
#[doc = "Field `CLK_EMMC_DIV_CON` writer - clk_emmc divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkEmmcDivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_emmc clock source select control register\n\nValue on reset: 4"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkEmmcPllSel {
    #[doc = "0: CPLL"]
    B000 = 0,
    #[doc = "1: GPLL"]
    B001 = 1,
    #[doc = "2: NPLL"]
    B010 = 2,
    #[doc = "3: USB_480M"]
    B011 = 3,
    #[doc = "4: xin_24m"]
    B1xx = 4,
}
impl From<ClkEmmcPllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkEmmcPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkEmmcPllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_EMMC_PLL_SEL` reader - clk_emmc clock source select control register"]
pub type ClkEmmcPllSelR = crate::FieldReader<ClkEmmcPllSel>;
impl ClkEmmcPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkEmmcPllSel> {
        match self.bits {
            0 => Some(ClkEmmcPllSel::B000),
            1 => Some(ClkEmmcPllSel::B001),
            2 => Some(ClkEmmcPllSel::B010),
            3 => Some(ClkEmmcPllSel::B011),
            4 => Some(ClkEmmcPllSel::B1xx),
            _ => None,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == ClkEmmcPllSel::B000
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == ClkEmmcPllSel::B001
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == ClkEmmcPllSel::B010
    }
    #[doc = "USB_480M"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == ClkEmmcPllSel::B011
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b1xx(&self) -> bool {
        *self == ClkEmmcPllSel::B1xx
    }
}
#[doc = "Field `CLK_EMMC_PLL_SEL` writer - clk_emmc clock source select control register"]
pub type ClkEmmcPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, ClkEmmcPllSel>;
impl<'a, REG> ClkEmmcPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEmmcPllSel::B000)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEmmcPllSel::B001)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEmmcPllSel::B010)
    }
    #[doc = "USB_480M"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEmmcPllSel::B011)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b1xx(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEmmcPllSel::B1xx)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_emmc divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_emmc_div_con(&self) -> ClkEmmcDivConR {
        ClkEmmcDivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - clk_emmc clock source select control register"]
    #[inline(always)]
    pub fn clk_emmc_pll_sel(&self) -> ClkEmmcPllSelR {
        ClkEmmcPllSelR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_emmc divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_emmc_div_con(&mut self) -> ClkEmmcDivConW<CruClkselCon22Spec> {
        ClkEmmcDivConW::new(self, 0)
    }
    #[doc = "Bits 8:10 - clk_emmc clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_emmc_pll_sel(&mut self) -> ClkEmmcPllSelW<CruClkselCon22Spec> {
        ClkEmmcPllSelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon22Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon22Spec;
impl crate::RegisterSpec for CruClkselCon22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con22::R`](R) reader structure"]
impl crate::Readable for CruClkselCon22Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con22::W`](W) writer structure"]
impl crate::Writable for CruClkselCon22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON22 to value 0x0400"]
impl crate::Resettable for CruClkselCon22Spec {
    const RESET_VALUE: u32 = 0x0400;
}
