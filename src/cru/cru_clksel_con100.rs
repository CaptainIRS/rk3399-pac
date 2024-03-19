#[doc = "Register `CRU_CLKSEL_CON100` reader"]
pub type R = crate::R<CruClkselCon100Spec>;
#[doc = "Register `CRU_CLKSEL_CON100` writer"]
pub type W = crate::W<CruClkselCon100Spec>;
#[doc = "Field `CLK_UART0_FRAC_DIV_CON` reader - uart0_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
pub type ClkUart0FracDivConR = crate::FieldReader<u32>;
#[doc = "Field `CLK_UART0_FRAC_DIV_CON` writer - uart0_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
pub type ClkUart0FracDivConW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - uart0_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
    #[inline(always)]
    pub fn clk_uart0_frac_div_con(&self) -> ClkUart0FracDivConR {
        ClkUart0FracDivConR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - uart0_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart0_frac_div_con(&mut self) -> ClkUart0FracDivConW<CruClkselCon100Spec> {
        ClkUart0FracDivConW::new(self, 0)
    }
}
#[doc = "Internal clock select and divide register84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con100::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con100::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon100Spec;
impl crate::RegisterSpec for CruClkselCon100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con100::R`](R) reader structure"]
impl crate::Readable for CruClkselCon100Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con100::W`](W) writer structure"]
impl crate::Writable for CruClkselCon100Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON100 to value 0x0bb8_ea60"]
impl crate::Resettable for CruClkselCon100Spec {
    const RESET_VALUE: u32 = 0x0bb8_ea60;
}
