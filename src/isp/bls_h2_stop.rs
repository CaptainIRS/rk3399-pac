#[doc = "Register `BLS_H2_STOP` reader"]
pub type R = crate::R<BlsH2StopSpec>;
#[doc = "Register `BLS_H2_STOP` writer"]
pub type W = crate::W<BlsH2StopSpec>;
#[doc = "Field `BLS_H2_STOP` reader - Black pixel position"]
pub type BlsH2StopR = crate::FieldReader<u16>;
#[doc = "Field `BLS_H2_STOP` writer - Black pixel position"]
pub type BlsH2StopW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Black pixel position"]
    #[inline(always)]
    pub fn bls_h2_stop(&self) -> BlsH2StopR {
        BlsH2StopR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Black pixel position"]
    #[inline(always)]
    #[must_use]
    pub fn bls_h2_stop(&mut self) -> BlsH2StopW<BlsH2StopSpec> {
        BlsH2StopW::new(self, 0)
    }
}
#[doc = "window 2 horizontal stop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_h2_stop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_h2_stop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsH2StopSpec;
impl crate::RegisterSpec for BlsH2StopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_h2_stop::R`](R) reader structure"]
impl crate::Readable for BlsH2StopSpec {}
#[doc = "`write(|w| ..)` method takes [`bls_h2_stop::W`](W) writer structure"]
impl crate::Writable for BlsH2StopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLS_H2_STOP to value 0"]
impl crate::Resettable for BlsH2StopSpec {
    const RESET_VALUE: u32 = 0;
}
