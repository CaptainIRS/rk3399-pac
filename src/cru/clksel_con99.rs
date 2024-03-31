#[doc = "Register `CLKSEL_CON99` reader"]
pub type R = crate::R<ClkselCon99Spec>;
#[doc = "Register `CLKSEL_CON99` writer"]
pub type W = crate::W<ClkselCon99Spec>;
#[doc = "Field `CLK_SPDIF_8CH_FRAC_DIV_CON` reader - spdif_8ch_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
pub type ClkSpdif8chFracDivConR = crate::FieldReader<u32>;
#[doc = "Field `CLK_SPDIF_8CH_FRAC_DIV_CON` writer - spdif_8ch_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
pub type ClkSpdif8chFracDivConW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - spdif_8ch_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
    #[inline(always)]
    pub fn clk_spdif_8ch_frac_div_con(&self) -> ClkSpdif8chFracDivConR {
        ClkSpdif8chFracDivConR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - spdif_8ch_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spdif_8ch_frac_div_con(&mut self) -> ClkSpdif8chFracDivConW<ClkselCon99Spec> {
        ClkSpdif8chFracDivConW::new(self, 0)
    }
}
#[doc = "Internal clock select and divide register83\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con99::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con99::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon99Spec;
impl crate::RegisterSpec for ClkselCon99Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con99::R`](R) reader structure"]
impl crate::Readable for ClkselCon99Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con99::W`](W) writer structure"]
impl crate::Writable for ClkselCon99Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON99 to value 0x0bb8_ea60"]
impl crate::Resettable for ClkselCon99Spec {
    const RESET_VALUE: u32 = 0x0bb8_ea60;
}
