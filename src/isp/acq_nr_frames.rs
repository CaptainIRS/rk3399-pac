#[doc = "Register `ACQ_NR_FRAMES` reader"]
pub type R = crate::R<AcqNrFramesSpec>;
#[doc = "Register `ACQ_NR_FRAMES` writer"]
pub type W = crate::W<AcqNrFramesSpec>;
#[doc = "Field `ACQ_NR_FRAMES` reader - number of input frames to be sampled ( 0 =\n\ncontinuous )\n\n"]
pub type AcqNrFramesR = crate::FieldReader<u16>;
#[doc = "Field `ACQ_NR_FRAMES` writer - number of input frames to be sampled ( 0 =\n\ncontinuous )\n\n"]
pub type AcqNrFramesW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - number of input frames to be sampled ( 0 =\n\ncontinuous )\n\n"]
    #[inline(always)]
    pub fn acq_nr_frames(&self) -> AcqNrFramesR {
        AcqNrFramesR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - number of input frames to be sampled ( 0 =\n\ncontinuous )\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn acq_nr_frames(&mut self) -> AcqNrFramesW<AcqNrFramesSpec> {
        AcqNrFramesW::new(self, 0)
    }
}
#[doc = "Number of frames to be captured\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acq_nr_frames::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acq_nr_frames::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcqNrFramesSpec;
impl crate::RegisterSpec for AcqNrFramesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acq_nr_frames::R`](R) reader structure"]
impl crate::Readable for AcqNrFramesSpec {}
#[doc = "`write(|w| ..)` method takes [`acq_nr_frames::W`](W) writer structure"]
impl crate::Writable for AcqNrFramesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACQ_NR_FRAMES to value 0"]
impl crate::Resettable for AcqNrFramesSpec {
    const RESET_VALUE: u32 = 0;
}
