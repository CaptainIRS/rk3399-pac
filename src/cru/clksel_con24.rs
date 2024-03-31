#[doc = "Register `CLKSEL_CON24` reader"]
pub type R = crate::R<ClkselCon24Spec>;
#[doc = "Register `CLKSEL_CON24` writer"]
pub type W = crate::W<ClkselCon24Spec>;
#[doc = "Field `CLK_CRYPTO0_DIV_CON` reader - clk_crypto0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkCrypto0DivConR = crate::FieldReader;
#[doc = "Field `CLK_CRYPTO0_DIV_CON` writer - clk_crypto0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkCrypto0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "clk_crypto0 clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkCrypto0PllSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: PPLL"]
    B10 = 2,
}
impl From<ClkCrypto0PllSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkCrypto0PllSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkCrypto0PllSel {
    type Ux = u8;
}
#[doc = "Field `CLK_CRYPTO0_PLL_SEL` reader - clk_crypto0 clock source select control register"]
pub type ClkCrypto0PllSelR = crate::FieldReader<ClkCrypto0PllSel>;
impl ClkCrypto0PllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkCrypto0PllSel> {
        match self.bits {
            0 => Some(ClkCrypto0PllSel::B00),
            1 => Some(ClkCrypto0PllSel::B01),
            2 => Some(ClkCrypto0PllSel::B10),
            _ => None,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkCrypto0PllSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkCrypto0PllSel::B01
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkCrypto0PllSel::B10
    }
}
#[doc = "Field `CLK_CRYPTO0_PLL_SEL` writer - clk_crypto0 clock source select control register"]
pub type ClkCrypto0PllSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkCrypto0PllSel>;
impl<'a, REG> ClkCrypto0PllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCrypto0PllSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCrypto0PllSel::B01)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkCrypto0PllSel::B10)
    }
}
#[doc = "Field `FCLK_CM0S_DIV_CON` reader - fclk_cm0s divider control register\n\nclk=clk_src/(div_con+1)"]
pub type FclkCm0sDivConR = crate::FieldReader;
#[doc = "Field `FCLK_CM0S_DIV_CON` writer - fclk_cm0s divider control register\n\nclk=clk_src/(div_con+1)"]
pub type FclkCm0sDivConW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "fclk_cm0s clock source select control register\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FclkCm0sPllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<FclkCm0sPllSel> for bool {
    #[inline(always)]
    fn from(variant: FclkCm0sPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCLK_CM0S_PLL_SEL` reader - fclk_cm0s clock source select control register"]
pub type FclkCm0sPllSelR = crate::BitReader<FclkCm0sPllSel>;
impl FclkCm0sPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FclkCm0sPllSel {
        match self.bits {
            false => FclkCm0sPllSel::B0,
            true => FclkCm0sPllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FclkCm0sPllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FclkCm0sPllSel::B1
    }
}
#[doc = "Field `FCLK_CM0S_PLL_SEL` writer - fclk_cm0s clock source select control register"]
pub type FclkCm0sPllSelW<'a, REG> = crate::BitWriter<'a, REG, FclkCm0sPllSel>;
impl<'a, REG> FclkCm0sPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FclkCm0sPllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FclkCm0sPllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - clk_crypto0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_crypto0_div_con(&self) -> ClkCrypto0DivConR {
        ClkCrypto0DivConR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - clk_crypto0 clock source select control register"]
    #[inline(always)]
    pub fn clk_crypto0_pll_sel(&self) -> ClkCrypto0PllSelR {
        ClkCrypto0PllSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - fclk_cm0s divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn fclk_cm0s_div_con(&self) -> FclkCm0sDivConR {
        FclkCm0sDivConR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - fclk_cm0s clock source select control register"]
    #[inline(always)]
    pub fn fclk_cm0s_pll_sel(&self) -> FclkCm0sPllSelR {
        FclkCm0sPllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - clk_crypto0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_crypto0_div_con(&mut self) -> ClkCrypto0DivConW<ClkselCon24Spec> {
        ClkCrypto0DivConW::new(self, 0)
    }
    #[doc = "Bits 6:7 - clk_crypto0 clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_crypto0_pll_sel(&mut self) -> ClkCrypto0PllSelW<ClkselCon24Spec> {
        ClkCrypto0PllSelW::new(self, 6)
    }
    #[doc = "Bits 8:12 - fclk_cm0s divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn fclk_cm0s_div_con(&mut self) -> FclkCm0sDivConW<ClkselCon24Spec> {
        FclkCm0sDivConW::new(self, 8)
    }
    #[doc = "Bit 15 - fclk_cm0s clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn fclk_cm0s_pll_sel(&mut self) -> FclkCm0sPllSelW<ClkselCon24Spec> {
        FclkCm0sPllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon24Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon24Spec;
impl crate::RegisterSpec for ClkselCon24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con24::R`](R) reader structure"]
impl crate::Readable for ClkselCon24Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con24::W`](W) writer structure"]
impl crate::Writable for ClkselCon24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON24 to value 0x8103"]
impl crate::Resettable for ClkselCon24Spec {
    const RESET_VALUE: u32 = 0x8103;
}
