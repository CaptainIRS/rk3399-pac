#[doc = "Register `CRU_CLKSEL_CON35` reader"]
pub type R = crate::R<CruClkselCon35Spec>;
#[doc = "Register `CRU_CLKSEL_CON35` writer"]
pub type W = crate::W<CruClkselCon35Spec>;
#[doc = "Field `CLK_UART2_DIV_CON` reader - clk_uart2 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkUart2DivConR = crate::FieldReader;
#[doc = "Field `CLK_UART2_DIV_CON` writer - clk_uart2 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkUart2DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_uart2 clock select control register\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkUart2Sel {
    #[doc = "0: clk_uart2_divout"]
    B00 = 0,
    #[doc = "1: clk_uart2_frac"]
    B01 = 1,
    #[doc = "2: xin_24m"]
    B10 = 2,
}
impl From<ClkUart2Sel> for u8 {
    #[inline(always)]
    fn from(variant: ClkUart2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkUart2Sel {
    type Ux = u8;
}
#[doc = "Field `CLK_UART2_SEL` reader - clk_uart2 clock select control register"]
pub type ClkUart2SelR = crate::FieldReader<ClkUart2Sel>;
impl ClkUart2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkUart2Sel> {
        match self.bits {
            0 => Some(ClkUart2Sel::B00),
            1 => Some(ClkUart2Sel::B01),
            2 => Some(ClkUart2Sel::B10),
            _ => None,
        }
    }
    #[doc = "clk_uart2_divout"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkUart2Sel::B00
    }
    #[doc = "clk_uart2_frac"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkUart2Sel::B01
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkUart2Sel::B10
    }
}
#[doc = "Field `CLK_UART2_SEL` writer - clk_uart2 clock select control register"]
pub type ClkUart2SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkUart2Sel>;
impl<'a, REG> ClkUart2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clk_uart2_divout"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart2Sel::B00)
    }
    #[doc = "clk_uart2_frac"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart2Sel::B01)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart2Sel::B10)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_uart2 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_uart2_div_con(&self) -> ClkUart2DivConR {
        ClkUart2DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - clk_uart2 clock select control register"]
    #[inline(always)]
    pub fn clk_uart2_sel(&self) -> ClkUart2SelR {
        ClkUart2SelR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_uart2 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart2_div_con(&mut self) -> ClkUart2DivConW<CruClkselCon35Spec> {
        ClkUart2DivConW::new(self, 0)
    }
    #[doc = "Bits 8:9 - clk_uart2 clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart2_sel(&mut self) -> ClkUart2SelW<CruClkselCon35Spec> {
        ClkUart2SelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon35Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon35Spec;
impl crate::RegisterSpec for CruClkselCon35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con35::R`](R) reader structure"]
impl crate::Readable for CruClkselCon35Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con35::W`](W) writer structure"]
impl crate::Writable for CruClkselCon35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON35 to value 0x0200"]
impl crate::Resettable for CruClkselCon35Spec {
    const RESET_VALUE: u32 = 0x0200;
}
