#[doc = "Register `PMUCRU_CLKFRAC_CON0` reader"]
pub type R = crate::R<PmucruClkfracCon0Spec>;
#[doc = "Register `PMUCRU_CLKFRAC_CON0` writer"]
pub type W = crate::W<PmucruClkfracCon0Spec>;
#[doc = "Field `UART4_FRAC_DIV_CON` reader - uart4_frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
pub type Uart4FracDivConR = crate::FieldReader<u32>;
#[doc = "Field `UART4_FRAC_DIV_CON` writer - uart4_frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
pub type Uart4FracDivConW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - uart4_frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
    #[inline(always)]
    pub fn uart4_frac_div_con(&self) -> Uart4FracDivConR {
        Uart4FracDivConR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - uart4_frac divider control register Fout = Fsrc*numerator/denominator High 16-bit for numerator Low 16-bit for denominator"]
    #[inline(always)]
    #[must_use]
    pub fn uart4_frac_div_con(&mut self) -> Uart4FracDivConW<PmucruClkfracCon0Spec> {
        Uart4FracDivConW::new(self, 0)
    }
}
#[doc = "Internal clock select and divide register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_clkfrac_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_clkfrac_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucruClkfracCon0Spec;
impl crate::RegisterSpec for PmucruClkfracCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucru_clkfrac_con0::R`](R) reader structure"]
impl crate::Readable for PmucruClkfracCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmucru_clkfrac_con0::W`](W) writer structure"]
impl crate::Writable for PmucruClkfracCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUCRU_CLKFRAC_CON0 to value 0x0bb8_ea60"]
impl crate::Resettable for PmucruClkfracCon0Spec {
    const RESET_VALUE: u32 = 0x0bb8_ea60;
}
