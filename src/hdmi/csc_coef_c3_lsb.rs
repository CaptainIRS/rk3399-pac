#[doc = "Register `CSC_COEF_C3_LSB` reader"]
pub type R = crate::R<CscCoefC3LsbSpec>;
#[doc = "Register `CSC_COEF_C3_LSB` writer"]
pub type W = crate::W<CscCoefC3LsbSpec>;
#[doc = "Field `CSC_COEF_C3_LSB` reader - Color Space Converter Matrix C3 Coefficient\n\nRegister LSB"]
pub type CscCoefC3LsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_C3_LSB` writer - Color Space Converter Matrix C3 Coefficient\n\nRegister LSB"]
pub type CscCoefC3LsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix C3 Coefficient\n\nRegister LSB"]
    #[inline(always)]
    pub fn csc_coef_c3_lsb(&self) -> CscCoefC3LsbR {
        CscCoefC3LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix C3 Coefficient\n\nRegister LSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_c3_lsb(&mut self) -> CscCoefC3LsbW<CscCoefC3LsbSpec> {
        CscCoefC3LsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix C3 Coefficient Register LSB Color Space\n\nConversion C3 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_c3_lsb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_c3_lsb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefC3LsbSpec;
impl crate::RegisterSpec for CscCoefC3LsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_c3_lsb::R`](R) reader structure"]
impl crate::Readable for CscCoefC3LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_c3_lsb::W`](W) writer structure"]
impl crate::Writable for CscCoefC3LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_C3_LSB to value 0"]
impl crate::Resettable for CscCoefC3LsbSpec {
    const RESET_VALUE: u8 = 0;
}
