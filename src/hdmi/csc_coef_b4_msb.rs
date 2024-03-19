#[doc = "Register `CSC_COEF_B4_MSB` reader"]
pub type R = crate::R<CscCoefB4MsbSpec>;
#[doc = "Register `CSC_COEF_B4_MSB` writer"]
pub type W = crate::W<CscCoefB4MsbSpec>;
#[doc = "Field `CSC_COEF_B4_MSB` reader - Color Space Converter Matrix B4 Coefficient\n\nRegister MSB"]
pub type CscCoefB4MsbR = crate::FieldReader;
#[doc = "Field `CSC_COEF_B4_MSB` writer - Color Space Converter Matrix B4 Coefficient\n\nRegister MSB"]
pub type CscCoefB4MsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix B4 Coefficient\n\nRegister MSB"]
    #[inline(always)]
    pub fn csc_coef_b4_msb(&self) -> CscCoefB4MsbR {
        CscCoefB4MsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix B4 Coefficient\n\nRegister MSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_coef_b4_msb(&mut self) -> CscCoefB4MsbW<CscCoefB4MsbSpec> {
        CscCoefB4MsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix B4 Coefficient Register MSB Color Space\n\nConversion B4 coefficient.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_coef_b4_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_coef_b4_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscCoefB4MsbSpec;
impl crate::RegisterSpec for CscCoefB4MsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_coef_b4_msb::R`](R) reader structure"]
impl crate::Readable for CscCoefB4MsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_coef_b4_msb::W`](W) writer structure"]
impl crate::Writable for CscCoefB4MsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_COEF_B4_MSB to value 0"]
impl crate::Resettable for CscCoefB4MsbSpec {
    const RESET_VALUE: u8 = 0;
}
