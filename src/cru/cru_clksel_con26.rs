#[doc = "Register `CRU_CLKSEL_CON26` reader"]
pub type R = crate::R<CruClkselCon26Spec>;
#[doc = "Register `CRU_CLKSEL_CON26` writer"]
pub type W = crate::W<CruClkselCon26Spec>;
#[doc = "Field `CLK_CRYPTO1_DIV_CON` reader - clk_crypto1 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkCrypto1DivConR = crate::FieldReader;
#[doc = "Field `CLK_CRYPTO1_DIV_CON` writer - clk_crypto1 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkCrypto1DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_crypto1 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkCrypto1PllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: PPLL"]
    B10 = 2,
}
impl From<ClkCrypto1PllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkCrypto1PllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkCrypto1PllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_CRYPTO1_PLL_SEL` reader - clk_crypto1 clock source select control register"]
pub type ClkCrypto1PllSelR = crate::FieldReader<ClkCrypto1PllSel>;
impl ClkCrypto1PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkCrypto1PllSel> {
        match self.bits {
            0 => Some(ClkCrypto1PllSel::B00),
            1 => Some(ClkCrypto1PllSel::B01),
            2 => Some(ClkCrypto1PllSel::B10),
            _ => None,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkCrypto1PllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkCrypto1PllSel::B01
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkCrypto1PllSel::B10
    }
}
#[doc = "Field `CLK_CRYPTO1_PLL_SEL` writer - clk_crypto1 clock source select control register"]
pub type ClkCrypto1PllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkCrypto1PllSel>;
impl<'a, REG> ClkCrypto1PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCrypto1PllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCrypto1PllSel::B01)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCrypto1PllSel::B10)
    }
}
#[doc = "Field `CLK_SARADC_DIV_CON` reader - clk_saradc divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkSaradcDivConR = crate::FieldReader;
#[doc = "Field `CLK_SARADC_DIV_CON` writer - clk_saradc divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkSaradcDivConW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - clk_crypto1 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_crypto1_div_con(&self) -> ClkCrypto1DivConR {
        ClkCrypto1DivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - clk_crypto1 clock source select control register"]
    #[inline(always)]
    pub fn clk_crypto1_pll_sel(&self) -> ClkCrypto1PllSelR {
        ClkCrypto1PllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - clk_saradc divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_saradc_div_con(&self) -> ClkSaradcDivConR {
        ClkSaradcDivConR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk_crypto1 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_crypto1_div_con(&mut self) -> ClkCrypto1DivConW<CruClkselCon26Spec> {
        ClkCrypto1DivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - clk_crypto1 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_crypto1_pll_sel(&mut self) -> ClkCrypto1PllSelW<CruClkselCon26Spec> {
        ClkCrypto1PllSelW::new(self, 6)
    }
    #[doc = "Bits 8:15 - clk_saradc divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_saradc_div_con(&mut self) -> ClkSaradcDivConW<CruClkselCon26Spec> {
        ClkSaradcDivConW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon26Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon26Spec;
impl crate::RegisterSpec for CruClkselCon26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con26::R`](R) reader structure"]
impl crate::Readable for CruClkselCon26Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con26::W`](W) writer structure"]
impl crate::Writable for CruClkselCon26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON26 to value 0x0103"]
impl crate::Resettable for CruClkselCon26Spec {
    const RESET_VALUE: u32 = 0x0103;
}
