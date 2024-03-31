#[doc = "Register `CLKSEL_CON96` reader"]
pub type R = crate::R<ClkselCon96Spec>;
#[doc = "Register `CLKSEL_CON96` writer"]
pub type W = crate::W<ClkselCon96Spec>;
#[doc = "Field `CLK_I2S0_FRAC_DIV_CON` reader - clk_i2s0_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
pub type ClkI2s0FracDivConR = crate::FieldReader<u32>;
#[doc = "Field `CLK_I2S0_FRAC_DIV_CON` writer - clk_i2s0_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
pub type ClkI2s0FracDivConW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - clk_i2s0_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
    #[inline(always)]
    pub fn clk_i2s0_frac_div_con(&self) -> ClkI2s0FracDivConR {
        ClkI2s0FracDivConR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clk_i2s0_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s0_frac_div_con(&mut self) -> ClkI2s0FracDivConW<ClkselCon96Spec> {
        ClkI2s0FracDivConW::new(self, 0)
    }
}
#[doc = "Internal clock select and divide register80\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con96::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con96::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon96Spec;
impl crate::RegisterSpec for ClkselCon96Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con96::R`](R) reader structure"]
impl crate::Readable for ClkselCon96Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con96::W`](W) writer structure"]
impl crate::Writable for ClkselCon96Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON96 to value 0x0bb8_ea60"]
impl crate::Resettable for ClkselCon96Spec {
    const RESET_VALUE: u32 = 0x0bb8_ea60;
}
