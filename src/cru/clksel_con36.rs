#[doc = "Register `CLKSEL_CON36` reader"]
pub type R = crate::R<ClkselCon36Spec>;
#[doc = "Register `CLKSEL_CON36` writer"]
pub type W = crate::W<ClkselCon36Spec>;
#[doc = "Field `CLK_UART3_DIV_CON` reader - clk_uart3 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkUart3DivConR = crate::FieldReader;
#[doc = "Field `CLK_UART3_DIV_CON` writer - clk_uart3 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkUart3DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_uart3 clock select control register\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkUart3Sel {
    #[doc = "0: clk_uart3_divout"]
    B00 = 0,
    #[doc = "1: clk_uart3_frac"]
    B01 = 1,
    #[doc = "2: xin_24m"]
    B10 = 2,
}
impl From<ClkUart3Sel> for u8 {
    #[inline(always)]
    fn from(variant: ClkUart3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkUart3Sel {
    type Ux = u8;
}
#[doc = "Field `CLK_UART3_SEL` reader - clk_uart3 clock select control register"]
pub type ClkUart3SelR = crate::FieldReader<ClkUart3Sel>;
impl ClkUart3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkUart3Sel> {
        match self.bits {
            0 => Some(ClkUart3Sel::B00),
            1 => Some(ClkUart3Sel::B01),
            2 => Some(ClkUart3Sel::B10),
            _ => None,
        }
    }
    #[doc = "clk_uart3_divout"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkUart3Sel::B00
    }
    #[doc = "clk_uart3_frac"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkUart3Sel::B01
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkUart3Sel::B10
    }
}
#[doc = "Field `CLK_UART3_SEL` writer - clk_uart3 clock select control register"]
pub type ClkUart3SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkUart3Sel>;
impl<'a, REG> ClkUart3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clk_uart3_divout"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart3Sel::B00)
    }
    #[doc = "clk_uart3_frac"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart3Sel::B01)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart3Sel::B10)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_uart3 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_uart3_div_con(&self) -> ClkUart3DivConR {
        ClkUart3DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - clk_uart3 clock select control register"]
    #[inline(always)]
    pub fn clk_uart3_sel(&self) -> ClkUart3SelR {
        ClkUart3SelR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_uart3 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart3_div_con(&mut self) -> ClkUart3DivConW<ClkselCon36Spec> {
        ClkUart3DivConW::new(self, 0)
    }
    #[doc = "Bits 8:9 - clk_uart3 clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart3_sel(&mut self) -> ClkUart3SelW<ClkselCon36Spec> {
        ClkUart3SelW::new(self, 8)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkselCon36Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon36Spec;
impl crate::RegisterSpec for ClkselCon36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con36::R`](R) reader structure"]
impl crate::Readable for ClkselCon36Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con36::W`](W) writer structure"]
impl crate::Writable for ClkselCon36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON36 to value 0x0200"]
impl crate::Resettable for ClkselCon36Spec {
    const RESET_VALUE: u32 = 0x0200;
}
