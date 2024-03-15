#[doc = "Register `CRU_CLKSEL_CON106` reader"]
pub type R = crate::R<CruClkselCon106Spec>;
#[doc = "Register `CRU_CLKSEL_CON106` writer"]
pub type W = crate::W<CruClkselCon106Spec>;
#[doc = "Field `DCLK_VOP0_FRAC_DIV_CON` reader - dclk_vop0_frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
pub type DclkVop0FracDivConR = crate::FieldReader<u32>;
#[doc = "Field `DCLK_VOP0_FRAC_DIV_CON` writer - dclk_vop0_frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
pub type DclkVop0FracDivConW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - dclk_vop0_frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
    #[inline(always)]
    pub fn dclk_vop0_frac_div_con(&self) -> DclkVop0FracDivConR {
        DclkVop0FracDivConR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - dclk_vop0_frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_vop0_frac_div_con(&mut self) -> DclkVop0FracDivConW<CruClkselCon106Spec> {
        DclkVop0FracDivConW::new(self, 0)
    }
}
#[doc = "Internal clock select and divide register90\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clksel_con106::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clksel_con106::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkselCon106Spec;
impl crate::RegisterSpec for CruClkselCon106Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clksel_con106::R`](R) reader structure"]
impl crate::Readable for CruClkselCon106Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clksel_con106::W`](W) writer structure"]
impl crate::Writable for CruClkselCon106Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKSEL_CON106 to value 0x0bb8_ea60"]
impl crate::Resettable for CruClkselCon106Spec {
    const RESET_VALUE: u32 = 0x0bb8_ea60;
}
