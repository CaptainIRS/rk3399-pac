#[doc = "Register `SPDIF_REPETTION_SHD` reader"]
pub type R = crate::R<SpdifRepettionShdSpec>;
#[doc = "Field `REPETTION` reader - Repetition\n\nThis register provides the repetition of the bitstream when\n\nchannel conveys non-linear PCM. In the design, it defines the\n\nlength between Pa of the two consecutive data-burst. For the\n\nsame audio format, the definition is different. Please convert the\n\nactual repetition in order to comply with the design."]
pub type RepettionR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Repetition\n\nThis register provides the repetition of the bitstream when\n\nchannel conveys non-linear PCM. In the design, it defines the\n\nlength between Pa of the two consecutive data-burst. For the\n\nsame audio format, the definition is different. Please convert the\n\nactual repetition in order to comply with the design."]
    #[inline(always)]
    pub fn repettion(&self) -> RepettionR {
        RepettionR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Shadow Channel Repetition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdif_repettion_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpdifRepettionShdSpec;
impl crate::RegisterSpec for SpdifRepettionShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdif_repettion_shd::R`](R) reader structure"]
impl crate::Readable for SpdifRepettionShdSpec {}
#[doc = "`reset()` method sets SPDIF_REPETTION_SHD to value 0"]
impl crate::Resettable for SpdifRepettionShdSpec {
    const RESET_VALUE: u32 = 0;
}
