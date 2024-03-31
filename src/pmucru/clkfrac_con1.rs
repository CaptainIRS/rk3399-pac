#[doc = "Register `CLKFRAC_CON1` reader"]
pub type R = crate::R<ClkfracCon1Spec>;
#[doc = "Register `CLKFRAC_CON1` writer"]
pub type W = crate::W<ClkfracCon1Spec>;
#[doc = "Field `WIFI_FRAC_DIV_CON` reader - wifi_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
pub type WifiFracDivConR = crate::FieldReader<u32>;
#[doc = "Field `WIFI_FRAC_DIV_CON` writer - wifi_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
pub type WifiFracDivConW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - wifi_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
    #[inline(always)]
    pub fn wifi_frac_div_con(&self) -> WifiFracDivConR {
        WifiFracDivConR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - wifi_frac divider control register\n\nFout = Fsrc*numerator/denominator\n\nHigh 16-bit for numerator\n\nLow 16-bit for denominator"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_frac_div_con(&mut self) -> WifiFracDivConW<ClkfracCon1Spec> {
        WifiFracDivConW::new(self, 0)
    }
}
#[doc = "Internal clock select and divide register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkfrac_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkfrac_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkfracCon1Spec;
impl crate::RegisterSpec for ClkfracCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkfrac_con1::R`](R) reader structure"]
impl crate::Readable for ClkfracCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`clkfrac_con1::W`](W) writer structure"]
impl crate::Writable for ClkfracCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKFRAC_CON1 to value 0x0bb8_ea60"]
impl crate::Resettable for ClkfracCon1Spec {
    const RESET_VALUE: u32 = 0x0bb8_ea60;
}
