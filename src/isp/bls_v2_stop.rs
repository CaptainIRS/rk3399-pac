#[doc = "Register `BLS_V2_STOP` reader"]
pub type R = crate::R<BlsV2StopSpec>;
#[doc = "Register `BLS_V2_STOP` writer"]
pub type W = crate::W<BlsV2StopSpec>;
#[doc = "Field `BLS_V2_STOP` reader - Black pixel position\n\n"]
pub type BlsV2StopR = crate::FieldReader<u16>;
#[doc = "Field `BLS_V2_STOP` writer - Black pixel position\n\n"]
pub type BlsV2StopW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Black pixel position\n\n"]
    #[inline(always)]
    pub fn bls_v2_stop(&self) -> BlsV2StopR {
        BlsV2StopR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Black pixel position\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn bls_v2_stop(&mut self) -> BlsV2StopW<BlsV2StopSpec> {
        BlsV2StopW::new(self, 0)
    }
}
#[doc = "window 2 vertical stop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_v2_stop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_v2_stop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsV2StopSpec;
impl crate::RegisterSpec for BlsV2StopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_v2_stop::R`](R) reader structure"]
impl crate::Readable for BlsV2StopSpec {}
#[doc = "`write(|w| ..)` method takes [`bls_v2_stop::W`](W) writer structure"]
impl crate::Writable for BlsV2StopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLS_V2_STOP to value 0"]
impl crate::Resettable for BlsV2StopSpec {
    const RESET_VALUE: u32 = 0;
}
