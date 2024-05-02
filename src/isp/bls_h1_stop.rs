#[doc = "Register `BLS_H1_STOP` reader"]
pub type R = crate::R<BlsH1StopSpec>;
#[doc = "Register `BLS_H1_STOP` writer"]
pub type W = crate::W<BlsH1StopSpec>;
#[doc = "Field `BLS_H1_STOP` reader - Black pixel position"]
pub type BlsH1StopR = crate::FieldReader<u16>;
#[doc = "Field `BLS_H1_STOP` writer - Black pixel position"]
pub type BlsH1StopW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Black pixel position"]
    #[inline(always)]
    pub fn bls_h1_stop(&self) -> BlsH1StopR {
        BlsH1StopR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Black pixel position"]
    #[inline(always)]
    #[must_use]
    pub fn bls_h1_stop(&mut self) -> BlsH1StopW<BlsH1StopSpec> {
        BlsH1StopW::new(self, 0)
    }
}
#[doc = "window 1 horizontal stop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_h1_stop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_h1_stop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsH1StopSpec;
impl crate::RegisterSpec for BlsH1StopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_h1_stop::R`](R) reader structure"]
impl crate::Readable for BlsH1StopSpec {}
#[doc = "`write(|w| ..)` method takes [`bls_h1_stop::W`](W) writer structure"]
impl crate::Writable for BlsH1StopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLS_H1_STOP to value 0"]
impl crate::Resettable for BlsH1StopSpec {
    const RESET_VALUE: u32 = 0;
}
