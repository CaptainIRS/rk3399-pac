#[doc = "Register `CSC_COEF_C4_MSB` reader"]
pub type R = crate::R<CscCoefC4MsbSpec>;
#[doc = "Register `CSC_COEF_C4_MSB` writer"]
pub type W = crate::W<CscCoefC4MsbSpec>;
#[doc = "Field `CSC_COEF_C4_MSB` reader - Description: Color Space Converter Matrix C4 Coefficient Register MSB"]
pub type CscCoefC4MsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_C4_MSB` writer - Description: Color Space Converter Matrix C4 Coefficient Register MSB"]
pub type CscCoefC4MsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Description: Color Space Converter Matrix C4 Coefficient Register MSB"]
    #[inline(always)]
    pub fn csc_coef_c4_msb(&self) -> CscCoefC4MsbR {
        CscCoefC4MsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Description: Color Space Converter Matrix C4 Coefficient Register MSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_c4_msb(&mut self) -> CscCoefC4MsbW<CscCoefC4MsbSpec> {
        CscCoefC4MsbW::new(self, 0)
    }
}
#[doc = "Description: Color Space Converter Matrix C4 Coefficient Register MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_c4_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_c4_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefC4MsbSpec;
impl crate::RegisterSpec for CscCoefC4MsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_c4_msb::R`](R) reader structure"]
impl crate::Readable for CscCoefC4MsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_c4_msb::W`](W) writer structure"]
impl crate::Writable for CscCoefC4MsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_C4_MSB to value 0"]
impl crate::Resettable for CscCoefC4MsbSpec {
    const RESET_VALUE: u8 = 0;
}
