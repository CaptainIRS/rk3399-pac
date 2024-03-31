#[doc = "Register `CLKSEL_CON107` reader"]
pub type R = crate::R<ClkselCon107Spec>;
#[doc = "Register `CLKSEL_CON107` writer"]
pub type W = crate::W<ClkselCon107Spec>;
#[doc = "Field `DCLK_VOP1_FRAC_DIV_CON` reader - dclk_vop1_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
pub type DclkVop1FracDivConR = crate::FieldReader<u32>;
#[doc = "Field `DCLK_VOP1_FRAC_DIV_CON` writer - dclk_vop1_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
pub type DclkVop1FracDivConW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - dclk_vop1_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
    #[inline(always)]
    pub fn dclk_vop1_frac_div_con(&self) -> DclkVop1FracDivConR {
        DclkVop1FracDivConR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - dclk_vop1_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_vop1_frac_div_con(&mut self) -> DclkVop1FracDivConW<ClkselCon107Spec> {
        DclkVop1FracDivConW::new(self, 0)
    }
}
#[doc = "Internal clock select and divide register91\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel_con107::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel_con107::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselCon107Spec;
impl crate::RegisterSpec for ClkselCon107Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel_con107::R`](R) reader structure"]
impl crate::Readable for ClkselCon107Spec {}
#[doc = "`write(|w| ..)` method takes [`clksel_con107::W`](W) writer structure"]
impl crate::Writable for ClkselCon107Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL_CON107 to value 0x0bb8_ea60"]
impl crate::Resettable for ClkselCon107Spec {
    const RESET_VALUE: u32 = 0x0bb8_ea60;
}
