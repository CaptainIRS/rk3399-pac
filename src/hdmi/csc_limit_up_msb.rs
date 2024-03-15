#[doc = "Register `CSC_LIMIT_UP_MSB` reader"]
pub type R = crate::R<CscLimitUpMsbSpec>;
#[doc = "Register `CSC_LIMIT_UP_MSB` writer"]
pub type W = crate::W<CscLimitUpMsbSpec>;
#[doc = "Field `CSC_LIMIT_UP_MSB` reader - Color Space Converter Matrix Output Upper Limit Register MSB"]
pub type CscLimitUpMsbR = crate::FieldReader;
#[doc = "Field `CSC_LIMIT_UP_MSB` writer - Color Space Converter Matrix Output Upper Limit Register MSB"]
pub type CscLimitUpMsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Space Converter Matrix Output Upper Limit Register MSB"]
    #[inline(always)]
    pub fn csc_limit_up_msb(&self) -> CscLimitUpMsbR {
        CscLimitUpMsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Space Converter Matrix Output Upper Limit Register MSB"]
    #[inline(always)]
    #[must_use]
    pub fn csc_limit_up_msb(&mut self) -> CscLimitUpMsbW<CscLimitUpMsbSpec> {
        CscLimitUpMsbW::new(self, 0)
    }
}
#[doc = "Color Space Converter Matrix Output Upper Limit Register MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csc_limit_up_msb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csc_limit_up_msb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscLimitUpMsbSpec;
impl crate::RegisterSpec for CscLimitUpMsbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csc_limit_up_msb::R`](R) reader structure"]
impl crate::Readable for CscLimitUpMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`csc_limit_up_msb::W`](W) writer structure"]
impl crate::Writable for CscLimitUpMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CSC_LIMIT_UP_MSB to value 0xff"]
impl crate::Resettable for CscLimitUpMsbSpec {
    const RESET_VALUE: u8 = 0xff;
}
