#[doc = "Register `CRU_CLKSEL_CON97` reader"]
pub type R = crate::R<CruClkselCon97Spec>;
#[doc = "Register `CRU_CLKSEL_CON97` writer"]
pub type W = crate::W<CruClkselCon97Spec>;
#[doc = "Field `CLK_I2S1_FRAC_DIV_CON` reader - clk_i2s1_frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
pub type ClkI2s1FracDivConR = crate::FieldReader<u32>;
#[doc = "Field `CLK_I2S1_FRAC_DIV_CON` writer - clk_i2s1_frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
pub type ClkI2s1FracDivConW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - clk_i2s1_frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
    #[inline(always)]
    pub fn clk_i2s1_frac_div_con(&self) -> ClkI2s1FracDivConR {
        ClkI2s1FracDivConR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - clk_i2s1_frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s1_frac_div_con(&mut self) -> ClkI2s1FracDivConW<CruClkselCon97Spec> {
        ClkI2s1FracDivConW::new(self, 0)
    }
}
#[doc = "Internal clock select and divide register81\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con97::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con97::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon97Spec;
impl crate::RegisterSpec for CruClkselCon97Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con97::R`](R) reader structure"]
impl crate::Readable for CruClkselCon97Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con97::W`](W) writer structure"]
impl crate::Writable for CruClkselCon97Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON97 to value 0x0bb8_ea60"]
impl crate::Resettable for CruClkselCon97Spec {
    const RESET_VALUE: u32 = 0x0bb8_ea60;
}
