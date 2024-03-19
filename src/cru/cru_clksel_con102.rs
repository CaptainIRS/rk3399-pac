#[doc = "Register `CRU_CLKSEL_CON102` reader"]
pub type R = crate::R<CruClkselCon102Spec>;
#[doc = "Register `CRU_CLKSEL_CON102` writer"]
pub type W = crate::W<CruClkselCon102Spec>;
#[doc = "Field `CLK_UART2_FRAC_DIV_CON` reader - uart2_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
pub type ClkUart2FracDivConR = crate::FieldReader<u32>;
#[doc = "Field `CLK_UART2_FRAC_DIV_CON` writer - uart2_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
pub type ClkUart2FracDivConW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - uart2_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
    #[inline(always)]
    pub fn clk_uart2_frac_div_con(&self) -> ClkUart2FracDivConR {
        ClkUart2FracDivConR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - uart2_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uart2_frac_div_con(&mut self) -> ClkUart2FracDivConW<CruClkselCon102Spec> {
        ClkUart2FracDivConW::new(self, 0)
    }
}
#[doc = "Internal clock select and divide register86\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con102::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con102::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon102Spec;
impl crate::RegisterSpec for CruClkselCon102Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con102::R`](R) reader structure"]
impl crate::Readable for CruClkselCon102Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con102::W`](W) writer structure"]
impl crate::Writable for CruClkselCon102Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON102 to value 0x0bb8_ea60"]
impl crate::Resettable for CruClkselCon102Spec {
    const RESET_VALUE: u32 = 0x0bb8_ea60;
}
