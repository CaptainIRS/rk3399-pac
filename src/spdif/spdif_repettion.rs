#[doc = "Register `SPDIF_REPETTION` reader"]
pub type R = crate::R<SpdifRepettionSpec>;
#[doc = "Register `SPDIF_REPETTION` writer"]
pub type W = crate::W<SpdifRepettionSpec>;
#[doc = "Field `REPETTION` reader - Repetition This define the repetition period when the channel conveys non- linear PCM"]
pub type RepettionR = crate::FieldReader<u16>;
#[doc = "Field `REPETTION` writer - Repetition This define the repetition period when the channel conveys non- linear PCM"]
pub type RepettionW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Repetition This define the repetition period when the channel conveys non- linear PCM"]
    #[inline(always)]
    pub fn repettion(&self) -> RepettionR {
        RepettionR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Repetition This define the repetition period when the channel conveys non- linear PCM"]
    #[inline(always)]
    #[must_use]
    pub fn repettion(&mut self) -> RepettionW<SpdifRepettionSpec> {
        RepettionW::new(self, 0)
    }
}
#[doc = "Channel Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_repettion::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdif_repettion::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdifRepettionSpec;
impl crate::RegisterSpec for SpdifRepettionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdif_repettion::R`](R) reader structure"]
impl crate::Readable for SpdifRepettionSpec {}
#[doc = "`write(|w| ..)` method takes [`spdif_repettion::W`](W) writer structure"]
impl crate::Writable for SpdifRepettionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDIF_REPETTION to value 0"]
impl crate::Resettable for SpdifRepettionSpec {
    const RESET_VALUE: u32 = 0;
}
