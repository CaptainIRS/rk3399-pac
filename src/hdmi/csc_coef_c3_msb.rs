#[doc = "Register `CSC_COEF_C3_MSB` reader"]
pub type R = crate::R<CscCoefC3MsbSpec>;
#[doc = "Register `CSC_COEF_C3_MSB` writer"]
pub type W = crate::W<CscCoefC3MsbSpec>;
#[doc = "Field `CSC_COEF_C3_MSB` reader - Color Space Converter Matrix C3 Coefficient\n\nRegister MSB"]
pub type CscCoefC3MsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_C3_MSB` writer - Color Space Converter Matrix C3 Coefficient\n\nRegister MSB"]
pub type CscCoefC3MsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix C3 Coefficient\n\nRegister MSB"]
    #[inline(always)]
    pub fn csc_coef_c3_msb(&self) -> CscCoefC3MsbR {
        CscCoefC3MsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix C3 Coefficient\n\nRegister MSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_c3_msb(&mut self) -> CscCoefC3MsbW<CscCoefC3MsbSpec> {
        CscCoefC3MsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix C3 Coefficient Register MSB Color Space\n\nConversion C3 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_c3_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_c3_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefC3MsbSpec;
impl crate::RegisterSpec for CscCoefC3MsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_c3_msb::R`](R) reader structure"]
impl crate::Readable for CscCoefC3MsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_c3_msb::W`](W) writer structure"]
impl crate::Writable for CscCoefC3MsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_C3_MSB to value 0x20"]
impl crate::Resettable for CscCoefC3MsbSpec {
    const RESET_VALUE: u8 = 0x20;
}
