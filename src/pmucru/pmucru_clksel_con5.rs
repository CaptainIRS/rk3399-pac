#[doc = "Register `PMUCRU_CLKSEL_CON5` reader"]
pub type R = crate::R<PmucruClkselCon5Spec>;
#[doc = "Register `PMUCRU_CLKSEL_CON5` writer"]
pub type W = crate::W<PmucruClkselCon5Spec>;
#[doc = "Field `UART4_DIV_CON` reader - uart4 divider control register clk=clk_src/(div_con+1)"]
pub type Uart4DivConR = crate::FieldReader;
#[doc = "Field `UART4_DIV_CON` writer - uart4 divider control register clk=clk_src/(div_con+1)"]
pub type Uart4DivConW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "uart4_clk source select control register\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uart4ClkSel {
    #[doc = "0: xin_24m"]
    B00 = 0,
    #[doc = "1: xin_24m"]
    B01 = 1,
    #[doc = "2: xin_24m"]
    B10 = 2,
}
impl From<Uart4ClkSel> for u8 {
    #[inline(always)]
    fn from(variant: Uart4ClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uart4ClkSel {
    type Ux = u8;
}
#[doc = "Field `UART4_CLK_SEL` reader - uart4_clk source select control register"]
pub type Uart4ClkSelR = crate::FieldReader<Uart4ClkSel>;
impl Uart4ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uart4ClkSel> {
        match self.bits {
            0 => Some(Uart4ClkSel::B00),
            1 => Some(Uart4ClkSel::B01),
            2 => Some(Uart4ClkSel::B10),
            _ => None,
        }
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Uart4ClkSel::B00
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Uart4ClkSel::B01
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Uart4ClkSel::B10
    }
}
#[doc = "Field `UART4_CLK_SEL` writer - uart4_clk source select control register"]
pub type Uart4ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Uart4ClkSel>;
impl<'a, REG> Uart4ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4ClkSel::B00)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4ClkSel::B01)
    }
    #[doc = "xin_24m"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Uart4ClkSel::B10)
    }
}
#[doc = "clk_uart_pll source select control register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkUartPllSel {
    #[doc = "0: PPLL"]
    B0 = 0,
    #[doc = "1: PPLL"]
    B1 = 1,
}
impl From<ClkUartPllSel> for bool {
    #[inline(always)]
    fn from(variant: ClkUartPllSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_UART_PLL_SEL` reader - clk_uart_pll source select control register"]
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
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClkUartPllSel::B0
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClkUartPllSel::B1
    }
}
#[doc = "Field `CLK_UART_PLL_SEL` writer - clk_uart_pll source select control register"]
pub type ClkUartPllSelW<'a, REG> = crate::BitWriter<'a, REG, ClkUartPllSel>;
impl<'a, REG> ClkUartPllSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUartPllSel::B0)
    }
    #[doc = "PPLL"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkUartPllSel::B1)
    }
}
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - uart4 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    pub fn uart4_div_con(&self) -> Uart4DivConR {
        Uart4DivConR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - uart4_clk source select control register"]
    #[inline(always)]
    pub fn uart4_clk_sel(&self) -> Uart4ClkSelR {
        Uart4ClkSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - clk_uart_pll source select control register"]
    #[inline(always)]
    pub fn clk_uart_pll_sel(&self) -> ClkUartPllSelR {
        ClkUartPllSelR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - uart4 divider control register clk=clk_src/(div_con+1)"]
    #[inline(always)]
    #[must_use]
    pub fn uart4_div_con(&mut self) -> Uart4DivConW<PmucruClkselCon5Spec> {
        Uart4DivConW::new(self, 0)
    }
    #[doc = "Bits 8:9 - uart4_clk source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn uart4_clk_sel(&mut self) -> Uart4ClkSelW<PmucruClkselCon5Spec> {
        Uart4ClkSelW::new(self, 8)
    }
    #[doc = "Bit 10 - clk_uart_pll source select control register"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart_pll_sel(&mut self) -> ClkUartPllSelW<PmucruClkselCon5Spec> {
        ClkUartPllSelW::new(self, 10)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PmucruClkselCon5Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock select and divide register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clksel_con5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clksel_con5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucruClkselCon5Spec;
impl crate::RegisterSpec for PmucruClkselCon5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucru_clksel_con5::R`](R) reader structure"]
impl crate::Readable for PmucruClkselCon5Spec {}
#[doc = "`write(|w| ..)` method takes [`pmucru_clksel_con5::W`](W) writer structure"]
impl crate::Writable for PmucruClkselCon5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUCRU_CLKSEL_CON5 to value 0x0200"]
impl crate::Resettable for PmucruClkselCon5Spec {
    const RESET_VALUE: u32 = 0x0200;
}
