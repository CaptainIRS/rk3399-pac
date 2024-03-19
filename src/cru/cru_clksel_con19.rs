#[doc = "Register `CRU_CLKSEL_CON19` reader"]
pub type R = crate::R<CruClkselCon19Spec>;
#[doc = "Register `CRU_CLKSEL_CON19` writer"]
pub type W = crate::W<CruClkselCon19Spec>;
#[doc = "clk_hsicphy clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkHsicphyPllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: NPLL"]
    B10 = 2,
    #[doc = "3: USB_480M"]
    B11 = 3,
}
impl From<ClkHsicphyPllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkHsicphyPllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkHsicphyPllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_HSICPHY_PLL_SEL` reader - clk_hsicphy clock source select control register"]
pub type ClkHsicphyPllSelR = crate::FieldReader<ClkHsicphyPllSel>;
impl ClkHsicphyPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkHsicphyPllSel {
        match self.bits {
            0 => ClkHsicphyPllSel::B00,
            1 => ClkHsicphyPllSel::B01,
            2 => ClkHsicphyPllSel::B10,
            3 => ClkHsicphyPllSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkHsicphyPllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkHsicphyPllSel::B01
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkHsicphyPllSel::B10
    }
    #[doc = "USB_480M"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ClkHsicphyPllSel::B11
    }
}
#[doc = "Field `CLK_HSICPHY_PLL_SEL` writer - clk_hsicphy clock source select control register"]
pub type ClkHsicphyPllSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ClkHsicphyPllSel>;
impl<'a, REG> ClkHsicphyPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkHsicphyPllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkHsicphyPllSel::B01)
    }
    #[doc = "NPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkHsicphyPllSel::B10)
    }
    #[doc = "USB_480M"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(ClkHsicphyPllSel::B11)
    }
}
#[doc = "clk_rmii_src clock select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkRmiiSrcSel {
    #[doc = "0: clk_mac_divout"]
    B0 = 0,
    #[doc = "1: rmii_clkin from IO"]
    B1 = 1,
}
impl From<ClkRmiiSrcSel> for bool {
    #[inline(always)]
    fn from(variant: ClkRmiiSrcSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_RMII_SRC_SEL` reader - clk_rmii_src clock select control register"]
pub type ClkRmiiSrcSelR = crate::BitReader<ClkRmiiSrcSel>;
impl ClkRmiiSrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkRmiiSrcSel {
        match self.bits {
            false => ClkRmiiSrcSel::B0,
            true => ClkRmiiSrcSel::B1,
        }
    }
    #[doc = "clk_mac_divout"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkRmiiSrcSel::B0
    }
    #[doc = "rmii_clkin from IO"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkRmiiSrcSel::B1
    }
}
#[doc = "Field `CLK_RMII_SRC_SEL` writer - clk_rmii_src clock select control register"]
pub type ClkRmiiSrcSelW<'a, REG> = crate::BitWriter<'a, REG, ClkRmiiSrcSel>;
impl<'a, REG> ClkRmiiSrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clk_mac_divout"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkRmiiSrcSel::B0)
    }
    #[doc = "rmii_clkin from IO"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkRmiiSrcSel::B1)
    }
}
#[doc = "Field `PCLK_GMAC_DIV_CON` reader - pclk_gmac divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkGmacDivConR = crate::FieldReader;
#[doc = "Field `PCLK_GMAC_DIV_CON` writer - pclk_gmac divider control register\n\nclk=clk_src/(div_con+1)"]
pub type PclkGmacDivConW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - clk_hsicphy clock source select control register"]
    #[inline(always)]
    pub fn clk_hsicphy_pll_sel(&self) -> ClkHsicphyPllSelR {
        ClkHsicphyPllSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - clk_rmii_src clock select control register"]
    #[inline(always)]
    pub fn clk_rmii_src_sel(&self) -> ClkRmiiSrcSelR {
        ClkRmiiSrcSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - pclk_gmac divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn pclk_gmac_div_con(&self) -> PclkGmacDivConR {
        PclkGmacDivConR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - clk_hsicphy clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_hsicphy_pll_sel(&mut self) -> ClkHsicphyPllSelW<CruClkselCon19Spec> {
        ClkHsicphyPllSelW::new(self, 0)
    }
    #[doc = "Bit 4 - clk_rmii_src clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_rmii_src_sel(&mut self) -> ClkRmiiSrcSelW<CruClkselCon19Spec> {
        ClkRmiiSrcSelW::new(self, 4)
    }
    #[doc = "Bits 8:10 - pclk_gmac divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_gmac_div_con(&mut self) -> PclkGmacDivConW<CruClkselCon19Spec> {
        PclkGmacDivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon19Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon19Spec;
impl crate::RegisterSpec for CruClkselCon19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con19::R`](R) reader structure"]
impl crate::Readable for CruClkselCon19Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con19::W`](W) writer structure"]
impl crate::Writable for CruClkselCon19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON19 to value 0x0100"]
impl crate::Resettable for CruClkselCon19Spec {
    const RESET_VALUE: u32 = 0x0100;
}
