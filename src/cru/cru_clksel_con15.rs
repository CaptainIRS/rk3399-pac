#[doc = "Register `CRU_CLKSEL_CON15` reader"]
pub type R = crate::R<CruClkselCon15Spec>;
#[doc = "Register `CRU_CLKSEL_CON15` writer"]
pub type W = crate::W<CruClkselCon15Spec>;
#[doc = "Field `CLK_SDIO_DIV_CON` reader - clk_sdio divider control register clk=clk_src/(div_con+1)"]
pub type ClkSdioDivConR = crate::FieldReader;
#[doc = "Field `CLK_SDIO_DIV_CON` writer - clk_sdio divider control register clk=clk_src/(div_con+1)"]
pub type ClkSdioDivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_sdio clock source select control register\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkSdioPllSel {
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
impl From<ClkSdioPllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkSdioPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkSdioPllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_SDIO_PLL_SEL` reader - clk_sdio clock source select control register"]
pub type ClkSdioPllSelR = crate::FieldReader<ClkSdioPllSel>;
impl ClkSdioPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkSdioPllSel> {
        match self.bits {
            0 => Some(ClkSdioPllSel::B000),
            1 => Some(ClkSdioPllSel::B001),
            2 => Some(ClkSdioPllSel::B010),
            3 => Some(ClkSdioPllSel::B011),
            4 => Some(ClkSdioPllSel::B100),
            5 => Some(ClkSdioPllSel::B101),
            _ => None,
        }
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == ClkSdioPllSel::B000
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == ClkSdioPllSel::B001
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == ClkSdioPllSel::B010
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == ClkSdioPllSel::B011
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == ClkSdioPllSel::B100
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == ClkSdioPllSel::B101
    }
}
#[doc = "Field `CLK_SDIO_PLL_SEL` writer - clk_sdio clock source select control register"]
pub type ClkSdioPllSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, ClkSdioPllSel>;
impl<'a, REG> ClkSdioPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSdioPllSel::B000)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSdioPllSel::B001)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSdioPllSel::B010)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSdioPllSel::B011)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSdioPllSel::B100)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSdioPllSel::B101)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_sdio divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_sdio_div_con(&self) -> ClkSdioDivConR {
        ClkSdioDivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - clk_sdio clock source select control register"]
    #[inline(always)]
    pub fn clk_sdio_pll_sel(&self) -> ClkSdioPllSelR {
        ClkSdioPllSelR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_sdio divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sdio_div_con(&mut self) -> ClkSdioDivConW<CruClkselCon15Spec> {
        ClkSdioDivConW::new(self, 0)
    }
    #[doc = "Bits 8:10 - clk_sdio clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sdio_pll_sel(&mut self) -> ClkSdioPllSelW<CruClkselCon15Spec> {
        ClkSdioPllSelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon15Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon15Spec;
impl crate::RegisterSpec for CruClkselCon15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con15::R`](R) reader structure"]
impl crate::Readable for CruClkselCon15Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con15::W`](W) writer structure"]
impl crate::Writable for CruClkselCon15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON15 to value 0x0500"]
impl crate::Resettable for CruClkselCon15Spec {
    const RESET_VALUE: u32 = 0x0500;
}
