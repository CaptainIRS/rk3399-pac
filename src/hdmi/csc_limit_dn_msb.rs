#[doc = "Register `CSC_LIMIT_DN_MSB` reader"]
pub type R = crate::R<CscLimitDnMsbSpec>;
#[doc = "Register `CSC_LIMIT_DN_MSB` writer"]
pub type W = crate::W<CscLimitDnMsbSpec>;
#[doc = "Field `CSC_LIMIT_DN_MSB` reader - Color Space Converter Matrix output Down Limit Register MSB"]
pub type CscLimitDnMsbR = crate::FieldReader;
#[doc = "Field `CSC_LIMIT_DN_MSB` writer - Color Space Converter Matrix output Down Limit Register MSB"]
pub type CscLimitDnMsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix output Down Limit Register MSB"]
    #[inline(always)]
    pub fn csc_limit_dn_msb(&self) -> CscLimitDnMsbR {
        CscLimitDnMsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix output Down Limit Register MSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_limit_dn_msb(&mut self) -> CscLimitDnMsbW<CscLimitDnMsbSpec> {
        CscLimitDnMsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix output Down Limit Register MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_limit_dn_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_limit_dn_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscLimitDnMsbSpec;
impl crate::RegisterSpec for CscLimitDnMsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_limit_dn_msb::R`](R) reader structure"]
impl crate::Readable for CscLimitDnMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_limit_dn_msb::W`](W) writer structure"]
impl crate::Writable for CscLimitDnMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_LIMIT_DN_MSB to value 0"]
impl crate::Resettable for CscLimitDnMsbSpec {
    const RESET_VALUE: u8 = 0;
}
