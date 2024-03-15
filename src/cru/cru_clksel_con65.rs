#[doc = "Register `CRU_CLKSEL_CON65` reader"]
pub type R = crate::R<CruClkselCon65Spec>;
#[doc = "Register `CRU_CLKSEL_CON65` writer"]
pub type W = crate::W<CruClkselCon65Spec>;
#[doc = "Field `CLK_UPHY1_TCPDCORE_DIV_CON` reader - clk_uphy1_tcpdcore divider control register clk=clk_src/(div_con+1)"]
pub type ClkUphy1TcpdcoreDivConR = crate::FieldReader;
#[doc = "Field `CLK_UPHY1_TCPDCORE_DIV_CON` writer - clk_uphy1_tcpdcore divider control register clk=clk_src/(div_con+1)"]
pub type ClkUphy1TcpdcoreDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_uphy1_tcpdcore clock select control register\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkUphy1TcpdcoreClkSel {
    #[doc = "0: gpll"]
    B00 = 0,
    #[doc = "1: gpll"]
    B01 = 1,
    #[doc = "2: gpll"]
    B10 = 2,
    #[doc = "3: gpll"]
    B11 = 3,
}
impl From<ClkUphy1TcpdcoreClkSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkUphy1TcpdcoreClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkUphy1TcpdcoreClkSel {
    type Ux = u8;
}
#[doc = "Field `CLK_UPHY1_TCPDCORE_CLK_SEL` reader - clk_uphy1_tcpdcore clock select control register"]
pub type ClkUphy1TcpdcoreClkSelR = crate::FieldReader<ClkUphy1TcpdcoreClkSel>;
impl ClkUphy1TcpdcoreClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkUphy1TcpdcoreClkSel {
        match self.bits {
            0 => ClkUphy1TcpdcoreClkSel::B00,
            1 => ClkUphy1TcpdcoreClkSel::B01,
            2 => ClkUphy1TcpdcoreClkSel::B10,
            3 => ClkUphy1TcpdcoreClkSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpll"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkUphy1TcpdcoreClkSel::B00
    }
    #[doc = "gpll"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkUphy1TcpdcoreClkSel::B01
    }
    #[doc = "gpll"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkUphy1TcpdcoreClkSel::B10
    }
    #[doc = "gpll"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ClkUphy1TcpdcoreClkSel::B11
    }
}
#[doc = "Field `CLK_UPHY1_TCPDCORE_CLK_SEL` writer - clk_uphy1_tcpdcore clock select control register"]
pub type ClkUphy1TcpdcoreClkSelW<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 2, ClkUphy1TcpdcoreClkSel>;
impl<'a, REG> ClkUphy1TcpdcoreClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpll"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUphy1TcpdcoreClkSel::B00)
    }
    #[doc = "gpll"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUphy1TcpdcoreClkSel::B01)
    }
    #[doc = "gpll"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUphy1TcpdcoreClkSel::B10)
    }
    #[doc = "gpll"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUphy1TcpdcoreClkSel::B11)
    }
}
#[doc = "Field `CLK_UPHY1_TCPDPHY_REF_DIV_CON` reader - clk_uphy1_tcpdphy_ref divider control register clk=clk_src/(div_con+1)"]
pub type ClkUphy1TcpdphyRefDivConR = crate::FieldReader;
#[doc = "Field `CLK_UPHY1_TCPDPHY_REF_DIV_CON` writer - clk_uphy1_tcpdphy_ref divider control register clk=clk_src/(div_con+1)"]
pub type ClkUphy1TcpdphyRefDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_uphy1_tcpdphy_ref clock select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkUphy1TcpdphyRefClkSel {
    #[doc = "0: clk_32k"]
    B0 = 0,
    #[doc = "1: clk_32k"]
    B1 = 1,
}
impl From<ClkUphy1TcpdphyRefClkSel> for bool {
    #[inline(always)]
    fn from(variant: ClkUphy1TcpdphyRefClkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_UPHY1_TCPDPHY_REF_CLK_SEL` reader - clk_uphy1_tcpdphy_ref clock select control register"]
pub type ClkUphy1TcpdphyRefClkSelR = crate::BitReader<ClkUphy1TcpdphyRefClkSel>;
impl ClkUphy1TcpdphyRefClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkUphy1TcpdphyRefClkSel {
        match self.bits {
            false => ClkUphy1TcpdphyRefClkSel::B0,
            true => ClkUphy1TcpdphyRefClkSel::B1,
        }
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkUphy1TcpdphyRefClkSel::B0
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkUphy1TcpdphyRefClkSel::B1
    }
}
#[doc = "Field `CLK_UPHY1_TCPDPHY_REF_CLK_SEL` writer - clk_uphy1_tcpdphy_ref clock select control register"]
pub type ClkUphy1TcpdphyRefClkSelW<'a, REG> = crate::BitWriter<'a, REG, ClkUphy1TcpdphyRefClkSel>;
impl<'a, REG> ClkUphy1TcpdphyRefClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUphy1TcpdphyRefClkSel::B0)
    }
    #[doc = "clk_32k"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUphy1TcpdphyRefClkSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - clk_uphy1_tcpdcore divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_uphy1_tcpdcore_div_con(&self) -> ClkUphy1TcpdcoreDivConR {
        ClkUphy1TcpdcoreDivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - clk_uphy1_tcpdcore clock select control register"]
    #[inline(always)]
    pub fn clk_uphy1_tcpdcore_clk_sel(&self) -> ClkUphy1TcpdcoreClkSelR {
        ClkUphy1TcpdcoreClkSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - clk_uphy1_tcpdphy_ref divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_uphy1_tcpdphy_ref_div_con(&self) -> ClkUphy1TcpdphyRefDivConR {
        ClkUphy1TcpdphyRefDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - clk_uphy1_tcpdphy_ref clock select control register"]
    #[inline(always)]
    pub fn clk_uphy1_tcpdphy_ref_clk_sel(&self) -> ClkUphy1TcpdphyRefClkSelR {
        ClkUphy1TcpdphyRefClkSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk_uphy1_tcpdcore divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uphy1_tcpdcore_div_con(&mut self) -> ClkUphy1TcpdcoreDivConW<CruClkselCon65Spec> {
        ClkUphy1TcpdcoreDivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - clk_uphy1_tcpdcore clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uphy1_tcpdcore_clk_sel(&mut self) -> ClkUphy1TcpdcoreClkSelW<CruClkselCon65Spec> {
        ClkUphy1TcpdcoreClkSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - clk_uphy1_tcpdphy_ref divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uphy1_tcpdphy_ref_div_con(
        &mut self,
    ) -> ClkUphy1TcpdphyRefDivConW<CruClkselCon65Spec> {
        ClkUphy1TcpdphyRefDivConW::new(self, 8)
    }
    #[doc = "Bit 15 - clk_uphy1_tcpdphy_ref clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uphy1_tcpdphy_ref_clk_sel(
        &mut self,
    ) -> ClkUphy1TcpdphyRefClkSelW<CruClkselCon65Spec> {
        ClkUphy1TcpdphyRefClkSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon65Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con65::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con65::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon65Spec;
impl crate::RegisterSpec for CruClkselCon65Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con65::R`](R) reader structure"]
impl crate::Readable for CruClkselCon65Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con65::W`](W) writer structure"]
impl crate::Writable for CruClkselCon65Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON65 to value 0xc5"]
impl crate::Resettable for CruClkselCon65Spec {
    const RESET_VALUE: u32 = 0xc5;
}
