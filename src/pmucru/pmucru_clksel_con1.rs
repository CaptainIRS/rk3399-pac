#[doc = "Register `PMUCRU_CLKSEL_CON1` reader"]
pub type R = crate::R<PmucruClkselCon1Spec>;
#[doc = "Register `PMUCRU_CLKSEL_CON1` writer"]
pub type W = crate::W<PmucruClkselCon1Spec>;
#[doc = "Field `CLK_SPI3_DIV_CON` reader - clk_spi3 divider control register clk=clk_src/(div_con+1)"]
pub type ClkSpi3DivConR = crate::FieldReader;
#[doc = "Field `CLK_SPI3_DIV_CON` writer - clk_spi3 divider control register clk=clk_src/(div_con+1)"]
pub type ClkSpi3DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_spi3_pll source select control register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSpi3PllSel {
    #[doc = "0: PPLL"]
    B0 = 0,
    #[doc = "1: PPLL"]
    B1 = 1,
}
impl From<ClkSpi3PllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkSpi3PllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_SPI3_PLL_SEL` reader - clk_spi3_pll source select control register"]
pub type ClkSpi3PllSelR = crate::BitReader<ClkSpi3PllSel>;
impl ClkSpi3PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSpi3PllSel {
        match self.bits {
            false => ClkSpi3PllSel::B0,
            true => ClkSpi3PllSel::B1,
        }
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkSpi3PllSel::B0
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkSpi3PllSel::B1
    }
}
#[doc = "Field `CLK_SPI3_PLL_SEL` writer - clk_spi3_pll source select control register"]
pub type ClkSpi3PllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkSpi3PllSel>;
impl<'a, REG> ClkSpi3PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpi3PllSel::B0)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSpi3PllSel::B1)
    }
}
#[doc = "Field `CLK_WIFI_DIV_CON` reader - clk_wifi divider control register clk=clk_src/(div_con+1)"]
pub type ClkWifiDivConR = crate::FieldReader;
#[doc = "Field `CLK_WIFI_DIV_CON` writer - clk_wifi divider control register clk=clk_src/(div_con+1)"]
pub type ClkWifiDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_wifi_pll source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkWifiPllSel {
    #[doc = "0: xin_24m"]
    B0 = 0,
    #[doc = "1: xin_24m"]
    B1 = 1,
}
impl From<ClkWifiPllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkWifiPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_WIFI_PLL_SEL` reader - clk_wifi_pll source select control register"]
pub type ClkWifiPllSelR = crate::BitReader<ClkWifiPllSel>;
impl ClkWifiPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkWifiPllSel {
        match self.bits {
            false => ClkWifiPllSel::B0,
            true => ClkWifiPllSel::B1,
        }
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkWifiPllSel::B0
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkWifiPllSel::B1
    }
}
#[doc = "Field `CLK_WIFI_PLL_SEL` writer - clk_wifi_pll source select control register"]
pub type ClkWifiPllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkWifiPllSel>;
impl<'a, REG> ClkWifiPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkWifiPllSel::B0)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkWifiPllSel::B1)
    }
}
#[doc = "clk_wifi source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkWifiSel {
    #[doc = "0: clk_wifi_frac"]
    B0 = 0,
    #[doc = "1: clk_wifi_frac"]
    B1 = 1,
}
impl From<ClkWifiSel> for bool {
    #[inline(always)]
    fn from(variant: ClkWifiSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_WIFI_SEL` reader - clk_wifi source select control register"]
pub type ClkWifiSelR = crate::BitReader<ClkWifiSel>;
impl ClkWifiSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkWifiSel {
        match self.bits {
            false => ClkWifiSel::B0,
            true => ClkWifiSel::B1,
        }
    }
    #[doc = "clk_wifi_frac"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkWifiSel::B0
    }
    #[doc = "clk_wifi_frac"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkWifiSel::B1
    }
}
#[doc = "Field `CLK_WIFI_SEL` writer - clk_wifi source select control register"]
pub type ClkWifiSelW<'a, REG> = crate::BitWriter<'a, REG, ClkWifiSel>;
impl<'a, REG> ClkWifiSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clk_wifi_frac"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkWifiSel::B0)
    }
    #[doc = "clk_wifi_frac"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkWifiSel::B1)
    }
}
#[doc = "clk_timer source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkTimerSel {
    #[doc = "0: clk_32k"]
    B0 = 0,
    #[doc = "1: clk_32k"]
    B1 = 1,
}
impl From<ClkTimerSel> for bool {
    #[inline(always)]
    fn from(variant: ClkTimerSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_TIMER_SEL` reader - clk_timer source select control register"]
pub type ClkTimerSelR = crate::BitReader<ClkTimerSel>;
impl ClkTimerSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkTimerSel {
        match self.bits {
            false => ClkTimerSel::B0,
            true => ClkTimerSel::B1,
        }
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkTimerSel::B0
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkTimerSel::B1
    }
}
#[doc = "Field `CLK_TIMER_SEL` writer - clk_timer source select control register"]
pub type ClkTimerSelW<'a, REG> = crate::BitWriter<'a, REG, ClkTimerSel>;
impl<'a, REG> ClkTimerSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTimerSel::B0)
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkTimerSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_spi3 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_spi3_div_con(&self) -> ClkSpi3DivConR {
        ClkSpi3DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - clk_spi3_pll source select control register"]
    #[inline(always)]
    pub fn clk_spi3_pll_sel(&self) -> ClkSpi3PllSelR {
        ClkSpi3PllSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - clk_wifi divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_wifi_div_con(&self) -> ClkWifiDivConR {
        ClkWifiDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - clk_wifi_pll source select control register"]
    #[inline(always)]
    pub fn clk_wifi_pll_sel(&self) -> ClkWifiPllSelR {
        ClkWifiPllSelR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - clk_wifi source select control register"]
    #[inline(always)]
    pub fn clk_wifi_sel(&self) -> ClkWifiSelR {
        ClkWifiSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - clk_timer source select control register"]
    #[inline(always)]
    pub fn clk_timer_sel(&self) -> ClkTimerSelR {
        ClkTimerSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_spi3 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi3_div_con(&mut self) -> ClkSpi3DivConW<PmucruClkselCon1Spec> {
        ClkSpi3DivConW::new(self, 0)
    }
    #[doc = "Bit 7 - clk_spi3_pll source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi3_pll_sel(&mut self) -> ClkSpi3PllSelW<PmucruClkselCon1Spec> {
        ClkSpi3PllSelW::new(self, 7)
    }
    #[doc = "Bits 8:12 - clk_wifi divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifi_div_con(&mut self) -> ClkWifiDivConW<PmucruClkselCon1Spec> {
        ClkWifiDivConW::new(self, 8)
    }
    #[doc = "Bit 13 - clk_wifi_pll source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifi_pll_sel(&mut self) -> ClkWifiPllSelW<PmucruClkselCon1Spec> {
        ClkWifiPllSelW::new(self, 13)
    }
    #[doc = "Bit 14 - clk_wifi source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifi_sel(&mut self) -> ClkWifiSelW<PmucruClkselCon1Spec> {
        ClkWifiSelW::new(self, 14)
    }
    #[doc = "Bit 15 - clk_timer source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_timer_sel(&mut self) -> ClkTimerSelW<PmucruClkselCon1Spec> {
        ClkTimerSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PmucruClkselCon1Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clksel_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clksel_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucruClkselCon1Spec;
impl crate::RegisterSpec for PmucruClkselCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucru_clksel_con1::R`](R) reader structure"]
impl crate::Readable for PmucruClkselCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`pmucru_clksel_con1::W`](W) writer structure"]
impl crate::Writable for PmucruClkselCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUCRU_CLKSEL_CON1 to value 0x1986"]
impl crate::Resettable for PmucruClkselCon1Spec {
    const RESET_VALUE: u32 = 0x1986;
}
