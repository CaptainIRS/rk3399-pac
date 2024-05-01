#[doc = "Register `SWREG51_H264_REFER17_POC` reader"]
pub type R = crate::R<Swreg51H264Refer17PocSpec>;
#[doc = "Register `SWREG51_H264_REFER17_POC` writer"]
pub type W = crate::W<Swreg51H264Refer17PocSpec>;
#[doc = "Field `SW_REFER17_POC` reader - the poc of reference picture index 17\n\nthe poc of reference picture index 17"]
pub type SwRefer17PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER17_POC` writer - the poc of reference picture index 17\n\nthe poc of reference picture index 17"]
pub type SwRefer17PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 17\n\nthe poc of reference picture index 17"]
    #[inline(always)]
    pub fn sw_refer17_poc(&self) -> SwRefer17PocR {
        SwRefer17PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 17\n\nthe poc of reference picture index 17"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer17_poc(&mut self) -> SwRefer17PocW<Swreg51H264Refer17PocSpec> {
        SwRefer17PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg51_h264_refer17_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg51_h264_refer17_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg51H264Refer17PocSpec;
impl crate::RegisterSpec for Swreg51H264Refer17PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg51_h264_refer17_poc::R`](R) reader structure"]
impl crate::Readable for Swreg51H264Refer17PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg51_h264_refer17_poc::W`](W) writer structure"]
impl crate::Writable for Swreg51H264Refer17PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG51_H264_REFER17_POC to value 0"]
impl crate::Resettable for Swreg51H264Refer17PocSpec {
    const RESET_VALUE: u32 = 0;
}
