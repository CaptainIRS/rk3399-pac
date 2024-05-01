#[doc = "Register `SWREG53_H264_REFER19_POC` reader"]
pub type R = crate::R<Swreg53H264Refer19PocSpec>;
#[doc = "Register `SWREG53_H264_REFER19_POC` writer"]
pub type W = crate::W<Swreg53H264Refer19PocSpec>;
#[doc = "Field `SW_REFER19_POC` reader - the poc of reference picture index 19\n\nthe poc of reference picture index 19"]
pub type SwRefer19PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER19_POC` writer - the poc of reference picture index 19\n\nthe poc of reference picture index 19"]
pub type SwRefer19PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 19\n\nthe poc of reference picture index 19"]
    #[inline(always)]
    pub fn sw_refer19_poc(&self) -> SwRefer19PocR {
        SwRefer19PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 19\n\nthe poc of reference picture index 19"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer19_poc(&mut self) -> SwRefer19PocW<Swreg53H264Refer19PocSpec> {
        SwRefer19PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg53_h264_refer19_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg53_h264_refer19_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg53H264Refer19PocSpec;
impl crate::RegisterSpec for Swreg53H264Refer19PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg53_h264_refer19_poc::R`](R) reader structure"]
impl crate::Readable for Swreg53H264Refer19PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg53_h264_refer19_poc::W`](W) writer structure"]
impl crate::Writable for Swreg53H264Refer19PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG53_H264_REFER19_POC to value 0"]
impl crate::Resettable for Swreg53H264Refer19PocSpec {
    const RESET_VALUE: u32 = 0;
}
