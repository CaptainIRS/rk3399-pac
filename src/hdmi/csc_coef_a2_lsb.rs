#[doc = "Register `CSC_COEF_A2_LSB` reader"]
pub type R = crate::R<CscCoefA2LsbSpec>;
#[doc = "Register `CSC_COEF_A2_LSB` writer"]
pub type W = crate::W<CscCoefA2LsbSpec>;
#[doc = "Field `CSC_COEF_A2_LSB` reader - Color Space Converter Matrix A2 Coefficient Register LSB"]
pub type CscCoefA2LsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_A2_LSB` writer - Color Space Converter Matrix A2 Coefficient Register LSB"]
pub type CscCoefA2LsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix A2 Coefficient Register LSB"]
    #[inline(always)]
    pub fn csc_coef_a2_lsb(&self) -> CscCoefA2LsbR {
        CscCoefA2LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix A2 Coefficient Register LSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_a2_lsb(&mut self) -> CscCoefA2LsbW<CscCoefA2LsbSpec> {
        CscCoefA2LsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix A2 Coefficient Register LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a2_lsb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a2_lsb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefA2LsbSpec;
impl crate::RegisterSpec for CscCoefA2LsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_a2_lsb::R`](R) reader structure"]
impl crate::Readable for CscCoefA2LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_a2_lsb::W`](W) writer structure"]
impl crate::Writable for CscCoefA2LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_A2_LSB to value 0"]
impl crate::Resettable for CscCoefA2LsbSpec {
    const RESET_VALUE: u8 = 0;
}
