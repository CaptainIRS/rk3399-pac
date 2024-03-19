#[doc = "Register `CRU_CLKSEL_CON33` reader"]
pub type R = crate::R<CruClkselCon33Spec>;
#[doc = "Register `CRU_CLKSEL_CON33` writer"]
pub type W = crate::W<CruClkselCon33Spec>;
#[doc = "Field `CLK_UART0_DIV_CON` reader - clk_uart0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkUart0DivConR = crate::FieldReader;
#[doc = "Field `CLK_UART0_DIV_CON` writer - clk_uart0 divider control register\n\nclk=clk_src/(div_con+1)"]
pub type ClkUart0DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "clk_uart0 clock select control register\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkUart0Sel {
    #[doc = "0: clk_uart0_divout"]
    B00 = 0,
    #[doc = "1: clk_uart0_frac"]
    B01 = 1,
    #[doc = "2: xin_24m"]
    B10 = 2,
}
impl From<ClkUart0Sel> for u8 {
    #[inline(always)]
    fn from(variant: ClkUart0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkUart0Sel {
    type Ux = u8;
}
#[doc = "Field `CLK_UART0_SEL` reader - clk_uart0 clock select control register"]
pub type ClkUart0SelR = crate::FieldReader<ClkUart0Sel>;
impl ClkUart0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkUart0Sel> {
        match self.bits {
            0 => Some(ClkUart0Sel::B00),
            1 => Some(ClkUart0Sel::B01),
            2 => Some(ClkUart0Sel::B10),
            _ => None,
        }
    }
    #[doc = "clk_uart0_divout"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkUart0Sel::B00
    }
    #[doc = "clk_uart0_frac"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkUart0Sel::B01
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ClkUart0Sel::B10
    }
}
#[doc = "Field `CLK_UART0_SEL` writer - clk_uart0 clock select control register"]
pub type ClkUart0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkUart0Sel>;
impl<'a, REG> ClkUart0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "clk_uart0_divout"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart0Sel::B00)
    }
    #[doc = "clk_uart0_frac"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart0Sel::B01)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart0Sel::B10)
    }
}
#[doc = "clk_uart0_src clock select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkUart0SrcSel {
    #[doc = "0: CPLL"]
    B00 = 0,
    #[doc = "1: GPLL"]
    B01 = 1,
    #[doc = "2: USB_480M"]
    B1x = 2,
}
impl From<ClkUart0SrcSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkUart0SrcSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkUart0SrcSel {
    type Ux = u8;
}
#[doc = "Field `CLK_UART0_SRC_SEL` reader - clk_uart0_src clock select control register"]
pub type ClkUart0SrcSelR = crate::FieldReader<ClkUart0SrcSel>;
impl ClkUart0SrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkUart0SrcSel> {
        match self.bits {
            0 => Some(ClkUart0SrcSel::B00),
            1 => Some(ClkUart0SrcSel::B01),
            2 => Some(ClkUart0SrcSel::B1x),
            _ => None,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ClkUart0SrcSel::B00
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ClkUart0SrcSel::B01
    }
    #[doc = "USB_480M"]
    #[inline(always)]
    pub fn is_b1x(&self) -> bool {
        *self == ClkUart0SrcSel::B1x
    }
}
#[doc = "Field `CLK_UART0_SRC_SEL` writer - clk_uart0_src clock select control register"]
pub type ClkUart0SrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkUart0SrcSel>;
impl<'a, REG> ClkUart0SrcSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart0SrcSel::B00)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart0SrcSel::B01)
    }
    #[doc = "USB_480M"]
    #[inline(always)]
    pub fn b1x(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUart0SrcSel::B1x)
    }
}
#[doc = "clk_uart clock source select control register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkUartPllSel {
    #[doc = "0: CPLL"]
    B0 = 0,
    #[doc = "1: GPLL"]
    B1 = 1,
}
impl From<ClkUartPllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkUartPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_UART_PLL_SEL` reader - clk_uart clock source select control register"]
pub type ClkUartPllSelR = crate::BitReader<ClkUartPllSel>;
impl ClkUartPllSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkUartPllSel {
        match self.bits {
            false => ClkUartPllSel::B0,
            true => ClkUartPllSel::B1,
        }
    }
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkUartPllSel::B0
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkUartPllSel::B1
    }
}
#[doc = "Field `CLK_UART_PLL_SEL` writer - clk_uart clock source select control register"]
pub type ClkUartPllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkUartPllSel>;
impl<'a, REG> ClkUartPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUartPllSel::B0)
    }
    #[doc = "GPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUartPllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - clk_uart0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn clk_uart0_div_con(&self) -> ClkUart0DivConR {
        ClkUart0DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - clk_uart0 clock select control register"]
    #[inline(always)]
    pub fn clk_uart0_sel(&self) -> ClkUart0SelR {
        ClkUart0SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - clk_uart0_src clock select control register"]
    #[inline(always)]
    pub fn clk_uart0_src_sel(&self) -> ClkUart0SrcSelR {
        ClkUart0SrcSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - clk_uart clock source select control register"]
    #[inline(always)]
    pub fn clk_uart_pll_sel(&self) -> ClkUartPllSelR {
        ClkUartPllSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - clk_uart0 divider control register\n\nclk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart0_div_con(&mut self) -> ClkUart0DivConW<CruClkselCon33Spec> {
        ClkUart0DivConW::new(self, 0)
    }
    #[doc = "Bits 8:9 - clk_uart0 clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart0_sel(&mut self) -> ClkUart0SelW<CruClkselCon33Spec> {
        ClkUart0SelW::new(self, 8)
    }
    #[doc = "Bits 12:13 - clk_uart0_src clock select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart0_src_sel(&mut self) -> ClkUart0SrcSelW<CruClkselCon33Spec> {
        ClkUart0SrcSelW::new(self, 12)
    }
    #[doc = "Bit 15 - clk_uart clock source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart_pll_sel(&mut self) -> ClkUartPllSelW<CruClkselCon33Spec> {
        ClkUartPllSelW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkselCon33Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon33Spec;
impl crate::RegisterSpec for CruClkselCon33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con33::R`](R) reader structure"]
impl crate::Readable for CruClkselCon33Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con33::W`](W) writer structure"]
impl crate::Writable for CruClkselCon33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON33 to value 0x0200"]
impl crate::Resettable for CruClkselCon33Spec {
    const RESET_VALUE: u32 = 0x0200;
}
