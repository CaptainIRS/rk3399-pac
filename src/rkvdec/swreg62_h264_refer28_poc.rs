#[doc = "Register `SWREG62_H264_REFER28_POC` reader"]
pub type R = crate::R<Swreg62H264Refer28PocSpec>;
#[doc = "Register `SWREG62_H264_REFER28_POC` writer"]
pub type W = crate::W<Swreg62H264Refer28PocSpec>;
#[doc = "Field `SW_REFER28_POC` reader - the poc of reference picture index 28\n\nthe poc of reference picture index 28"]
pub type SwRefer28PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER28_POC` writer - the poc of reference picture index 28\n\nthe poc of reference picture index 28"]
pub type SwRefer28PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 28\n\nthe poc of reference picture index 28"]
    #[inline(always)]
    pub fn sw_refer28_poc(&self) -> SwRefer28PocR {
        SwRefer28PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 28\n\nthe poc of reference picture index 28"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer28_poc(&mut self) -> SwRefer28PocW<Swreg62H264Refer28PocSpec> {
        SwRefer28PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg62_h264_refer28_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg62_h264_refer28_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg62H264Refer28PocSpec;
impl crate::RegisterSpec for Swreg62H264Refer28PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg62_h264_refer28_poc::R`](R) reader structure"]
impl crate::Readable for Swreg62H264Refer28PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg62_h264_refer28_poc::W`](W) writer structure"]
impl crate::Writable for Swreg62H264Refer28PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG62_H264_REFER28_POC to value 0"]
impl crate::Resettable for Swreg62H264Refer28PocSpec {
    const RESET_VALUE: u32 = 0;
}
