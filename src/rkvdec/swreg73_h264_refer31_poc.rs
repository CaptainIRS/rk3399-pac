#[doc = "Register `SWREG73_H264_REFER31_POC` reader"]
pub type R = crate::R<Swreg73H264Refer31PocSpec>;
#[doc = "Register `SWREG73_H264_REFER31_POC` writer"]
pub type W = crate::W<Swreg73H264Refer31PocSpec>;
#[doc = "Field `SW_REFER31_POC` reader - the poc of reference picture index 31\n\nthe poc of reference picture index 31"]
pub type SwRefer31PocR = crate::FieldReader<u32>;
#[doc = "Field `SW_REFER31_POC` writer - the poc of reference picture index 31\n\nthe poc of reference picture index 31"]
pub type SwRefer31PocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the poc of reference picture index 31\n\nthe poc of reference picture index 31"]
    #[inline(always)]
    pub fn sw_refer31_poc(&self) -> SwRefer31PocR {
        SwRefer31PocR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - the poc of reference picture index 31\n\nthe poc of reference picture index 31"]
    #[inline(always)]
    #[must_use]
    pub fn sw_refer31_poc(&mut self) -> SwRefer31PocW<Swreg73H264Refer31PocSpec> {
        SwRefer31PocW::new(self, 0)
    }
}
#[doc = "the poc of reference picture index 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg73_h264_refer31_poc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg73_h264_refer31_poc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg73H264Refer31PocSpec;
impl crate::RegisterSpec for Swreg73H264Refer31PocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg73_h264_refer31_poc::R`](R) reader structure"]
impl crate::Readable for Swreg73H264Refer31PocSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg73_h264_refer31_poc::W`](W) writer structure"]
impl crate::Writable for Swreg73H264Refer31PocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG73_H264_REFER31_POC to value 0"]
impl crate::Resettable for Swreg73H264Refer31PocSpec {
    const RESET_VALUE: u32 = 0;
}
