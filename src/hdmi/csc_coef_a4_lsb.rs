#[doc = "Register `CSC_COEF_A4_LSB` reader"]
pub type R = crate::R<CscCoefA4LsbSpec>;
#[doc = "Register `CSC_COEF_A4_LSB` writer"]
pub type W = crate::W<CscCoefA4LsbSpec>;
#[doc = "Field `CSC_COEF_A4_LSB` reader - Color Space Converter Matrix A4 Coefficient\n\nRegister LSB"]
pub type CscCoefA4LsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_A4_LSB` writer - Color Space Converter Matrix A4 Coefficient\n\nRegister LSB"]
pub type CscCoefA4LsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix A4 Coefficient\n\nRegister LSB"]
    #[inline(always)]
    pub fn csc_coef_a4_lsb(&self) -> CscCoefA4LsbR {
        CscCoefA4LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix A4 Coefficient\n\nRegister LSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_a4_lsb(&mut self) -> CscCoefA4LsbW<CscCoefA4LsbSpec> {
        CscCoefA4LsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix A4 Coefficient Register LSB Color Space\n\nConversion A4 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_a4_lsb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_a4_lsb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefA4LsbSpec;
impl crate::RegisterSpec for CscCoefA4LsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_a4_lsb::R`](R) reader structure"]
impl crate::Readable for CscCoefA4LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_a4_lsb::W`](W) writer structure"]
impl crate::Writable for CscCoefA4LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_A4_LSB to value 0"]
impl crate::Resettable for CscCoefA4LsbSpec {
    const RESET_VALUE: u8 = 0;
}
