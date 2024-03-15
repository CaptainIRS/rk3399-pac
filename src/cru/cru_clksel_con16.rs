#[doc = "Register `CRU_CLKSEL_CON16` reader"]
pub type R = crate::R<CruClkselCon16Spec>;
#[doc = "Register `CRU_CLKSEL_CON16` writer"]
pub type W = crate::W<CruClkselCon16Spec>;
#[doc = "Field `CLK_SDMMC_DIV_CON` reader - clk_sdmmc divider control register clk=clk_src/(div_con+1)"]
pub type ClkSdmmcDivConR = crate::FieldReader;
#[doc = "Field `CLK_SDMMC_DIV_CON` writer - clk_sdmmc divider control register clk=clk_src/(div_con+1)"]
pub type ClkSdmmcDivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_sdmmc clock source select control register\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkSdmmcPllSel {
    #[doc = "0: xin_24m"]
    B000 = 0,
    #[doc = "1: xin_24m"]
    B001 = 1,
    #[doc = "2: xin_24m"]
    B010 = 2,
    #[doc = "3: xin_24m"]
    B011 = 3,
    #[doc = "4: xin_24m"]
    B100 = 4,
    #[doc = "5: xin_24m"]
    B101 = 5,
}
impl From<ClkSdmmcPllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkSdmmcPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkSdmmcPllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_SDMMC_PLL_SEL` reader - clk_sdmmc clock source select control register"]
pub type ClkSdmmcPllSelR = crate::FieldReader<ClkSdmmcPllSel>;
impl ClkSdmmcPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkSdmmcPllSel> {
        match self.bits {
            0 => Some(ClkSdmmcPllSel::B000),
            1 => Some(ClkSdmmcPllSel::B001),
            2 => Some(ClkSdmmcPllSel::B010),
            3 => Some(ClkSdmmcPllSel::B011),
            4 => Some(ClkSdmmcPllSel::B100),
            5 => Some(ClkSdmmcPllSel::B101),
            _ => None,
        }
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == ClkSdmmcPllSel::B000
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == ClkSdmmcPllSel::B001
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == ClkSdmmcPllSel::B010
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == ClkSdmmcPllSel::B011
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == ClkSdmmcPllSel::B100
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == ClkSdmmcPllSel::B101
    }
}
#[doc = "Field `CLK_SDMMC_PLL_SEL` writer - clk_sdmmc clock source select control register"]
pub type ClkSdmmcPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, ClkSdmmcPllSel>;
impl<'a, REG> ClkSdmmcPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSdmmcPllSel::B000)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSdmmcPllSel::B001)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSdmmcPllSel::B010)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSdmmcPllSel::B011)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSdmmcPllSel::B100)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSdmmcPllSel::B101)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_sdmmc divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_sdmmc_div_con(&self) -> ClkSdmmcDivConR {
        ClkSdmmcDivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - clk_sdmmc clock source select control register"]
    #[inline(always)]
    pub fn clk_sdmmc_pll_sel(&self) -> ClkSdmmcPllSelR {
        ClkSdmmcPllSelR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_sdmmc divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sdmmc_div_con(&mut self) -> ClkSdmmcDivConW<CruClkselCon16Spec> {
        ClkSdmmcDivConW::new(self, 0)
    }
    #[doc = "Bits 8:10 - clk_sdmmc clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sdmmc_pll_sel(&mut self) -> ClkSdmmcPllSelW<CruClkselCon16Spec> {
        ClkSdmmcPllSelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon16Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon16Spec;
impl crate::RegisterSpec for CruClkselCon16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con16::R`](R) reader structure"]
impl crate::Readable for CruClkselCon16Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con16::W`](W) writer structure"]
impl crate::Writable for CruClkselCon16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON16 to value 0x0500"]
impl crate::Resettable for CruClkselCon16Spec {
    const RESET_VALUE: u32 = 0x0500;
}
