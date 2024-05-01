#[doc = "Register `SWREG52_H264_REFER18_POC` reader"]
pub type R = crate::R<Swreg52H264Refer18PocSpec>;
#[doc = "Register `SWREG52_H264_REFER18_POC` writer"]
pub type W = crate::W<Swreg52H264Refer18PocSpec>;
#[doc = "Field `SW_REFER18_POC` reader - the poc of reference picture index 18\n\nthe poc of reference picture index 18"]
pub type SwRefer18PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER18_POC` writer - the poc of reference picture index 18\n\nthe poc of reference picture index 18"]
pub type SwRefer18PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 18\n\nthe poc of reference picture index 18"]
    #[inline(always)]
    pub fn sw_refer18_poc(&self) -> SwRefer18PocR {
        SwRefer18PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 18\n\nthe poc of reference picture index 18"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer18_poc(&mut self) -> SwRefer18PocW<Swreg52H264Refer18PocSpec> {
        SwRefer18PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg52_h264_refer18_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg52_h264_refer18_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg52H264Refer18PocSpec;
impl crate::RegisterSpec for Swreg52H264Refer18PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg52_h264_refer18_poc::R`](R) reader structure"]
impl crate::Readable for Swreg52H264Refer18PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg52_h264_refer18_poc::W`](W) writer structure"]
impl crate::Writable for Swreg52H264Refer18PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG52_H264_REFER18_POC to value 0"]
impl crate::Resettable for Swreg52H264Refer18PocSpec {
    const RESET_VALUE: u32 = 0;
}
