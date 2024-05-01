#[doc = "Register `SWREG56_H264_REFER22_POC` reader"]
pub type R = crate::R<Swreg56H264Refer22PocSpec>;
#[doc = "Register `SWREG56_H264_REFER22_POC` writer"]
pub type W = crate::W<Swreg56H264Refer22PocSpec>;
#[doc = "Field `SW_REFER22_POC` reader - the poc of reference picture index 22\n\nthe poc of reference picture index 22"]
pub type SwRefer22PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER22_POC` writer - the poc of reference picture index 22\n\nthe poc of reference picture index 22"]
pub type SwRefer22PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 22\n\nthe poc of reference picture index 22"]
    #[inline(always)]
    pub fn sw_refer22_poc(&self) -> SwRefer22PocR {
        SwRefer22PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 22\n\nthe poc of reference picture index 22"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer22_poc(&mut self) -> SwRefer22PocW<Swreg56H264Refer22PocSpec> {
        SwRefer22PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg56_h264_refer22_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg56_h264_refer22_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg56H264Refer22PocSpec;
impl crate::RegisterSpec for Swreg56H264Refer22PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg56_h264_refer22_poc::R`](R) reader structure"]
impl crate::Readable for Swreg56H264Refer22PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg56_h264_refer22_poc::W`](W) writer structure"]
impl crate::Writable for Swreg56H264Refer22PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG56_H264_REFER22_POC to value 0"]
impl crate::Resettable for Swreg56H264Refer22PocSpec {
    const RESET_VALUE: u32 = 0;
}
