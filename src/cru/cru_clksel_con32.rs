#[doc = "Register `CRU_CLKSEL_CON32` reader"]
pub type R = crate::R<CruClkselCon32Spec>;
#[doc = "Register `CRU_CLKSEL_CON32` writer"]
pub type W = crate::W<CruClkselCon32Spec>;
#[doc = "Field `CLK_SPDIF_8CH_PLL_DIV_CON` reader - clk_spdif_8ch_pll divider control register clk=clk_src/(div_con+1)"]
pub type ClkSpdif8chPllDivConR = crate::FieldReader;
#[doc = "Field `CLK_SPDIF_8CH_PLL_DIV_CON` writer - clk_spdif_8ch_pll divider control register clk=clk_src/(div_con+1)"]
pub type ClkSpdif8chPllDivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_spdif_8ch clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSpdif8chPllSel {
    #[doc = "0: GPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkSpdif8chPllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkSpdif8chPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SPDIF_8CH_PLL_SEL` reader - clk_spdif_8ch clock source select control register"]
pub type ClkSpdif8chPllSelR = crate::BitReader<ClkSpdif8chPllSel>;
impl ClkSpdif8chPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSpdif8chPllSel {
        match self.bits {
            false => ClkSpdif8chPllSel::B0,
            true => ClkSpdif8chPllSel::B1,
        }
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkSpdif8chPllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkSpdif8chPllSel::B1
    }
}
#[doc = "Field `CLK_SPDIF_8CH_PLL_SEL` writer - clk_spdif_8ch clock source select control register"]
pub type ClkSpdif8chPllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkSpdif8chPllSel>;
impl<'a, REG> ClkSpdif8chPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpdif8chPllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpdif8chPllSel::B1)
    }
}
#[doc = "Field `CLK_DPTX_SPDIF_REC_DIV_CON` reader - clk_dptx_spdif_rec divider control register clk=clk_src/(div_con+1)"]
pub type ClkDptxSpdifRecDivConR = crate::FieldReader;
#[doc = "Field `CLK_DPTX_SPDIF_REC_DIV_CON` writer - clk_dptx_spdif_rec divider control register clk=clk_src/(div_con+1)"]
pub type ClkDptxSpdifRecDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_spdif_8ch clock select control register\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkSpdif8chClkSel {
    #[doc = "0: clk_12m"]
    B00 = 0,
    #[doc = "1: clk_12m"]
    B01 = 1,
    #[doc = "2: clk_12m"]
    B10 = 2,
    #[doc = "3: clk_12m"]
    B11 = 3,
}
impl From<ClkSpdif8chClkSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkSpdif8chClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkSpdif8chClkSel {
    type Ux = u8;
}
#[doc = "Field `CLK_SPDIF_8CH_CLK_SEL` reader - clk_spdif_8ch clock select control register"]
pub type ClkSpdif8chClkSelR = crate::FieldReader<ClkSpdif8chClkSel>;
impl ClkSpdif8chClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSpdif8chClkSel {
        match self.bits {
            0 => ClkSpdif8chClkSel::B00,
            1 => ClkSpdif8chClkSel::B01,
            2 => ClkSpdif8chClkSel::B10,
            3 => ClkSpdif8chClkSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkSpdif8chClkSel::B00
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkSpdif8chClkSel::B01
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkSpdif8chClkSel::B10
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ClkSpdif8chClkSel::B11
    }
}
#[doc = "Field `CLK_SPDIF_8CH_CLK_SEL` writer - clk_spdif_8ch clock select control register"]
pub type ClkSpdif8chClkSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ClkSpdif8chClkSel>;
impl<'a, REG> ClkSpdif8chClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpdif8chClkSel::B00)
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpdif8chClkSel::B01)
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpdif8chClkSel::B10)
    }
    #[doc = "clk_12m"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpdif8chClkSel::B11)
    }
}
#[doc = "clk_dptx_spdif_rec clock source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkDptxSpdifRecPllSel {
    #[doc = "0: GPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkDptxSpdifRecPllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkDptxSpdifRecPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_DPTX_SPDIF_REC_PLL_SEL` reader - clk_dptx_spdif_rec clock source select control register"]
pub type ClkDptxSpdifRecPllSelR = crate::BitReader<ClkDptxSpdifRecPllSel>;
impl ClkDptxSpdifRecPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkDptxSpdifRecPllSel {
        match self.bits {
            false => ClkDptxSpdifRecPllSel::B0,
            true => ClkDptxSpdifRecPllSel::B1,
        }
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkDptxSpdifRecPllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkDptxSpdifRecPllSel::B1
    }
}
#[doc = "Field `CLK_DPTX_SPDIF_REC_PLL_SEL` writer - clk_dptx_spdif_rec clock source select control register"]
pub type ClkDptxSpdifRecPllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkDptxSpdifRecPllSel>;
impl<'a, REG> ClkDptxSpdifRecPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDptxSpdifRecPllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkDptxSpdifRecPllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_spdif_8ch_pll divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_spdif_8ch_pll_div_con(&self) -> ClkSpdif8chPllDivConR {
        ClkSpdif8chPllDivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - clk_spdif_8ch clock source select control register"]
    #[inline(always)]
    pub fn clk_spdif_8ch_pll_sel(&self) -> ClkSpdif8chPllSelR {
        ClkSpdif8chPllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - clk_dptx_spdif_rec divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_dptx_spdif_rec_div_con(&self) -> ClkDptxSpdifRecDivConR {
        ClkDptxSpdifRecDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:14 - clk_spdif_8ch clock select control register"]
    #[inline(always)]
    pub fn clk_spdif_8ch_clk_sel(&self) -> ClkSpdif8chClkSelR {
        ClkSpdif8chClkSelR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - clk_dptx_spdif_rec clock source select control register"]
    #[inline(always)]
    pub fn clk_dptx_spdif_rec_pll_sel(&self) -> ClkDptxSpdifRecPllSelR {
        ClkDptxSpdifRecPllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_spdif_8ch_pll divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spdif_8ch_pll_div_con(&mut self) -> ClkSpdif8chPllDivConW<CruClkselCon32Spec> {
        ClkSpdif8chPllDivConW::new(self, 0)
    }
    #[doc = "Bit 7 - clk_spdif_8ch clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spdif_8ch_pll_sel(&mut self) -> ClkSpdif8chPllSelW<CruClkselCon32Spec> {
        ClkSpdif8chPllSelW::new(self, 7)
    }
    #[doc = "Bits 8:12 - clk_dptx_spdif_rec divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dptx_spdif_rec_div_con(&mut self) -> ClkDptxSpdifRecDivConW<CruClkselCon32Spec> {
        ClkDptxSpdifRecDivConW::new(self, 8)
    }
    #[doc = "Bits 13:14 - clk_spdif_8ch clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spdif_8ch_clk_sel(&mut self) -> ClkSpdif8chClkSelW<CruClkselCon32Spec> {
        ClkSpdif8chClkSelW::new(self, 13)
    }
    #[doc = "Bit 15 - clk_dptx_spdif_rec clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dptx_spdif_rec_pll_sel(&mut self) -> ClkDptxSpdifRecPllSelW<CruClkselCon32Spec> {
        ClkDptxSpdifRecPllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon32Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon32Spec;
impl crate::RegisterSpec for CruClkselCon32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con32::R`](R) reader structure"]
impl crate::Readable for CruClkselCon32Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con32::W`](W) writer structure"]
impl crate::Writable for CruClkselCon32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON32 to value 0x6300"]
impl crate::Resettable for CruClkselCon32Spec {
    const RESET_VALUE: u32 = 0x6300;
}
