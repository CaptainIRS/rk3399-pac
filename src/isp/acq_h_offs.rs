#[doc = "Register `ACQ_H_OFFS` reader"]
pub type R = crate::R<AcqHOffsSpec>;
#[doc = "Register `ACQ_H_OFFS` writer"]
pub type W = crate::W<AcqHOffsSpec>;
#[doc = "Field `ACQ_H_OFFS` reader - horizontal sample offset in 8-bit samples (yuv: 4\n\nsamples=2pix)\n\n"]
pub type AcqHOffsR = crate::FieldReader<u16>;
#[doc = "Field `ACQ_H_OFFS` writer - horizontal sample offset in 8-bit samples (yuv: 4\n\nsamples=2pix)\n\n"]
pub type AcqHOffsW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - horizontal sample offset in 8-bit samples (yuv: 4\n\nsamples=2pix)\n\n"]
    #[inline(always)]
    pub fn acq_h_offs(&self) -> AcqHOffsR {
        AcqHOffsR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - horizontal sample offset in 8-bit samples (yuv: 4\n\nsamples=2pix)\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn acq_h_offs(&mut self) -> AcqHOffsW<AcqHOffsSpec> {
        AcqHOffsW::new(self, 0)
    }
}
#[doc = "horizontal input offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acq_h_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acq_h_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcqHOffsSpec;
impl crate::RegisterSpec for AcqHOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acq_h_offs::R`](R) reader structure"]
impl crate::Readable for AcqHOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`acq_h_offs::W`](W) writer structure"]
impl crate::Writable for AcqHOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACQ_H_OFFS to value 0"]
impl crate::Resettable for AcqHOffsSpec {
    const RESET_VALUE: u32 = 0;
}
