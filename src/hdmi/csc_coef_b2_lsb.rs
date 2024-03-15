#[doc = "Register `CSC_COEF_B2_LSB` reader"]
pub type R = crate::R<CscCoefB2LsbSpec>;
#[doc = "Register `CSC_COEF_B2_LSB` writer"]
pub type W = crate::W<CscCoefB2LsbSpec>;
#[doc = "Field `CSC_COEF_B2_LSB` reader - Color Space Converter Matrix B2 Coefficient Register LSB"]
pub type CscCoefB2LsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_B2_LSB` writer - Color Space Converter Matrix B2 Coefficient Register LSB"]
pub type CscCoefB2LsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix B2 Coefficient Register LSB"]
    #[inline(always)]
    pub fn csc_coef_b2_lsb(&self) -> CscCoefB2LsbR {
        CscCoefB2LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix B2 Coefficient Register LSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_b2_lsb(&mut self) -> CscCoefB2LsbW<CscCoefB2LsbSpec> {
        CscCoefB2LsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix B2 Coefficient Register LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_b2_lsb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_b2_lsb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefB2LsbSpec;
impl crate::RegisterSpec for CscCoefB2LsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_b2_lsb::R`](R) reader structure"]
impl crate::Readable for CscCoefB2LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_b2_lsb::W`](W) writer structure"]
impl crate::Writable for CscCoefB2LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_B2_LSB to value 0"]
impl crate::Resettable for CscCoefB2LsbSpec {
    const RESET_VALUE: u8 = 0;
}
