#[doc = "Register `CRU_CLKSEL_CON34` reader"]
pub type R = crate::R<CruClkselCon34Spec>;
#[doc = "Register `CRU_CLKSEL_CON34` writer"]
pub type W = crate::W<CruClkselCon34Spec>;
#[doc = "Field `CLK_UART1_DIV_CON` reader - clk_uart1 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkUart1DivConR = crate::FieldReader;
#[doc = "Field `CLK_UART1_DIV_CON` writer - clk_uart1 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkUart1DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_uart1 clock select control register\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkUart1Sel {
    #[doc = "0: clk_uart1_divout"]
    B00 = 0,
    #[doc = "1: clk_uart1_frac"]
    B01 = 1,
    #[doc = "2: xin_24m"]
    B10 = 2,
}
impl From<ClkUart1Sel> for u8 {
    #[inline(always)]
    fn from(variant: ClkUart1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkUart1Sel {
    type Ux = u8;
}
#[doc = "Field `CLK_UART1_SEL` reader - clk_uart1 clock select control register"]
pub type ClkUart1SelR = crate::FieldReader<ClkUart1Sel>;
impl ClkUart1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkUart1Sel> {
        match self.bits {
            0 => Some(ClkUart1Sel::B00),
            1 => Some(ClkUart1Sel::B01),
            2 => Some(ClkUart1Sel::B10),
            _ => None,
        }
    }
    #[doc = "clk_uart1_divout"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkUart1Sel::B00
    }
    #[doc = "clk_uart1_frac"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkUart1Sel::B01
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkUart1Sel::B10
    }
}
#[doc = "Field `CLK_UART1_SEL` writer - clk_uart1 clock select control register"]
pub type ClkUart1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkUart1Sel>;
impl<'a, REG> ClkUart1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clk_uart1_divout"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart1Sel::B00)
    }
    #[doc = "clk_uart1_frac"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart1Sel::B01)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart1Sel::B10)
    }
}
#[doc = "Field `CLK_WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type ClkWriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_uart1 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_uart1_div_con(&self) -> ClkUart1DivConR {
        ClkUart1DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - clk_uart1 clock select control register"]
    #[inline(always)]
    pub fn clk_uart1_sel(&self) -> ClkUart1SelR {
        ClkUart1SelR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_uart1 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart1_div_con(&mut self) -> ClkUart1DivConW<CruClkselCon34Spec> {
        ClkUart1DivConW::new(self, 0)
    }
    #[doc = "Bits 8:9 - clk_uart1 clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart1_sel(&mut self) -> ClkUart1SelW<CruClkselCon34Spec> {
        ClkUart1SelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn clk_write_mask(&mut self) -> ClkWriteMaskW<CruClkselCon34Spec> {
        ClkWriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon34Spec;
impl crate::RegisterSpec for CruClkselCon34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con34::R`](R) reader structure"]
impl crate::Readable for CruClkselCon34Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con34::W`](W) writer structure"]
impl crate::Writable for CruClkselCon34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON34 to value 0x0200"]
impl crate::Resettable for CruClkselCon34Spec {
    const RESET_VALUE: u32 = 0x0200;
}
